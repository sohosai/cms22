#[macro_use]
extern crate log;

mod filters;
mod handlers;
mod model;
mod sos_data;
mod strapi;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    info!("Starting up...");

    let config = match envy::from_env::<model::Config>() {
        Ok(config) => config,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    // Initialize if nessessary
    strapi::init(&config).await?;

    info!("Listening on port 3030");

    warp::serve(filters::filter(&config))
        .run(([0, 0, 0, 0], 3030))
        .await;

    Ok(())
}

// APIS
// GET /contents/my
// 自分の企画一覧を取得する。実委人の場合は本企の企画も見える。

// GET /contents/:project_code
// /contents/myの企画  (実委人なら全ての企画)が取得できる
// コンテンツを読む.
// my-projectに含まれるコンテンツのみ取得できる。(If not-> 403)

// PUT /contents/:project_code
// コンテンツを上書きする。my-projectsに含まれる場合または実委人の場合のみ実行できる( if not -> 403)
// 編集期間 or コンテンツの状態がAllow editの場合のみ実行できる。(if not -> 403)
// Review statusをpendingにする

// PUT /contents/:project_code/review
// 権限: 実委人
// レビューを更新する。
// Review statusを設定する
// Approvedだった場合は、publishedcontentsを上書きする

// PUT /contents/:project_code/thumbnail
// 権限: 実委人
// $STRAPI_HOST/api/upload へのproxyになる

// GET /contents/list
// 権限: 実委人
// 全てのコンテンツの一覧を取得する。
