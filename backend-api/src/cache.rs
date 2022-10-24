use crate::model::Config;
use crate::sos_data::{load_all_projects, ProjectRecord};
use crate::strapi::get_contents::{get_contents, GetContentsConfig};
use crate::strapi::model::read::GetContentsItem;
use anyhow::Context;
use anyhow::Result;
use chrono::{DateTime, Duration, TimeZone, Utc};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::{collections::HashMap, ops::Add};
use crate::handlers::list_all_projects::Project as ProjectListItem;

pub type Cache = Arc<Mutex<CacheInner>>;

#[derive(Clone,Debug)]
pub struct CacheInner {
    cache_ttl: Duration,
    output_cache_ttl: Duration,
    projects: (HashMap<String, ProjectRecord>, DateTime<Utc>), // SOS Data. (cache<project_code,project> , expire)
    contents: (HashMap<String, GetContentsItem>, DateTime<Utc>), // Strapi Data. (cache<project_code,content> , last_updated)
    project_list: (Vec<ProjectListItem>, DateTime<Utc>), // Output cache. Expires soon.
}

pub fn empty_cache()-> Cache{
    Arc::new(Mutex::new(CacheInner::empty()))
}

impl CacheInner{
pub fn empty() -> Self {
    Self{
        cache_ttl: Duration::minutes(15),
        output_cache_ttl: Duration::minutes(1),
        projects: (HashMap::new(), Utc.timestamp(0, 0)),
        contents: (HashMap::new(), Utc.timestamp(0, 0)),
        project_list: (Vec::new(), Utc.timestamp(0, 0)),
    }
}
}

/// Project list
impl CacheInner {
    pub fn load_projects(&mut self, config: &Config) -> Result<Vec<ProjectRecord>> {
        info!("SOS file cache expired. Updating projects from the file...");
        let projects = load_all_projects(config).with_context(|| "failed to load all projects")?;
        info!("File load succeed");
        self.store_projects(projects.clone());
        info!("SOS file cache updated.");
        Ok(projects)
    }

    pub fn store_projects(&mut self, projects: Vec<ProjectRecord>) {
        let projects = projects
            .into_iter()
            .map(|x| (x.project_code.clone(), x))
            .collect::<HashMap<_, _>>();
        info!("Storing projects to cache. Awaiting lock...");
        info!("Lock acquired. Storing projects to cache...");
        self.projects = (projects, Utc::now().add(self.cache_ttl));
    }

    pub fn get_project(&mut self, project_code: &str, config: &Config) -> Option<ProjectRecord> {
       
        if self.projects.1 < Utc::now() {
            self.load_projects(config);
        }

        let found = self.projects.0.get(project_code);

        found.cloned()
    }

    pub fn projects(&mut self,config:&Config)->HashMap<String,ProjectRecord>{
     if self.projects.1 < Utc::now() {
         self.load_projects(config);
     }

     self.projects.0.clone()
    }
}

/// Contents
impl CacheInner {
    pub async fn get_content(
        &self,
        project_code: &str
    ) -> Option<GetContentsItem> {
        self.contents.0.get(project_code).cloned()
    }

    pub async fn pull_content_updates(&mut self, config: &Config) {
        info!("Cache expired. Fetching contents ...");

        let query = GetContentsConfig {
            project_code: None,
            is_committee: None,
            updated_since: Some(self.contents.1),
        };
        let contents = get_contents(config, &query).await;
        if contents.is_err() {
            error!(
                "Failed to update contents cache. Reason: {}",
                contents.err().unwrap()
            );
            return;
        }
        let contents = contents.unwrap();

        for content in contents {
            info!(
                "Updating content cache for project {}",
                content.project_code
            );
            self.store_content(&content.project_code.clone(), content);
        }

        self.contents.1 = Utc::now();
    }

    pub fn store_content(&mut self, project_code: &str, item: GetContentsItem) {
        let (contents, _) = &mut self.contents;
        if contents.contains_key(project_code) {
            contents.remove(project_code);
        }
        contents.insert(project_code.to_string(), item);
    }

    pub async fn contents (&mut self,config:&Config)->HashMap<String,GetContentsItem>{
        self.pull_content_updates(config).await;
        self.contents.0.clone()
    }
}


/// project list
impl CacheInner{
    pub fn get_project_list(&self)->Option<&Vec<ProjectListItem>>{
        if self.project_list.1 < Utc::now() {
             None
        }else{
            Some(self.project_list.0.as_ref())
        }
    }

    pub fn store_project_list(&mut self,project_list:&Vec<ProjectListItem>){
        self.project_list = (project_list.to_vec(),Utc::now().add(self.output_cache_ttl));
    }
}
