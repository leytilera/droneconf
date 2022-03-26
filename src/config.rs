use std::net::SocketAddr;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub addr: SocketAddr,
    pub proxy: Option<String>,
    pub proxy_auth: Option<ProxyAuth>,
}

#[derive(Deserialize)]
pub struct ProxyAuth {
    pub username: String,
    pub password: String,
}
