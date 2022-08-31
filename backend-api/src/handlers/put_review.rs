use crate::model::{Config, Message, User};
use crate::sos_data::get_user;
use crate::strapi::is_editable;
use crate::strapi::{
    model::{write::Content, ReviewStatus},
    update_content,
};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Input {
    pub review_status: ReviewStatus,
}

impl Into<Content> for Input {
    fn into(self) -> Content {
        Content {
            project_code: None,
            thumbnail: None,
            content_type: None,
            content_html: None,
            content_url: None,
            review_status: Some(Some(self.review_status)),
            editable: None,
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
                warp::reply::json(&Message::new(&format!("Error while decoding project_code"))),
                warp::http::StatusCode::BAD_REQUEST,
            ));
        }
    };

    info!("Updating review of {} by {}", project_code, user.user_id);

    let me = get_user(&config, &user.user_id).unwrap().unwrap();
    if !me.role.is_committee() {
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

    match update_content(&config, &project_code, &input.into()).await {
        Ok(_) => {
            info!(
                "Succeed to update review of {} by {}.",
                project_code, user.user_id
            );
            Ok(warp::reply::with_status(
                warp::reply::json(&Message::new("success")),
                warp::http::StatusCode::OK,
            ))
        }
        Err(e) => {
            error!(
                "Failed to update review of {} by {}: {}",
                project_code, user.user_id, e
            );
            Ok(warp::reply::with_status(
                warp::reply::json(&Message::new("failed")),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            ))
        }
    }
}
