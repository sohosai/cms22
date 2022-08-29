use super::model::{
    read::SingleGetData,
    write::{ApiData, Content, Meta},
};

use crate::model::Config;
use crate::sos_data::load_projects;
use anyhow::{anyhow, Result};

pub async fn check_if_initialized(config: &Config) -> Result<bool> {
    let url = format!("{}/api/meta", config.strapi_base);
    let client = reqwest::Client::new();
    let result = client
        .get(url)
        .bearer_auth(config.strapi_token.as_str())
        .send()
        .await?;

    if result.status() == reqwest::StatusCode::NOT_FOUND {
        return Ok(false);
    }

    let meta = result.json::<SingleGetData<Meta>>().await?;
    info!("Meta document found. Initialized: {:?}", &meta);

    if meta.data.is_some() {
        Ok(meta.data.unwrap().attributes.initialized)
    } else {
        Ok(false)
    }
}

pub async fn run_init(config: &Config) -> Result<()> {
    info!("Initialiazing strapi records...");

    let post_url = format!("{}/api/contents", config.strapi_base);
    let client = reqwest::Client::new();

    // Load projects from csv
    let projects = load_projects(config)?;
    info!("{} projects loaded", projects.len());
    info!("Trying to post at {}", post_url);

    // Post default project records
    for project in projects {
        let content = Content::init(&project.project_code);
        let req_body = content.for_post();
        let res = client
            .post(&post_url)
            .bearer_auth(&config.strapi_token)
            .json(&req_body)
            .send()
            .await?;

        if res.status() != reqwest::StatusCode::OK {
            return Err(anyhow!(
                "Error posting project record. reason:\n {:#?} \ndata:\n {:#?}",
                res,
                req_body
            ));
        }

        info!(
            "Initializing {} {}",
            project.project_code, project.project_name
        );
    }

    // Mark as initialized
    let meta = Meta { initialized: true };
    client
        .put(format!("{}/api/meta", config.strapi_base))
        .bearer_auth(&config.strapi_token)
        .json(&meta.for_put())
        .send()
        .await?;

    Ok(())
}

pub async fn init(config: &Config) -> Result<()> {
    if check_if_initialized(config).await? {
        info!("Strapi already initialized");
    } else {
        run_init(config).await?;
    }

    Ok(())
}
