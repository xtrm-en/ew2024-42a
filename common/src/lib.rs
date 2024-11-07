use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct News {
    pub title: String,
    pub content: String,
    pub r#type: NewsType,
}

#[derive(Debug, Clone, Deserialize, Serialize, strum::EnumString, strum::Display)]
pub enum NewsType {
    Intra42,
    StaffMsg,
    CampusEvent,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FormContent {
    pub login: String,
    pub email: String,
    pub message: Option<String>,
    pub datetime: String,
}
