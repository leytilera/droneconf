use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Request {
    pub repo: Repository,
}

#[derive(Deserialize)]
pub struct Repository {
    pub config_path: Option<String>,
    pub config_file: Option<String>,
    pub config: Option<String>,
}

#[derive(Serialize)]
pub struct Response {
    pub data: String,
}

#[derive(Clone)]
pub struct APIConfig(pub Client);

impl Request {
    pub fn config(&self) -> Option<String> {
        self.repo.config_path.clone().or(self.repo.config_file.clone()).or(self.repo.config.clone())
    }
}
