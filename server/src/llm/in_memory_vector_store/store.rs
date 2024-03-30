use std::collections::HashMap;
use std::error::Error;
use std::sync::{Arc};

use axum::async_trait;
use langchain_rust::embedding::embedder_trait::Embedder;
use langchain_rust::schemas::Document;
use langchain_rust::vectorstore::{VecStoreOptions, VectorStore};
use serde_json::Value;
use tokio::sync::RwLock;
use crate::llm::utils::similarity::cosine_similarity;

#[derive(Debug)]
pub struct Item {
    pub id: String,
    pub embed: Vec<f64>,
    pub metadata: HashMap<String, Value>,
}

pub struct Store {
    pub(crate) items: RwLock<Vec<Item>>,
    pub(crate) embedder: Arc<dyn Embedder>,
}

#[async_trait]
impl VectorStore for Store {
    async fn add_documents(&self, docs: &[Document], opt: &VecStoreOptions) -> Result<Vec<String>, Box<dyn Error>> {
        let texts: Vec<String> = docs.iter().map(|d| d.page_content.clone()).collect();

        let embedder = opt.embedder.as_ref().unwrap_or(&self.embedder);

        let vectors = embedder.embed_documents(&texts).await?;
        if vectors.len() != docs.len() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Number of vectors and documents do not match",
            )));
        }

        let mut ids = Vec::with_capacity(docs.len());

        for (doc, vector) in docs.iter().zip(vectors.iter()) {
            let id = uuid::Uuid::new_v4().to_string();
            let mut metadata = doc.metadata.clone();
            metadata.insert("content".to_string(), Value::String(doc.page_content.clone()));
            let item = Item {
                id: id.clone(),
                embed: vector.clone(),
                metadata: metadata.clone(),
            };
            self.items.write().await.push(item);
            ids.push(id);
        }

        Ok(ids)
    }

    async fn similarity_search(&self, query: &str, limit: usize, opt: &VecStoreOptions) -> Result<Vec<Document>, Box<dyn Error>> {
        let embedder = opt.embedder.as_ref().unwrap_or(&self.embedder);

        let query_vector = embedder.embed_query(query).await?;

        let items = self.items.read().await;

        let mut scores = Vec::with_capacity(items.len());
        for item in items.iter() {
            let score = cosine_similarity(&query_vector, &item.embed);
            scores.push((item, score));
        }

        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        let mut results = Vec::with_capacity(limit);
        for (item, score) in scores.iter().take(limit) {
            let mut metadata = item.metadata.clone();
            metadata.insert("id".to_string(), Value::String(item.id.clone()));
            let doc = Document {
                page_content: metadata.get("content")
                    .map(|it| it.as_str().unwrap_or(""))
                    .unwrap_or("").to_string(),
                metadata,
                score: *score,
            };
            results.push(doc);
        }

        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use langchain_rust::schemas::Document;
    use langchain_rust::vectorstore::VecStoreOptions;
    use std::collections::HashMap;
    use std::sync::Arc;
    use tokio_test::block_on;
    use mockall::mock;

    mock! {
        pub Embedder {}
        #[async_trait]
        impl Embedder for Embedder {
            async fn embed_documents(&self, texts: &[String]) -> Result<Vec<Vec<f64>>, Box<dyn Error>>;
            async fn embed_query(&self, query: &str) -> Result<Vec<f64>, Box<dyn Error>>;
        }
    }

    #[test]
    fn add_documents_adds_documents_to_store() {
        let mut mock_embedder = MockEmbedder::new();
        mock_embedder.expect_embed_documents()
            .returning(|_| Ok(vec![vec![1.0, 2.0, 3.0]]));

        let store = Store {
            items: RwLock::new(Vec::new()),
            embedder: Arc::new(mock_embedder),
        };

        let docs = vec![Document {
            page_content: "Test".to_string(),
            metadata: HashMap::new(),
            score: 0.0,
        }];

        let result = block_on(store.add_documents(&docs, &VecStoreOptions::default()));

        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 1);
        assert_eq!(store.items.blocking_read().len(), 1);
    }

    #[test]
    fn similarity_search_returns_similar_documents() {
        let mut mock_embedder = MockEmbedder::new();
        mock_embedder.expect_embed_query()
            .returning(|text| Ok(vec![1.0, 2.0, 3.0]));

        let store = Store {
            items: RwLock::new(vec![
                Item {
                    id: "1".to_string(),
                    embed: vec![1.0, 2.0, 3.0],
                    metadata: HashMap::new(),

                },
                Item {
                    id: "2".to_string(),
                    embed: vec![1.0, 5.2, 3.0],
                    metadata: HashMap::new(),

                },
                Item {
                    id: "3".to_string(),
                    embed: vec![1.2, 2.0, 3.0],
                    metadata: HashMap::new(),

                }, 
            ]),
            embedder: Arc::new(mock_embedder),
        };

        let result = block_on(store.similarity_search("Test", 3, &VecStoreOptions::default()));

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].metadata.get("id").unwrap().as_str(), Some("1"));
        assert_eq!(result[1].metadata.get("id").unwrap().as_str(), Some("3"));
        assert_eq!(result[2].metadata.get("id").unwrap().as_str(), Some("2"));
    }
}