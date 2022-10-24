use crate::model::{Config, Content, Message, User};
use crate::sos_data::get_user;
use crate::strapi;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Output {
    contents: Vec<Content>,
}

/*  get /contents/list
 *  機能: 存在する全てのコンテンツを取得する。
 *  パラメータ:なし
 *  認証: 必須
 *  権限: 実委人
 *  -----
 *  NOTE: 既知の問題
 *  - 高々200程度しか企画がないので無茶をしているが、普通にo(n)で気分は悪い。
 *  - listと名乗ってはいるが全部取得してくるので、allみたいなURLの方がbetterな気がする
 */

pub async fn run(config: Config, user: User) -> Result<impl warp::Reply, Infallible> {
    let me = get_user(&config, &user.user_id).unwrap().unwrap();
    if !me.role.is_committee() {
        return Ok(warp::reply::with_status(
            warp::reply::json(&Message::new("権限がありません")),
            warp::http::StatusCode::FORBIDDEN,
        ));
    }

    let contents = strapi::get_contents::get_contents(
        &config,
        &strapi::get_contents::GetContentsConfig {
            project_code: None,
            is_committee: None,
            updated_since: None,
        },
    )
    .await;

    if let Err(err) = contents {
        error!("{:#?}", err);
        return Ok(warp::reply::with_status(
            warp::reply::json(&Message::new("error getting contents")),
            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }

    Ok(warp::reply::with_status(
        warp::reply::json(&Output {
            contents: contents
                .unwrap()
                .iter()
                .map(|item| Content::from(item.clone(), &config))
                .collect(),
        }),
        warp::http::StatusCode::OK,
    ))
}
