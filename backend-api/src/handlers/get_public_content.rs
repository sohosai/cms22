use crate::cache::Cache;
use crate::model::{Config, Message};
use crate::sos_data::model::ProjectCategory;
use crate::sos_data::ProjectRecord;
use crate::strapi::model::read::GetContentsItem;
use crate::strapi::model::{ContentType, ReviewStatus};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Output {
    project_code: String,
    content_type: ContentType,
    thumbnail: Option<String>,
    content_html: Option<String>,
    content_url: Option<String>,
    project_name: String,
    project_category: ProjectCategory,
    project_class: Option<String>,
    description: Option<String>,
    is_academic: bool,
    is_art: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Output {
    pub fn from(project: ProjectRecord, content: GetContentsItem, config: &Config) -> Self {
        let thumbnail = match content.thumbnail.data {
            Some(data) => Some(format!("{}{}", config.strapi_base, data.attributes.url)),
            None => None,
        };

        Self {
            project_code: content.project_code,
            content_type: content.content_type,
            thumbnail: thumbnail,
            content_html: content.content_html,
            content_url: content.content_url,
            project_name: project.project_name,
            project_category: project.project_category,
            project_class: content.class,
            description: content.description,
            is_academic: project.is_academic.to_lowercase().contains("true"),
            is_art: project.is_art.to_lowercase().contains("true"),
            created_at: content.created_at,
            updated_at: content.updated_at,
        }
    }
}

/*
async fn construct_output(project_code: String, project:ProjectRecord,cache: &Cache,config:  &Config)-> Option<Project>{

}
*/

/*  get /anosymous/contents/{project_code}
 *  機能:企画のコンテンツを取得する
 *  パラメータ:なし
 *  認証: 不要
 *  -----
 */

pub async fn run(
    project_code_uriencoded: String,
    config: Config,
    cache: Cache,
) -> Result<impl warp::Reply, Infallible> {
    info!("Getting project content");
    let project_code = match urlencoding::decode(&project_code_uriencoded) {
        Ok(project_code) => project_code,
        Err(e) => {
            error!("Error while decoding project_code: {}", e);
            return Ok(warp::reply::with_status(
                warp::reply::json(&Message::new("Error while decoding project_code")),
                warp::http::StatusCode::BAD_REQUEST,
            ));
        }
    };

    info!("Project ID: {}", project_code);

    let mut cache = cache.lock().await; // Primary cache lock

    let project = match cache.get_project(&project_code, &config) {
        Some(p) => p,
        None => {
            return Ok(warp::reply::with_status(
                warp::reply::json(&Message::new("Project not found")),
                warp::http::StatusCode::NOT_FOUND,
            ));
        }
    };

    cache.pull_content_updates(&config).await;
    let content = match cache.get_content(&project_code).await {
        Some(c) => c,
        None => {
            return Ok(warp::reply::with_status(
                warp::reply::json(&Message::new("Content not found")),
                warp::http::StatusCode::NOT_FOUND,
            ));
        }
    };

    let content = match content.review_status {
        ReviewStatus::Approved => content,
        ReviewStatus::NeverSubmitted => {
            return Ok(warp::reply::with_status(
                warp::reply::json(&Message::new("No content has submitted")),
                warp::http::StatusCode::NO_CONTENT,
            ));
        }
        ReviewStatus::Pending => {
            return Ok(warp::reply::with_status(
                warp::reply::json(&Message::new("Article is under review")),
                warp::http::StatusCode::CONFLICT,
            ));
        }
        ReviewStatus::Rejected => {
            return Ok(warp::reply::with_status(
                warp::reply::json(&Message::new("Content has rejected")),
                warp::http::StatusCode::CONFLICT,
            ));
        }
    };

    let output = Output::from(project, content, &config);

    info!("Listing all projects succeeded");
    Ok(warp::reply::with_status(
        warp::reply::json(&output),
        warp::http::StatusCode::OK,
    ))
}
