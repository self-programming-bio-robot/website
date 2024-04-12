use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Message {
    pub content: String,
    pub is_assistant: bool,
    pub is_response: bool,
    pub is_question: bool,
    pub topic: Option<String>,
}