use crate::sos_data::get_project_by_code;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::sos_data;

#[derive(Clone, Deserialize, Debug)]
pub struct Config {
    pub strapi_base: String,
    pub sos_users_csv: String,
    pub sos_projects_csv: String,
    pub sos_honki_projects_csv: String,
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
pub struct UserProfile {
    pub name: String,
    pub email: String,
    pub is_committee: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Content {
    pub content_type: crate::strapi::model::ContentType,
    pub project_code: String,
    pub thumbnail: Option<String>,
    pub content_html: String,
    pub content_url: String,
    pub status: crate::strapi::model::ReviewStatus,
    pub owner: UserProfile,
    pub subowner: UserProfile,
    pub project_name: String,
    pub project_category: sos_data::model::ProjectCategory,
    pub is_academic: bool,
    pub is_art: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

fn parse_bool_field(s: &str) -> bool {
    match s.to_string().to_lowercase().as_str() {
        "true" => true,
        "false" => false,
        _ => false,
    }
}

impl Content {
    pub fn from(item: crate::strapi::model::read::GetContentsItem, config: &Config) -> Self {
        let project = get_project_by_code(config, &item.project_code)
            .unwrap()
            .expect(format!("Project {} not found", item.project_code).as_str());

        let thumbnail = match item.thumbnail.data {
            Some(data) => Some(format!("{}{}", config.strapi_base, data.attributes.url)),
            None => None,
        };

        let owner = sos_data::get_user(config, &project.owner_user_id)
            .unwrap()
            .unwrap();
        let subowner = sos_data::get_user(config, &project.subowner_user_id)
            .unwrap()
            .unwrap();

        Self {
            content_type: item.content_type,
            project_code: item.project_code,
            thumbnail,
            content_html: item.content_html.unwrap_or_else(|| "".to_string()),
            content_url: item.content_url.unwrap_or_else(|| "".to_string()),
            status: item.review_status,
            created_at: item.created_at,
            updated_at: item.updated_at,
            project_name: project.project_name,
            owner: owner.into(),
            subowner: subowner.into(),
            project_category: project.project_category,
            is_academic: parse_bool_field(&project.is_academic),
            is_art: parse_bool_field(&project.is_art),
        }
    }
}
