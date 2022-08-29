use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContentType {
    LinkContent,
    ArticleContent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReviewStatus {
    NeverSubmitted,
    Pending,
    Approved,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Thumbnail {
    pub name: String,
    pub mime: String,
    pub url: String,
    // Unused fiels are omitted
}
