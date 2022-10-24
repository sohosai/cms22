use crate::cache::Cache;
use crate::model::{Config};
use crate::sos_data:: ProjectRecord;
use crate::sos_data::model::ProjectCategory;
use crate::strapi::model::read::GetContentsItem;
use chrono::{DateTime,Utc};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use futures::future;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Place{
 is_online: bool,
 building: Option<String>,
 room: Option<String>
}

impl From<&GetContentsItem> for Place{
 fn from(item:&GetContentsItem)->Self{
  let is_online = item.project_code.contains('O');
  let building =serde_lexpr::to_string(&item.location_building).ok().map(|s| s.replace('(', "").replace(')', ""));
  let room = item.location_room.clone();
  Place{is_online,building,room}
 }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PeriodOfTime{
 starts_at: DateTime<Utc>,
 ends_at: DateTime<Utc>
}

impl From<&GetContentsItem> for Option<PeriodOfTime>{
 /// This conversion assumes that `stage_start` and `stage_end` is `Some()`
 fn from(item:&GetContentsItem)->Self{
  if item.project_code.contains('S') && item.stage_start.is_some() && item.stage_end.is_some(){
   Some(PeriodOfTime { starts_at:item.stage_start.unwrap() , ends_at: item.stage_end.unwrap() })
  }else{
   None
  }
 }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
struct Project{
 project_code	:String,
 project_name	:String,
 project_name_kana	:String,
 organization_name	:String,
 organization_name_kana	:String,
 description	:String,
 project_category	: ProjectCategory,
 is_academic	: bool,
 is_art	: bool,
 is_committee: bool,
 place :Place,
 period_of_time:Option<PeriodOfTime>,
 project_class	: Option<String>,
}

fn parse_bool(s: &str)->bool{
 s.to_ascii_lowercase().contains("true")
}

impl Project{
  pub fn from(project: ProjectRecord,content: GetContentsItem) -> Self {
   Self{
    project_code	:project.project_code,
    project_name	:project.project_name,
    project_name_kana	:project.project_name_kana,
    organization_name	:project.organization_name,
    organization_name_kana	:project.organization_name_kana,
    description	:content.description.clone().unwrap_or_else(|| "".to_string()),
    project_category	: project.project_category,
    is_academic	: parse_bool(&project.is_academic),
    is_art	: parse_bool(&project.is_art),
    is_committee: parse_bool(&project.is_committee),
    place : (&content).into(),
    period_of_time:(&content).into(),
    project_class	: content.class,
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
    let projects = cache.projects(&config).to_owned();

    cache.pull_content_updates(&config).await;

    let cache = &cache;
    let cache_ref=&cache;
   
    let projects:Vec<_>= projects
        .into_iter()
        .map(|(project_code,project)| async move{
         let content = cache_ref.get_content(&project_code).await;
         content.map(|content| Project::from(project, content))
        })
        .collect();

     let projects = future::join_all(projects).await;
     let projects = projects.iter().filter_map(|x| x.clone()).collect();

    info!("Listing all projects succeeded");
    Ok(warp::reply::with_status(
        warp::reply::json(&Output {
            projects
        }),
        warp::http::StatusCode::OK,
    ))
}
