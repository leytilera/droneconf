use std::{path::PathBuf, time::Duration};

use axum::{
    extract::Extension, handler::post, response::IntoResponse, AddExtensionLayer, Json, Router,
};
use config::Config;
use error::Error;
use model::{APIConfig, Request};
use reqwest::Proxy;
use structopt::StructOpt;

use crate::model::Response;

mod config;
mod error;
mod model;

#[derive(StructOpt)]
struct Opt {
    #[structopt(
        short,
        long,
        help = "config file to use",
        default_value = "./config.toml"
    )]
    config: PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let opt = Opt::from_args();
    let config = std::fs::read(&opt.config)?;
    let config = toml::from_slice::<Config>(&config)?;
    let mut builder = reqwest::ClientBuilder::new()
        .user_agent("curl")
        .timeout(Duration::from_secs(30));
    if let Some(px) = config.proxy {
        let mut proxy = Proxy::all(px)?;
        if let Some(auth) = config.proxy_auth {
            proxy = proxy.basic_auth(auth.username.as_str(), auth.password.as_str());
        }
        builder = builder.proxy(proxy);
    }
    let client = builder.build()?;
    let api_conf = APIConfig(client);

    let app = Router::new()
        .route("/", post(on_request))
        .layer(AddExtensionLayer::new(api_conf));

    axum::Server::bind(&config.addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn on_request(
    Json(body): Json<Request>,
    Extension(APIConfig(client)): Extension<APIConfig>,
) -> Result<impl IntoResponse, Error> {
    let conf = body.config().ok_or(Error::NoContent)?;
    if conf.starts_with("http://") || conf.starts_with("https://") {
        let drone_config = client.get(conf).send().await?.text().await?;
        let response = Response { data: drone_config };
        return Ok(Json(response));
    }
    Err(Error::NoContent)
}
