use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Question {
    pub id: QuestionId,
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewQuestion {
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct QuestionId(pub i32);
