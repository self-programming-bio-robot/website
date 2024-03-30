use std::error::Error;
use std::sync::Arc;
use langchain_rust::embedding::embedder_trait::Embedder;
use tokio::sync::RwLock;
use crate::llm::in_memory_vector_store::store::Store;

pub struct StoreBuilder {
    vector_dimensions: i32,
    embedder: Option<Arc<dyn Embedder>>,
}

impl StoreBuilder {
    pub fn new() -> Self {
        StoreBuilder {
            vector_dimensions: 0,
            embedder: None,
        }
    }

    pub fn vector_dimensions(mut self, vector_dimensions: i32) -> Self {
        self.vector_dimensions = vector_dimensions;
        self
    }

    pub fn embedder<E: Embedder + 'static>(mut self, embedder: E) -> Self {
        self.embedder = Some(Arc::new(embedder));
        self
    }

    // Finalize the builder and construct the Store object
    pub async fn build(self) -> Result<Store, Box<dyn Error>> {
        if self.embedder.is_none() {
            return Err("Embedder is required".into());
        }

        Ok(Store {
            embedder: self.embedder.unwrap(),
            items: RwLock::new(Vec::new()),
        })
    }
}