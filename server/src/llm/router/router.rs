use std::error::Error;
use async_trait::async_trait;

#[derive(Clone, Debug)]
pub struct Route {
    pub topic: String,
    pub prompt: String
}

#[async_trait]
pub trait Router: Send + Sync {
    async fn route(&self, request: String) -> Result<Route, Box<dyn Error>>;
}