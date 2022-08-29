use crate::handlers;
use crate::model::Config;
use warp::Filter;

mod auth;
use auth::with_auth;

// Filter components

fn list() -> warp::filters::BoxedFilter<()> {
    warp::path("list").boxed()
}

fn contents() -> warp::filters::BoxedFilter<()> {
    warp::path("contents").boxed()
}

fn my() -> warp::filters::BoxedFilter<()> {
    warp::path("my").boxed()
}

fn project_code() -> warp::filters::BoxedFilter<(String,)> {
    warp::path::param::<String>().boxed()
}

fn review() -> warp::filters::BoxedFilter<()> {
    warp::path("review").boxed()
}

fn thumbnail() -> warp::filters::BoxedFilter<()> {
    warp::path("thumbnail").boxed()
}

fn save_ability() -> warp::filters::BoxedFilter<()> {
    warp::path("save-ability").boxed()
}

fn me() -> warp::filters::BoxedFilter<()> {
    warp::path("me").boxed()
}

fn get() -> warp::filters::BoxedFilter<()> {
    warp::get().boxed()
}

fn put() -> warp::filters::BoxedFilter<()> {
    warp::put().boxed()
}

// Endpoint definitions

fn get_my_contents(
    config: Config,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get()
        .and(contents())
        .and(my())
        .and(with_auth())
        .and_then(move |user| handlers::get_my_contents(config.clone(), user))
}

fn get_one_content(
    config: Config,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get()
        .and(contents())
        .and(project_code())
        .and(with_auth())
        .and_then(move |user, project_code| {
            handlers::get_one_content(config.clone(), user, project_code)
        })
}

fn list_all_contents(
    config: Config,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get()
        .and(contents())
        .and(list())
        .and(with_auth())
        .and_then(move |user| handlers::list_all_contents(config.clone(), user))
}

fn put_content(
    config: Config,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    put()
        .and(contents())
        .and(project_code())
        .and(with_auth())
        .and(warp::body::json())
        .and_then(move |project_code, user, body| {
            handlers::put_content(config.clone(), user, project_code, body)
        })
}

fn put_review(
    config: Config,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    put()
        .and(contents())
        .and(project_code())
        .and(review())
        .and(with_auth())
        .and(warp::body::json())
        .and_then(move |project_code, user, body| {
            handlers::put_review(config.clone(), user, project_code, body)
        })
}

fn put_thumbnail(
    config: Config,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    put()
        .and(contents())
        .and(project_code())
        .and(thumbnail())
        .and(with_auth())
        .and(warp::body::json())
        .and_then(move |project_code, user, body| {
            handlers::put_thumbnail(config.clone(), user, project_code, body)
        })
}

fn check_save_ability(
    config: Config,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get()
        .and(contents())
        .and(project_code())
        .and(save_ability())
        .and(with_auth())
        .and_then(move |project_code, user| {
            handlers::check_save_ability(config.clone(), user, project_code)
        })
}

fn get_my_profile(
    config: Config,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get()
        .and(me())
        .and(with_auth())
        .and_then(move |user| handlers::get_my_profile(config.clone(), user))
}

pub fn filter(
    config: &Config,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_my_profile(config.clone())
        .or(check_save_ability(config.clone()))
        .or(list_all_contents(config.clone()))
        .or(get_my_contents(config.clone()))
        .or(get_one_content(config.clone()))
        .or(put_review(config.clone()))
        .or(put_thumbnail(config.clone()))
        .or(put_content(config.clone()))
        .with(warp::log("INFO"))
        .with(
            warp::cors()
                .allow_any_origin()
                .allow_headers(vec!["authorization", "content-type"])
                .allow_methods(vec!["GET", "PUT"]),
        )
}
