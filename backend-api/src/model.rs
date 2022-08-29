use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Debug)]
pub struct Config {
    pub strapi_base: String,
    pub sos_users_csv: String,
    pub sos_projects_csv: String,
    pub strapi_token: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct User {
    pub user_id: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Message {
    pub msg: String,
}

impl Message {
    pub fn new(msg: &str) -> Self {
        Message {
            msg: msg.to_string(),
        }
    }
}
impl warp::Reply for Message {
    fn into_response(self) -> warp::reply::Response {
        warp::http::Response::new(self.msg.into())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Content {
    pub content_type: crate::strapi::model::ContentType,
    pub project_code: String,
    pub thumbnail: Option<String>,
    pub content_html: String,
    pub content_url: String,
    pub status: crate::strapi::model::ReviewStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Content {
    pub fn from(item: crate::strapi::model::read::GetContentsItem, config: &Config) -> Self {
        let thumbnail = match item.thumbnail.data {
            Some(data) => Some(format!("{}{}", config.strapi_base, data.attributes.url)),
            None => None,
        };

        Self {
            content_type: item.content_type,
            project_code: item.project_code,
            thumbnail,
            content_html: item.content_html.unwrap_or("".to_string()),
            content_url: item.content_url.unwrap_or("".to_string()),
            status: item.review_status,
            created_at: item.created_at,
            updated_at: item.updated_at,
        }
    }
}
