use crate::cache::Cache;
use crate::model::Config;
use crate::sos_data::model::ProjectCategory;
use crate::sos_data::ProjectRecord;
use crate::strapi::model::read::GetContentsItem;
use crate::strapi::model::ContentType;
use crate::strapi::model::ReviewStatus;
use futures::future;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Place {
    is_online: bool,
    building: Option<String>,
    room: Option<String>,
}

impl From<&GetContentsItem> for Place {
    fn from(item: &GetContentsItem) -> Self {
        let is_online = item.project_code.contains('O') || item.is_online.unwrap_or_else(|| false);
        let building = item.location_building.clone().map(|x| format!("{}", x));
        let room = item.location_room.clone();
        Place {
            is_online,
            building,
            room,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    project_code: String,
    project_name: String,
    project_name_kana: String,
    organization_name: String,
    organization_name_kana: String,
    description: String,
    project_category: ProjectCategory,
    is_academic: bool,
    is_art: bool,
    is_committee: bool,
    place: Place,
    period_of_time: Option<Vec<crate::strapi::model::PeriodOfTime>>,
    project_class: Option<String>,
    content_available: bool,
    external_link: Option<String>,
    thumbnail_url: Option<String>,
}

fn parse_bool(s: &str) -> bool {
    s.to_ascii_lowercase().contains("true")
}

impl Project {
    pub fn from(project: ProjectRecord, content: GetContentsItem,config: &Config) -> Self {
        // Return null if period_of_time is null OR EMPTY
        let period_of_time = match content.period_of_time.clone() {
            Some(period_of_time) => {
                if period_of_time.len() == 0 {
                    None
                } else {
                    Some(period_of_time)
                }
            }
            None => None,
        };

        Self {
            project_code: project.project_code,
            project_name: project.project_name,
            project_name_kana: project.project_name_kana,
            organization_name: project.organization_name,
            organization_name_kana: project.organization_name_kana,
            description: content
                .description
                .clone()
                .unwrap_or_else(|| "".to_string()),
            project_category: project.project_category,
            is_academic: parse_bool(&project.is_academic),
            is_art: parse_bool(&project.is_art),
            is_committee: parse_bool(&project.is_committee),
            place: (&content).into(),
            period_of_time,
            project_class: content.class,
            content_available: content.review_status == ReviewStatus::Approved,
            external_link: if content.content_type == ContentType::LinkContent
                && content.content_url.is_some() && content.review_status == ReviewStatus::Approved
            {
                Some(content.content_url.unwrap())
            } else {
                None
            },
            thumbnail_url: if content.thumbnail.data.is_some() {
                Some(format!("{}{}",config.strapi_base,content.thumbnail.data.unwrap().attributes.url))
            } else {
                None
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Output {
    projects: Vec<Project>,
}

/*
async fn construct_output(project_code: String, project:ProjectRecord,cache: &Cache,config:  &Config)-> Option<Project>{

}
*/

/*  get /anosymous/project
 *  機能: 全企画の一覧を取得する
 *  パラメータ:なし
 *  認証: 不要
 *  -----
 */

pub async fn run(config: Config, cache: Cache) -> Result<impl warp::Reply, Infallible> {
    info!("Listing all projects");
    let mut cache = cache.lock().await; // Primary cache lock

    if let Some(projects) = cache.get_project_list() {
        let output = Output {
            projects: projects.to_vec(),
        };
        return Ok(warp::reply::with_status(
            warp::reply::json(&output),
            warp::http::StatusCode::OK,
        ));
    }

    let projects = cache.projects(&config).to_owned();

    cache.pull_content_updates(&config).await;

    let cache_ref = &cache;
    let config = &config;
    let projects: Vec<_> = projects
        .into_iter()
        .map(|(project_code, project)| async move {
            let content = cache_ref.get_content(&project_code).await;
            if content.is_none() {
                warn!("Content not found for project {}", project_code);
            }

            content.map(|content| Project::from(project, content,&config))
        })
        .collect();

    let projects = future::join_all(projects).await;
    let projects: Vec<Project> = projects.iter().filter_map(|x| x.clone()).collect();

    cache.store_project_list(&projects.to_vec());
    info!("Listing all projects succeeded");
    Ok(warp::reply::with_status(
        warp::reply::json(&Output { projects }),
        warp::http::StatusCode::OK,
    ))
}
