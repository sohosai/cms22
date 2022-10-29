use crate::model::{Config, Message, User};
use crate::sos_data::{get_project_by_user, get_user};
use crate::strapi::is_editable;
use crate::strapi::{
    model::{write::Content, ReviewStatus},
    update_content,
};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Input {
    pub content_type: crate::strapi::model::ContentType,
    pub content_html: Option<String>,
    pub content_url: Option<String>,
}

impl Input {
    fn into_strapi(self, project_code: &str) -> Content {
        Content {
            content_type: Some(self.content_type),
            project_code: Some(project_code.to_string()),
            thumbnail: None,
            content_html: Some(self.content_html),
            content_url: Some(self.content_url),
            review_status: Some(Some(ReviewStatus::Pending)),
            editable: None,
            description: None,
            class: None,
            location_building: None,
            location_room: None,
            period_of_time: None,
        }
    }
}

pub async fn run(
    config: Config,
    user: User,
    project_code_uriencoded: String,
    input: Input,
) -> Result<impl warp::Reply, Infallible> {
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

    info!("Updating content of {} by {}", project_code, user.user_id);

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

    if !me.role.is_committee() {
        let pj_is_editable = match is_editable(&config, &project_code).await {
            Ok(b) => b,
            Err(e) => {
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

    match update_content(&config, &project_code, &input.into_strapi(&project_code)).await {
        Ok(_) => {
            info!(
                "Succeed to update content of {} by {}.",
                project_code, user.user_id
            );
            Ok(warp::reply::with_status(
                warp::reply::json(&Message::new("success")),
                warp::http::StatusCode::OK,
            ))
        }
        Err(e) => {
            error!("Error while updating content: {}", e);
            Ok(warp::reply::with_status(
                warp::reply::json(&Message::new(
                    "You have right permission, but something went wrong updating content.",
                )),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            ))
        }
    }
}
