use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct UserQuestion {
    pub question: String,
    pub from_page: String,
}
