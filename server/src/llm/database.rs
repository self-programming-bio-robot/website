use std::error::Error;
use std::sync::Arc;
use langchain_rust::vectorstore::VectorStore;
use crate::llm::router::router::{Route};
use crate::llm::router::semantic_router::SemanticRouter;

fn load_default_route() -> Route {
    let prompt = include_str!("prompts/base/prompt.txt");
    let route = Route {
        topic: "base".to_string(),
        prompt: prompt.to_string(),
    };
    route
}

macro_rules! add_topic {
    ($router:ident, $topic:expr) => {
        $router.add_topic(
            $topic.to_string(),
            include_str!(concat!("prompts/", $topic, "/prompt.txt")).to_string(),
            include_str!(concat!("prompts/", $topic, "/queries.txt")).split("\n")
                .map(|x| x.to_string()).collect::<Vec<String>>()
        ).await?;
    };
}

macro_rules! add_topics {
    ($router:ident, $($topic:expr),*) => {
        $(
            add_topic!($router, $topic);
        )*
    };
}

pub async fn create_router(vector_store: Arc<dyn VectorStore>) -> Result<SemanticRouter, Box<dyn Error>> {
    let mut router = SemanticRouter::new(
        vector_store.clone(),
        load_default_route(),
    )
        .with_score_threshold(0.5)
        .with_k(10);

    add_topics!(router, "biography", "resume", "hobby", "projects", "base");
    
    Ok(router)
}