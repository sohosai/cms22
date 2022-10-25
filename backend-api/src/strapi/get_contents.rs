use super::model::read::{GetContents, GetContentsItem};
use crate::model::Config;
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct GetContentsConfig {
    pub project_code: Option<String>,
    pub is_committee: Option<bool>,
    pub updated_since: Option<DateTime<Utc>>,
}

impl GetContentsConfig {
    pub fn to_query(&self) -> Vec<(String, String)> {
        let mut query = Vec::new();
        if let Some(project_code) = &self.project_code {
            query.push((
                "filters[$or][0][project_code][$eq]".to_string(),
                project_code.to_string(),
            ));
        }

        if self.is_committee.unwrap_or(false) {
            query.push((
                format!("filters[$or][{}][project_code][$startsWith]", query.len()),
                "本企".to_string(),
            ));
        }

        if let Some(updated_after) = self.updated_since {
            query.push((
                "filters[updatedAt][$gt]".to_string(),
                updated_after.timestamp_millis().to_string(),
            ));
        }

        query
    }
}

pub async fn get_contents(
    config: &Config,
    option: &GetContentsConfig,
) -> Result<Vec<GetContentsItem>> {
    info!("Searching contents matching {:?}", option);

    let client = reqwest::Client::new();
    let (items, page_count) = get_contents_in_page(&client, config, option, 1).await?;

    info!(
        "Downloading first page complete. page_count: {}",
        page_count
    );

    let mut items = items;

    for page in 2..page_count + 1 {
        let (page_items, _) = get_contents_in_page(&client, config, option, page)
            .await
            .with_context(|| {
                format!("Error getting contents in page {} of {}", page, page_count)
            })?;
        items.extend(page_items);
    }

    info!("{} items found.", items.len());
    Ok(items)
}

pub async fn get_contents_in_page(
    client: &reqwest::Client,
    config: &Config,
    option: &GetContentsConfig,
    page: u32,
) -> Result<(Vec<GetContentsItem>, u32)> {
    let url = format!("{}/api/contents", &config.strapi_base);
    let result = client
        .get(url)
        .query(&option.to_query())
        .query(&[("populate", "thumbnail")])
        .query(&[("pagination[page]", page.to_string())])
        .bearer_auth(&config.strapi_token.as_str())
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
    let contents = result
        .json::<GetContents>()
        .await
        .with_context(|| format!("Parse error while parsing response at page {}", page))?;

    let items = contents
        .data
        .iter()
        .map(|x| {
            let mut c = x.attributes.clone();
            if (&c).content_html.is_some() && c.content_html.clone().unwrap().is_empty() {
                c.content_html = None;
            }

            if (&c).content_url.is_some() && c.content_url.clone().unwrap().is_empty() {
                c.content_url = None;
            }

            c
        })
        .collect();
    let page_count = contents.meta.pagination.page_count;

    info!("Download success at page {}", page);
    Ok((items, page_count))
}
