use super::model::read::GetContents;
use crate::model::Config;
use anyhow::{anyhow, Result};

pub async fn get_id_by_project_code(config: &Config, project_code: &str) -> Result<u16> {
    info!("Getting id by project code: {}", project_code);

    let url = format!("{}/api/contents", &config.strapi_base);
    let client = reqwest::Client::new();
    let result = client
        .get(url)
        .bearer_auth(&config.strapi_token.as_str())
        .query(&[
            ("populate", "thumbnail,period_of_time"),
            ("filters[project_code][$eq]", project_code),
        ])
        .send()
        .await?;

    info!("GET {} returned {}", result.url().as_str(), result.status());

    if result.status() != reqwest::StatusCode::OK {
        return Err(anyhow::anyhow!(format!(
            "Server responded with {}. Reason: \n {:#?}",
            result.status(),
            result
        )));
    }

    let contents = result.json::<GetContents>().await?;

    let contents = contents.data;

    if contents.len() != 1 {
        return Err(anyhow!(
            "Not exactly one content for {}. Found {} contents",
            project_code,
            contents.len()
        ));
    }

    let id = contents[0].id;
    info!("Project {} translated to {}", &project_code, id);

    Ok(id)
}
