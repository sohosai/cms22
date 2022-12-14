pub mod model;
pub use model::{ProjectRecord, UserRecord};

use crate::model::Config;
use anyhow::Result;

pub fn load_projects(file: &str) -> Result<Vec<ProjectRecord>> {
    let mut reader = csv::Reader::from_path(file)?;
    Ok(reader.deserialize().filter_map(|r| r.ok()).collect())
}

pub fn load_all_projects(config: &Config) -> Result<Vec<ProjectRecord>> {
    let projects = load_projects(&config.sos_projects_csv)?;
    let honki_projects = load_projects(&config.sos_honki_projects_csv)?;
    Ok((projects.into_iter().chain(honki_projects.into_iter())).collect())
}

pub fn load_users(config: &Config) -> Result<Vec<UserRecord>> {
    let mut reader = csv::Reader::from_path(&config.sos_users_csv)?;
    Ok(reader.deserialize().filter_map(|r| r.ok()).collect())
}

// TODO: DON'T load CSV every time.

pub fn get_user(config: &Config, user_id: &str) -> Result<Option<UserRecord>> {
    let users = load_users(config)?;
    Ok(users.iter().find(|u| u.user_id == user_id).cloned())
}

pub fn get_project_by_user(config: &Config, user_id: &str) -> Result<Option<ProjectRecord>> {
    let projects = load_all_projects(config)?;
    Ok(projects
        .iter()
        .find(|p| p.owner_user_id == user_id || p.subowner_user_id == user_id)
        .cloned())
}

pub fn get_project_by_code(config: &Config, project_code: &str) -> Result<Option<ProjectRecord>> {
    let projects = load_all_projects(config)?;
    Ok(projects
        .iter()
        .find(|p| p.project_code == project_code)
        .cloned())
}
