use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContentType {
    LinkContent,
    ArticleContent,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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
    #[serde(alias = "STG_UNI")]
    StgUNI,
    #[serde(rename(deserialize = "1A", serialize = "STG_1A"))]
    #[serde(alias = "STG_1A")]
    Stg1A,
    #[serde(rename(deserialize = "1E", serialize = "BLDG_1E"))]
    #[serde(alias = "BLDG_1E")]
    Bldg1E,
    #[serde(rename(deserialize = "1B", serialize = "BLDG_1B"))]
    #[serde(alias = "BLDG_1B")]
    Bldg1B,
    #[serde(rename(deserialize = "1C", serialize = "BLDG_1C"))]
    #[serde(alias = "BLDG_1C")]
    Bldg1C,
    #[serde(rename(deserialize = "2A", serialize = "BLDG_2A"))]
    #[serde(alias = "BLDG_2A")]
    Bldg2A,
    #[serde(rename(deserialize = "2B", serialize = "BLDG_2B"))]
    #[serde(alias = "BLDG_2B")]
    Bldg2B,
    #[serde(rename(deserialize = "2C", serialize = "BLDG_2C"))]
    #[serde(alias = "BLDG_2C")]
    Bldg2C,
    #[serde(rename(deserialize = "2D", serialize = "BLDG_2D"))]
    #[serde(alias = "BLDG_2D")]
    Bldg2D,
    #[serde(rename(deserialize = "2H", serialize = "BLDG_2H"))]
    #[serde(alias = "BLDG_2H")]
    Bldg2H,
    #[serde(rename(deserialize = "3A", serialize = "BLDG_3A"))]
    #[serde(alias = "BLDG_3A")]
    Bldg3A,
    #[serde(rename(deserialize = "3B", serialize = "BLDG_3B"))]
    #[serde(alias = "BLDG_3B")]
    Bldg3B,
    #[serde(rename(deserialize = "5C", serialize = "BLDG_5C"))]
    #[serde(alias = "BLDG_5C")]
    Bldg5C,
}

impl fmt::Display for Building {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::StgUNI => write!(f, "UNITEDステージ"),
            Self::Stg1A => write!(f, "1Aステージ"),
            Self::Bldg1E => write!(f, "1E"),
            Self::Bldg1B => write!(f, "1B"),
            Self::Bldg1C => write!(f, "1C"),
            Self::Bldg2A => write!(f, "2A"),
            Self::Bldg2B => write!(f, "2B"),
            Self::Bldg2C => write!(f, "2C"),
            Self::Bldg2D => write!(f, "2D"),
            Self::Bldg2H => write!(f, "2H"),
            Self::Bldg3A => write!(f, "3A"),
            Self::Bldg3B => write!(f, "3B"),
            Self::Bldg5C => write!(f, "5C"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PeriodOfTime {
    pub starts_at: DateTime<Local>,
    pub ends_at: DateTime<Local>,
}
