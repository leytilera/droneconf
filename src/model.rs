use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Request {
    pub repo: Repository,
}

#[derive(Deserialize)]
pub struct Repository {
    pub config_path: String,
}

#[derive(Serialize)]
pub struct Response {
    pub data: String,
}

#[derive(Clone)]
pub struct APIConfig(pub Client);

impl Request {
    pub fn config(&self) -> String {
        self.repo.config_path.clone()
    }
}
