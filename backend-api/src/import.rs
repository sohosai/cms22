// This is just an utility binary to import the data from the CSV. It's not the part of the API.
#[macro_use]
extern crate log;
mod strapi;
use std::env;

mod model;
use anyhow::Result;
use chrono::{DateTime, Local};
use model::Config;
use serde::{Deserialize, Serialize};
use strapi::model::Building;
use strapi::update_content;
mod sos_data;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ImportingData {
    project_code: String,
    description: String,
    category: String,
    building: Option<Building>,
    room: Option<String>,
    #[serde(with="datefmt")]
    starts_at: Option<DateTime<Local>>,
    #[serde(with="datefmt")]
    ends_at: Option<DateTime<Local>>,
}


pub mod datefmt {
    const FORMAT: &str = "%m/%d/%Y %H:%M:%S";
    use chrono::{DateTime,Local,TimeZone};
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(date: &Option<DateTime<Local>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.unwrap());
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Local>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let d:Result<DateTime<Local>, D::Error>  = Local.datetime_from_str(&s, FORMAT).map_err(serde::de::Error::custom);
        match d {
            Ok(d) => Ok(Some(d)),
            Err(_) => Ok(None),
        }
    }
}

async fn load_csv_data(file: &str) -> Result<Vec<ImportingData>> {
    let mut reader = csv::Reader::from_path(file)?;
    Ok(reader.deserialize().filter_map(|r| r.ok()).collect())
}

async fn import_data(config: &Config, data: &ImportingData) -> Result<()> {
    let project_code = data.project_code.clone();
    let content = strapi::model::write::Content {
        project_code: Some(data.project_code.clone()),
        thumbnail: None,
        content_type: None,
        content_html: None,
        content_url: None,
        review_status: None,
        editable: None,
        description: Some(data.description.clone()),
        class: Some(data.category.clone()),
        location_building: data.building.clone(),
        location_room: data.room.clone(),
        stage_start: data.starts_at,
        stage_end: data.ends_at,
    };

    update_content(config, &project_code, &content).await?;
    Ok(())
}


#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    info!("Starting up...");

    let config = match envy::from_env::<model::Config>() {
        Ok(config) => config,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("Loading from {}",filename);
    let data = load_csv_data(filename).await?;
    println!("Importing {} records",data.len());
    
    for proj in data {
        println!("Importing {}",proj.project_code);
        import_data(&config, &proj).await?;
    }
    Ok(())
}
