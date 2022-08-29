use crate::strapi::is_editable;
use crate::model::{Config, Message, User};
use crate::sos_data::{get_project_by_user, get_user};
use crate::strapi::{
    model::{write::Content, ReviewStatus},
    update_content, upload_file,
};

use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Input {
    pub base64: String,
    pub mime: String,
}

pub async fn run(
    config: Config,
    user: User,
    project_code: String,
    input: Input,
) -> Result<impl warp::Reply, Infallible> {
    info!(
        "Updating thumbnail for project {} by {}",
        project_code, user.user_id
    );

    let me = get_user(&config, &user.user_id).unwrap().unwrap();
    let my_project = get_project_by_user(&config, &user.user_id).unwrap();

    let is_my_project = match my_project {
        Some(project) => project.project_code == project_code,
        None => false,
    };

    if !is_my_project && !me.role.is_committee() {
        return Ok(warp::reply::with_status(
            warp::reply::json(&Message::new("権限がありません")),
            warp::http::StatusCode::FORBIDDEN,
        ));
    }

    if !me.role.is_committee(){
        let pj_is_editable = match is_editable(&config, &project_code).await{
            Ok(b)=>b,
            Err(e)=>{
                error!("Failed to check editable: {}", e);
                return Ok(warp::reply::with_status(
                    warp::reply::json(&Message::new("編集権限を確認できませんでした。")),
                    warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                ));
            }
        };

        if !pj_is_editable {
            return Ok(warp::reply::with_status(
                warp::reply::json(&Message::new("期間外です")),
                warp::http::StatusCode::FORBIDDEN,
            ));
        }
    }
    

    let filename = format!(
        "thumbnail_{}_{}",
        project_code,
        Utc::now().format("%Y-%m-%d_%H-%M-%S").to_string()
    );

    let data = base64::decode(&input.base64).unwrap();

    info!("Saving thumbnail to {}", filename);
    let id = match upload_file(&config, &filename, &input.mime, &data).await {
        Ok(id) => id,
        Err(e) => {
            error!("{}", e);
            return Ok(warp::reply::with_status(
                warp::reply::json(&Message::new(
                    "You have right permission, but something went wrong while uploading",
                )),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            ));
        }
    };

    info!(
        "Updating project {} content with thumbnail id {}",
        project_code, id
    );

    let res = update_content(
        &config,
        &project_code,
        &Content {
            project_code: None,
            content_type: None,
            content_html: None,
            content_url: None,
            review_status: Some(Some(ReviewStatus::Pending)),
            thumbnail: Some(Some(id)),
            editable: None,
        },
    )
    .await;

    match res {
        Err(e) => {
            error!("Could not update content: {}", e);
            return Ok(warp::reply::with_status(
                warp::reply::json(&Message::new(
                    "You have right permission, but something went wrong while updating thumbnail",
                )),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            ));
        }
        Ok(_) => {
            info!("Updated thumbnail for project {}", project_code);
            Ok(warp::reply::with_status(
                warp::reply::json(&Message::new("Success")),
                warp::http::StatusCode::OK,
            ))
        }
    }
}
