use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;

use axum::async_trait;
use itertools::Itertools;
use langchain_rust::{add_documents, similarity_search};
use langchain_rust::schemas::Document;
use langchain_rust::vectorstore::{VecStoreOptions, VectorStore};
use serde_json::Value;

use super::router::{Route, Router};

pub struct SemanticRouter {
    vector_store: Arc<dyn VectorStore>,
    topics: HashMap<String, (String, usize)>,
    default: Route,
    k: usize,
    score_threshold: f32,
    total_score_threshold: f64,
}

impl SemanticRouter {
    pub fn new(
        vector_store: Arc<dyn VectorStore>,
        default: Route,
    ) -> Self {
        let topics = HashMap::new();
        Self { vector_store, topics, k: 10, score_threshold: 0.5, total_score_threshold: 0.07, default }
    }

    pub fn with_k(mut self, k: usize) -> Self {
        self.k = k;
        self
    }

    pub fn with_score_threshold(mut self, score_threshold: f32) -> Self {
        self.score_threshold = score_threshold;
        self
    }

    pub async fn add_topic(&mut self, topic: String, prompt: String, examples: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        let documents: Vec<Document> = examples.iter()
            .map(|doc| {
                let mut document = Document::new(doc.trim().to_string());
                document.metadata.insert("topic".to_string(), Value::String(topic.clone()));
                document
            })
            .collect();
        _ = add_documents!(self.vector_store, &documents).await?;
        self.topics.insert(topic, (prompt, documents.len()));
        Ok(())
    }
}

#[async_trait]
impl Router for SemanticRouter {
    async fn route(&self, request: String) -> Result<Option<Route>, Box<dyn Error>> {
        let options = VecStoreOptions::default();
        let options = options.with_score_threshold(self.score_threshold);

        let suitable_documents = similarity_search!(self.vector_store, request.as_str(), self.k, &options)
            .await?;
        let topic: Option<(String, String, f64)> = suitable_documents.into_iter()
            .map(|doc| {
                let topic = doc.metadata.get("topic").map(|x| x.as_str().unwrap()).unwrap();
                (doc.score, topic.to_string())
            })
            .group_by(|x| x.1.clone())
            .into_iter()
            .map(|(t, scores)| {
                let scores: Vec<f64> = scores.map(|x| x.0).collect();
                let default = &(t.clone(), scores.len());
                let topic = self.topics.get(&t)
                    .unwrap_or(default);
                let score = scores.iter().sum::<f64>() / topic.1 as f64;
                (t, topic.0.clone(), score)
            })
            .sorted_by(|a, b| b.2.partial_cmp(&a.2).unwrap())
            .next();
        Ok(topic
            .filter(|r| r.2 > self.total_score_threshold)
            .map(|t| Route { topic: t.0, prompt: t.1 })
        )
    }
    fn default_route(&self) -> Route {
        self.default.clone()
    }

    fn get_route(&self, topic: &str) -> Option<Route> {
        self.topics.get(topic).map(|(prompt, _)| Route { topic: topic.to_string(), prompt: prompt.clone() })
    }
}