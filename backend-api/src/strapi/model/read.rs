use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::Building;

/*
 * Get request common types
 */
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    pub page: u32,
    pub page_size: u32,
    pub page_count: u32,
    pub total: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetMetaData {
    pub pagination: Pagination,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SingleGetDataItem<T> {
    pub id: u16,
    pub attributes: T,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SingleGetData<T> {
    pub data: Option<SingleGetDataItem<T>>,
}

/*
 * Get contents
 */

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetContents {
    pub data: Vec<SingleGetDataItem<GetContentsItem>>,
    pub meta: GetMetaData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetContentsItem {
    pub project_code: String,
    pub content_type: super::ContentType,
    pub content_html: Option<String>,
    pub content_url: Option<String>,
    pub thumbnail: SingleGetData<super::Thumbnail>,
    pub editable: bool,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
    #[serde(rename = "publishedAt")]
    pub published_at: Option<DateTime<Utc>>,
    pub review_status: super::ReviewStatus,
    pub location_building: Option<Building>,
    pub location_room: Option<String>,
    pub stage_start: Option<DateTime<Utc>>,
    pub stage_end: Option<DateTime<Utc>>,
    pub class: Option<String>,
    pub description: Option<String>,
}
