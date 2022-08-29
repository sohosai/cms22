use super::{
    check_update_period,
    get_contents::{get_contents, GetContentsConfig},
};
use crate::model::Config;
use anyhow::{anyhow, Result};

pub async fn is_editable(config: &Config, project_code: &str) -> Result<bool> {
    let is_in_update_period = check_update_period(config).await?;
    if is_in_update_period {
        return Ok(true);
    }

    let project = get_contents(
        config,
        &GetContentsConfig {
            project_code: Some(project_code.to_string()),
            is_committee: None,
        },
    )
    .await?;
    if (project.len() != 1) {
        return Err(anyhow!(
            "Not exactly one content found for project_code {}",
            project_code
        ));
    }

    let project = project[0].clone();
    let project_is_ediable = project.editable;
    Ok(project_is_ediable)
}
