use crate::model::{Config, Message, User};
use crate::sos_data::{get_project_by_user, get_user};

use crate::strapi::check_update_period;
use crate::strapi::get_contents::get_contents;
use crate::strapi::get_contents::GetContentsConfig;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Output {
    is_my_project: bool,
    in_save_period: bool,
    is_editable: bool,
    is_committee: bool,
    total: bool,
}

/*  get /contents/:project_code/save-ability
 *  機能:
 *  自分が現時点でその企画を保存する権限を有しているかを調べる。total:true なら権限あり。
 *  パラメータ: 企画ID
 *  認証: 必須
 *  -----
 */

pub async fn run(
    config: Config,
    user: User,
    project_code_uriencoded: String,
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

    info!(
        "Check save ability for {} by {}",
        project_code, &user.user_id
    );
    let me = get_user(&config, &user.user_id).unwrap().unwrap();
    let my_project = get_project_by_user(&config, &user.user_id).unwrap();

    let is_my_project = match my_project {
        Some(project) => project.project_code == project_code,
        None => false,
    };

    let in_save_period = match check_update_period(&config).await {
        Ok(b) => b,
        Err(e) => {
            error!("{}", e);
            return Ok(warp::reply::with_status(
                warp::reply::json(&Message::new("Coudn't get meta.")),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            ));
        }
    };

    let target_content = get_contents(
        &config,
        &GetContentsConfig {
            is_committee: None,
            project_code: Some(project_code.to_string()),
            updated_since: None,
        },
    )
    .await;
    let target_content = match target_content {
        Ok(content) => content,
        Err(e) => {
            error!("{}", e);
            return Ok(warp::reply::with_status(
                warp::reply::json(&Message::new(&format!(
                    "Contents for project {} does not exists",
                    &project_code
                ))),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            ));
        }
    };

    let target_content = match target_content.len() {
        1 => target_content[0].clone(),
        _ => {
            return Ok(warp::reply::with_status(
                warp::reply::json(&Message::new(&format!(
                    "Not exactlt one content for project {}. Found {}",
                    &project_code,
                    target_content.len()
                ))),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            ))
        }
    };

    let is_editable = target_content.editable;
    let is_committee = me.role.is_committee();

    let total = is_committee || (is_my_project && (in_save_period || is_editable));

    info!("Returning save ability");
    Ok(warp::reply::with_status(
        warp::reply::json(&Output {
            is_my_project,
            in_save_period,
            is_editable,
            is_committee,
            total,
        }),
        warp::http::StatusCode::OK,
    ))
}
