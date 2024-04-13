use std::error::Error;
use async_trait::async_trait;

#[derive(Clone, Debug)]
pub struct Route {
    pub topic: String,
    pub prompt: String
}

#[async_trait]
pub trait Router: Send + Sync {
    async fn route(&self, request: String) -> Result<Option<Route>, Box<dyn Error>>;
    fn default_route(&self) -> Route;
    fn get_route(&self, topic: &str) -> Option<Route>;
}