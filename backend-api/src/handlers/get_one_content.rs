use crate::model::{Config, Content, Message, User};
use crate::sos_data::{get_project_by_user, get_user};
use crate::strapi;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Output {
    content: Content,
}

/*  get /contents/project_code
 *  機能: 特定の企画のコンテンツを取得する。
 *  パラメータ:project_code(パス)
 *  認証: 必須
 *  権限: 実委人または企画のメンバー
 *  -----
 *  高々200程度しか企画がないので無茶をしているが、普通にo(n)で気分は悪い。
 */

pub async fn run(
    config: Config,
    project_code_uriencoded: String,
    user: User,
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

    let me = get_user(&config, &user.user_id).unwrap().unwrap();
    let my_project = get_project_by_user(&config, &user.user_id).unwrap();

    if me.role.is_committee() {
        info!("Permission granted. reason: committee");
    } else if my_project.is_some() && my_project.unwrap().project_code == project_code {
        info!("Permission granted. reason: project member");
    } else {
        error!("Permission denied.");
        return Ok(warp::reply::with_status(
            warp::reply::json(&Message::new("権限がありません")),
            warp::http::StatusCode::FORBIDDEN,
        ));
    }

    let contents = strapi::get_contents::get_contents(
        &config,
        &strapi::get_contents::GetContentsConfig {
            project_code: Some(project_code.to_string()),
            is_committee: None,
        },
    )
    .await;

    let contents = match contents {
        Ok(contents) => contents,
        Err(err) => {
            error!("{:#?}", err);
            return Ok(warp::reply::with_status(
                warp::reply::json(&Message::new("error getting contents")),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            ));
        }
    };

    if contents.len() != 1 {
        error!("Not exactly one content found for {}", &project_code);
        return Ok(warp::reply::with_status(
            warp::reply::json(&Message::new("Somehow there are not exact one contents")),
            warp::http::StatusCode::CONFLICT,
        ));
    }

    Ok(warp::reply::with_status(
        warp::reply::json(&Output {
            content: Content::from(contents[0].clone(), &config),
        }),
        warp::http::StatusCode::OK,
    ))
}
