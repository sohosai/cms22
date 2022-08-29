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
