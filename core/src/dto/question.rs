use serde::{Deserialize, Serialize};
use crate::dto::message::Message;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct UserQuestion {
    pub question: String,
    pub messages: Vec<Message>,
    pub from_page: String,
}
