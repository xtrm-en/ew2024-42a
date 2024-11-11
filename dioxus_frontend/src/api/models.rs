use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum BlogPostType {
    Intra42,
    StaffMsg,
    CampusEvent
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlogPostItem {
    pub title: String,
    pub content: String,
    pub r#type: BlogPostType,
}