use std::error::Error;
use std::pin::Pin;
use std::sync::Arc;

use futures::{Stream, StreamExt, TryStreamExt};
use langchain_rust::{fmt_message, fmt_template, message_formatter, prompt_args, template_fstring};
use langchain_rust::chain::{Chain, LLMChainBuilder};
use langchain_rust::chain::options::ChainCallOptions;
use langchain_rust::embedding::openai::openai_embedder::OpenAiEmbedder;
use langchain_rust::llm::openai::{OpenAI, OpenAIConfig};
use langchain_rust::llm::openai::OpenAIModel::Gpt35;
use langchain_rust::prompt::{HumanMessagePromptTemplate, MessageOrTemplate};
use langchain_rust::schemas::{Message};
use log::debug;
use tracing::log::{info};
use zhdanov_website_core::dto::action::Action;
use zhdanov_website_core::dto::question::UserQuestion;
use crate::llm::database::create_router;
use crate::llm::in_memory_vector_store::builder::StoreBuilder;
use crate::llm::question_assistant::QuestionAssistant;
use crate::llm::router::router::Router;
use crate::llm::utils::message::make_history;

#[derive(Clone)]
pub struct SimpleAgent {
    open_ai: OpenAI<OpenAIConfig>,
    router: Arc<dyn Router>,
    question_assistant: QuestionAssistant,
}

pub enum ResponseData {
    Stream(Pin<Box<dyn Stream<Item=Box<String>> + Send>>),
    Action(Action),
}

pub struct AgentResponse {
    pub topic: String,
    pub data: ResponseData,
    pub is_question: bool,
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
        let question_assistant = QuestionAssistant::new_with_open_ai(open_ai.clone());
        
        Self { open_ai, router: Arc::new(router), question_assistant }
    }

    pub async fn invoke(&self, query: UserQuestion) -> Result<AgentResponse, Box<dyn Error>> {
        let base_prompt = include_str!("prompts/system.txt");
        let route = self.router.route(query.question.clone()).await?;
        let route = if route.is_none() {
            self.router.route(query.question.clone()+"?").await?
        } else {
            route
        };
        
        debug!("Query: {:?}", query);
        
        let route = if let Some(message) = query.messages.last() {
            let route = if let Some(route) = route {
                route
            } else {
                if let Some(last_topic) = &message.topic {
                    if let Some(last_route) = self.router.get_route(last_topic.as_str()) {
                        last_route
                    } else {
                        self.router.default_route()
                    }
                } else {
                    self.router.default_route()
                }
            };
            
            if message.is_question {
                let action = self.question_assistant.invoke(&query.question, &query.messages).await?;
                if let Some(action) = action {
                    return Ok(AgentResponse {
                        topic: route.topic,
                        data: ResponseData::Action(action),
                        is_question: false
                    });
                }
            }
            route
        } else {
            route.unwrap_or(self.router.default_route())
        };
        info!("User question: '{:?}' => Route: '{}'", &query.question, route.topic);
        
        let mut system_prompt = base_prompt.to_owned() + route.prompt.as_str();
        
        let is_question: bool = if route.topic == "resume" && !query.from_page.contains("resume") {
            system_prompt.push_str(
                "\n In the end of you question suggest politely user to open page with CV. User should answer: Yes or No."
            );
            true
        } else { false };
        
        let prompt = message_formatter![
            fmt_message!(Message::new_system_message(system_prompt)),
            MessageOrTemplate::MessagesPlaceholder("chat_history".to_string()),
            fmt_template!(HumanMessagePromptTemplate::new(template_fstring!(
                "{input}", "input"
            )))
        ];
        let messages = make_history(&query.messages);
        let chain = LLMChainBuilder::new()
            .options(ChainCallOptions::new().with_temperature(0.1))
            .prompt(prompt)
            .llm(self.open_ai.clone())
            .build()?;

        let result = chain.stream(prompt_args! {
            "input" => query.question,
            "chat_history" => messages,
        }).await?;

        let stream = Box::pin(result).into_stream().filter_map(|data| async move {
            match data {
                Ok(message) => Some(Box::new(message.content)),
                Err(_) => None,
            }
        });
        let stream: Pin<Box<dyn Stream<Item=Box<String>> + Send>> = Box::pin(stream);
        Ok(AgentResponse {
            topic: route.topic,
            data: ResponseData::Stream(stream),
            is_question
        })
    }
}