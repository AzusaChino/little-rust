mod abi;

use std::{
    num::NonZeroUsize,
    sync::{Arc, Mutex},
};

use anyhow::Result;
use axum::{extract::Path, http::StatusCode, routing::get, Extension, Router};
use bytes::Bytes;
use lru::LruCache;
use percent_encoding::percent_decode_str;
use serde::Deserialize;
use tower::ServiceBuilder;
use tower_http::add_extension::AddExtensionLayer;

use crate::pb::abi::ImageSpec;

#[derive(Deserialize)]
struct Params {
    spec: String,
    url: String,
}

type Cache = Arc<Mutex<LruCache<u64, Bytes>>>;

pub async fn start_server() -> Result<()> {
    let cache: Cache = Arc::new(Mutex::new(LruCache::new(
        NonZeroUsize::new(1024).expect("non zero"),
    )));
    let app = Router::new()
        .route("/image/:spec/:url", get(generate))
        .layer(
            ServiceBuilder::new()
                .layer(AddExtensionLayer::new(cache))
                .into_inner(),
        );

    let addr = "127.0.0.1:3001".parse()?;

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

async fn generate(
    Path(Params { spec, url }): Path<Params>,
    Extension(cache): Extension<Cache>,
) -> Result<String, StatusCode> {
    let url = percent_decode_str(&url).decode_utf8_lossy();
    let spec: ImageSpec = spec
        .as_str()
        .try_into()
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    Ok("ok".to_owned())
}
