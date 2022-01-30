use std::net::SocketAddr;

use reqwest::Url;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub addr: SocketAddr,
    pub gitea_url: Url,
    pub config_repo_name: String,
    pub admin_token: String,
}
