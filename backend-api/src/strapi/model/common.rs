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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Building {
    #[serde(rename(deserialize = "UNI", serialize = "STG_UNI"))]
    StgUNI,
    #[serde(rename(deserialize = "1A", serialize = "STG_1A"))]
    Stg1A,
    #[serde(rename(deserialize = "1E", serialize = "BLDG_1E"))]
    Bldg1E,
    #[serde(rename(deserialize = "1B", serialize = "BLDG_1B"))]
    Bldg1B,
    #[serde(rename(deserialize = "1C", serialize = "BLDG_1C"))]
    Bldg1C,
    #[serde(rename(deserialize = "2A", serialize = "BLDG_2A"))]
    Bldg2A,
    #[serde(rename(deserialize = "2B", serialize = "BLDG_2B"))]
    Bldg2B,
    #[serde(rename(deserialize = "2C", serialize = "BLDG_2C"))]
    Bldg2C,
    #[serde(rename(deserialize = "2D", serialize = "BLDG_2D"))]
    Bldg2D,
    #[serde(rename(deserialize = "3A", serialize = "BLDG_3A"))]
    Bldg3A,
    #[serde(rename(deserialize = "3B", serialize = "BLDG_3B"))]
    Bldg3B,
    #[serde(rename(deserialize = "5C", serialize = "BLDG_5C"))]
    Bldg5C,
}
