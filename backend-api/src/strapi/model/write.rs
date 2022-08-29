use super::{ContentType, ReviewStatus, Thumbnail};
use serde::{Deserialize, Serialize};

// Outside Option is to indicate if that field is omitted(don't cange) and inner one means the
// value is null or some value

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Content {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<Option<u16>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<ContentType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_html: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_url: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_status: Option<Option<ReviewStatus>>,
}

impl Content {
    pub fn init(project_code: &str) -> Self {
        Content {
            project_code: Some(project_code.to_string()),
            thumbnail: None,
            content_type: Some(ContentType::ArticleContent),
            content_html: None,
            content_url: None,
            review_status: Some(Some(ReviewStatus::NeverSubmitted)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Meta {
    pub initialized: bool,
}

/*
 * Utility traits
 * Convers to post and put objects that are wrapped in a data field
 */

#[derive(Serialize, Debug)]
pub struct PostData<T> {
    pub data: T,
}

#[derive(Serialize, Debug)]
pub struct PutData<T> {
    pub data: T,
}

pub trait ApiData: Sized + Serialize + Clone {
    fn for_post(&self) -> PostData<Self> {
        PostData { data: self.clone() }
    }

    fn for_put(&self) -> PutData<Self> {
        PutData { data: self.clone() }
    }
}

impl ApiData for Content {}
impl ApiData for Meta {}
