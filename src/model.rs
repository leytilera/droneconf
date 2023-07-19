use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Request {
    pub repo: Repository,
    pub configs: Option<Vec<WoodpeckerConfig>>,
}

#[derive(Deserialize)]
pub struct Repository {
    pub config_path: Option<String>,
    pub config_file: Option<String>,
    pub config: Option<String>,
}

#[derive(Serialize)]
pub struct Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configs: Option<Vec<WoodpeckerConfig>>,
}

#[derive(Serialize, Deserialize)]
pub struct WoodpeckerConfig {
    pub name: String,
    pub data: String,
}

#[derive(Clone)]
pub struct APIConfig(pub Client);

impl Request {
    pub fn config(&self) -> Option<String> {
        self.repo
            .config_path
            .clone()
            .or(self.repo.config_file.clone())
            .or(self.repo.config.clone())
    }

    pub fn is_woodpecker(&self) -> bool {
        self.configs.is_some()
    }
}

impl Response {
    pub fn new(data: String, is_woodpecker: bool) -> Self {
        if is_woodpecker {
            Self {
                data: Some(data),
                configs: None,
            }
        } else {
            Self {
                data: None,
                configs: Some(vec![WoodpeckerConfig {
                    name: "central-override".to_string(),
                    data,
                }]),
            }
        }
    }
}
