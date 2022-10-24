use crate::model::{Config, Content, Message, User};
use crate::sos_data::{get_project_by_user, get_user};
use crate::strapi;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Output {
    contents: Vec<Content>,
}

/*  get /contents/my
 *  機能:
 *  「自分の企画」のコンテンツを取得する。「自分の企画」とは、自分が所属するか、または本部企画(実委人の場合のみ)のこと。
 *  パラメータ: なし
 *  認証: 必須
 *  権限: 実委人または企画のメンバー
 *  -----
 */

pub async fn run(config: Config, user: User) -> Result<impl warp::Reply, Infallible> {
    let me = get_user(&config, &user.user_id).unwrap().unwrap();
    let my_project = get_project_by_user(&config, &user.user_id).unwrap();

    let project_code = match my_project {
        Some(project) => Some(project.project_code),
        None => None,
    };

    let contents = strapi::get_contents::get_contents(
        &config,
        &strapi::get_contents::GetContentsConfig {
            project_code,
            is_committee: Some(me.role.is_committee()),
            updated_since: None,
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

    Ok(warp::reply::with_status(
        warp::reply::json(&Output {
            contents: contents
                .iter()
                .map(|content| Content::from(content.clone(), &config))
                .collect(),
        }),
        warp::http::StatusCode::OK,
    ))
}
