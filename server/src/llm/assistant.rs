use std::error::Error;
use std::pin::Pin;
use std::sync::Arc;

use futures::{Stream, StreamExt, TryStreamExt};
use langchain_rust::{fmt_message, fmt_template, message_formatter, prompt_args, template_fstring};
use langchain_rust::chain::{Chain, LLMChainBuilder};
use langchain_rust::embedding::openai::openai_embedder::OpenAiEmbedder;
use langchain_rust::llm::openai::{OpenAI, OpenAIConfig};
use langchain_rust::llm::openai::OpenAIModel::Gpt35;
use langchain_rust::prompt::HumanMessagePromptTemplate;
use langchain_rust::schemas::{Message};
use tracing::log::{info};

use crate::llm::database::create_router;
use crate::llm::in_memory_vector_store::builder::StoreBuilder;
use crate::llm::router::router::Router;

#[derive(Clone)]
pub struct SimpleAgent {
    open_ai: OpenAI<OpenAIConfig>,
    router: Arc<dyn Router>,
}

impl SimpleAgent {
    pub async fn new(open_ai_key: String) -> Self {
        let config = OpenAIConfig::new()
            .with_api_key(open_ai_key);
        let open_ai = OpenAI::new(config.clone())
            .with_model(Gpt35);
        let embedder = OpenAiEmbedder::new(config);
        let vector_store = Arc::new(
            StoreBuilder::new()
                .vector_dimensions(1536)
                .embedder(embedder)
                .build()
                .await
                .unwrap()
        );
        let router = create_router(vector_store.clone()).await.unwrap();
        
        Self { open_ai, router: Arc::new(router) }
    }

    pub async fn invoke(&self, query: String) -> Result<Pin<Box<dyn Stream<Item=Box<String>> + Send>>, Box<dyn Error>> {
        let base_prompt = include_str!("prompts/system.txt");
        let route = self.router.route(query.clone()).await?;
        info!("Query: '{}' => Route: '{}'", query, route.topic);
        
        let system_prompt = base_prompt.to_owned() + route.prompt.as_str();
        let prompt = message_formatter![
            fmt_message!(Message::new_system_message(system_prompt)),
            fmt_template!(HumanMessagePromptTemplate::new(template_fstring!(
                "{input}", "input"
            )))
        ];
        
        let chain = LLMChainBuilder::new()
            .prompt(prompt)
            .llm(self.open_ai.clone())
            .build()?;

        let result = chain.stream(prompt_args! {
            "input" => query.clone(),
        }).await?;

        let map = Box::pin(result).into_stream().filter_map(|data| async move {
            match data {
                Ok(message) => Some(Box::new(message.content)),
                Err(_) => None,
            }
        });
        
        Ok(Box::pin(map))
    }
}