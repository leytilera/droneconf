use reqwest::Url;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Request {
    pub repo: Repository,
}

#[derive(Deserialize)]
pub struct Repository {
    pub name: String,
    pub namespace: String,
}

#[derive(Serialize)]
pub struct Response {
    pub data: String,
}

#[derive(Serialize)]
pub struct AuthQuery {
    pub access_token: String,
}

#[derive(Clone)]
pub struct APIConfig(pub Url, pub String, pub String);

impl Request {
    pub fn namespace(&self) -> String {
        self.repo.namespace.clone()
    }

    pub fn name(&self) -> String {
        self.repo.name.clone()
    }
}
