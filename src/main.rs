use std::{path::PathBuf, time::Duration};

use axum::{
    extract::Extension, handler::post, response::IntoResponse, AddExtensionLayer, Json, Router,
};
use config::Config;
use error::Error;
use model::{APIConfig, Request};
use serde_json::Value;
use structopt::StructOpt;

use crate::model::{AuthQuery, Response};

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
    let api_conf = APIConfig(
        config.gitea_url,
        config.config_repo_name,
        config.admin_token,
    );

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
    Extension(APIConfig(base_url, repo_name, token)): Extension<APIConfig>,
) -> Result<impl IntoResponse, Error> {
    let client = reqwest::ClientBuilder::new()
        .user_agent("curl")
        .timeout(Duration::from_secs(30))
        .build()?;
    let auth = AuthQuery {
        access_token: token,
    };
    let index_url = format!(
        "{}/api/v1/repos/{}/{}/raw/index.json",
        base_url.to_string(),
        body.namespace(),
        &repo_name
    );
    let res: Value = client
        .get(index_url)
        .query(&auth)
        .send()
        .await?
        .json()
        .await?;

    if let Value::Object(obj) = res {
        let v = obj.get(&body.name()).ok_or(Error::NoContent)?;
        if let Value::String(path) = v {
            let conf_url = format!(
                "{}/api/v1/repos/{}/{}/raw/{}",
                base_url.to_string(),
                body.namespace(),
                &repo_name,
                path
            );
            let drone_config = client
                .get(conf_url)
                .query(&auth)
                .send()
                .await?
                .text()
                .await?;
            let response = Response { data: drone_config };
            return Ok(Json(response));
        }
    }
    Err(Error::NoContent)
}
