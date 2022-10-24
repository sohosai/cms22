use super::model::write::Meta;
use crate::model::Config;
use crate::strapi::model::read::SingleGetData;
use anyhow::{anyhow, Result};

pub async fn check_update_period(config: &Config) -> Result<bool> {
    info!("Checking update period.",);

    let url = format!("{}/api/meta", config.strapi_base);
    let client = reqwest::Client::new();
    let res = client
        .get(url)
        .bearer_auth(config.strapi_token.as_str())
        .send()
        .await?;

    if res.status() != reqwest::StatusCode::OK {
        return Err(anyhow!("Error to get meta data. reason:\n {:#?}", res));
    }

    let meta = res.json::<SingleGetData<Meta>>().await?;
    info!("Meta document found. {:?}", &meta);

    let meta = meta.data.ok_or_else(|| anyhow!("meta.data is None"))?;
    let meta = meta.attributes;

    let now = chrono::Utc::now();
    Ok(meta
        .allow_edit_since
        .ok_or_else(|| anyhow!("allow_edit_since is None"))?
        < now
        && now
            < meta
                .allow_edit_until
                .ok_or_else(|| anyhow!("allow_edit_until is None"))?)
}
