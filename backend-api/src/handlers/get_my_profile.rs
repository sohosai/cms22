use crate::model::UserProfile;
use crate::model::{Config, User};
use crate::sos_data::get_user;
use std::convert::Infallible;

/*  get /me
 *  機能:
 *  自分のプロフィールを取得。
 *  認証: 必須
 *  -----
 */

pub async fn run(config: Config, user: User) -> Result<impl warp::Reply, Infallible> {
    let me = get_user(&config, &user.user_id).unwrap().unwrap();

    let profile: UserProfile = me.into();
    Ok(warp::reply::with_status(
        warp::reply::json(&profile),
        warp::http::StatusCode::OK,
    ))
}
