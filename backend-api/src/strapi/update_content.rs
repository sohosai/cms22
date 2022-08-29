use super::get_id_by_project_code::get_id_by_project_code;
use super::model::write::{ApiData, Content};
use crate::model::Config;
use anyhow::{anyhow, Result};

pub async fn update_content(config: &Config, project_code: &str, content: &Content) -> Result<()> {
    let id = get_id_by_project_code(&config, project_code).await?;
    let put_url = format!("{}/api/contents/{}", config.strapi_base, id);

    let req_body = content.for_put();

    let client = reqwest::Client::new();
    let res = client
        .put(&put_url)
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

    Ok(())
}
