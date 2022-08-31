use crate::model::Config;
use anyhow::{anyhow, Result};
use reqwest::multipart;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Response {
    id: u16,
}

pub async fn upload_file(config: &Config, name: &str, mime: &str, data: &[u8]) -> Result<u16> {
    info!(
        "Uploading file {} with content of {} bytes",
        name,
        data.len()
    );

    let post_url = format!("{}/api/upload", config.strapi_base);

    let item = multipart::Part::bytes(data.to_vec())
        .mime_str(mime)?
        .file_name(name.to_string());

    let form = multipart::Form::new().part("files", item);

    let client = reqwest::Client::new();
    let res = client
        .post(&post_url)
        .bearer_auth(&config.strapi_token)
        .multipart(form)
        .send()
        .await?;

    if res.status() != reqwest::StatusCode::OK {
        return Err(anyhow!(
            "Error posting project record. reason:\n {:#?}",
            res,
        ));
    }

    let resp = res.json::<Vec<Response>>().await?;
    let id = resp[0].id;

    info!("Uploaded file with id {}", id);
    Ok(id)
}
