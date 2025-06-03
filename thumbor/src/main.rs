mod engine;
mod pb;
use engine::{Engine, ImageEngine};

use axum::extract::Path;
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::routing::get;
use axum::{Extension, Router};
use bytes::Bytes;
use image::ImageFormat;
use lru::LruCache;
use pb::*;
use percent_encoding::{NON_ALPHANUMERIC, percent_decode_str, percent_encode};
use serde::Deserialize;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::num::NonZeroUsize;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower::ServiceBuilder;
use tower_http::add_extension::AddExtensionLayer;
use tracing::{info, instrument};

#[derive(Deserialize)]
struct Params {
    spec: String,
    url: String,
}
type Cache = Arc<Mutex<LruCache<u64, Bytes>>>;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let cache: Cache = Arc::new(Mutex::new(LruCache::<u64, Bytes>::new(
        NonZeroUsize::new(1024).unwrap(),
    )));

    let app = Router::new()
        .route("/image/{spec}/{url}", get(generate))
        .layer(
            ServiceBuilder::new()
                .layer(AddExtensionLayer::new(cache))
                .into_inner(),
        );

    let addr = "127.0.0.1:3000".parse::<String>().unwrap();

    print_test_url(
        "https://images.pexels.com/photos/1562477/pexels-photo-1562477.jpeg?auto=compress&cs=tinysrgb&dpr=3&h=750&w=1260",
    );

    info!("listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn generate(
    Path(Params { spec, url }): Path<Params>,
    Extension(cache): Extension<Cache>,
) -> Result<(HeaderMap, Vec<u8>), StatusCode> {
    let spec: ImageSpec = spec
        .as_str()
        .try_into()
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let url = percent_decode_str(&url).decode_utf8_lossy();
    let data = retrieve_image(&url, cache)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    info!("spec {:#?}", spec);

    let mut engine: ImageEngine = data
        .try_into()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    engine.apply(&spec.specs);

    let image = engine.generate(ImageFormat::Png);
    info!("Finished processing: image size {}", image.len());

    let mut headers = HeaderMap::new();
    headers.insert("content-type", HeaderValue::from_static("image/png"));
    Ok((headers, image))
}

#[instrument(level = "info", skip(cache))]
async fn retrieve_image(url: &str, cache: Cache) -> anyhow::Result<Bytes> {
    let mut hasher = DefaultHasher::new();
    url.hash(&mut hasher);
    let key = hasher.finish();

    let g = &mut cache.lock().await;
    let data = match g.get(&key) {
        Some(v) => {
            info!("Match cache {}", key);
            v.to_owned()
        }
        None => {
            info!("Retrieve url: {}", url);
            let resp = reqwest::get(url).await?;
            let data = resp.bytes().await?;
            g.put(key, data.clone());
            data
        }
    };
    Ok(data)
}

fn print_test_url(url: &str) {
    use std::borrow::Borrow;
    let spec1 = Spec::new_resize(500, 800, resize::SampleFilter::CatmullRom);
    let spec2 = Spec::new_watermark(20, 20);
    let spec3 = Spec::new_filter(filter::Filter::Marine);
    let spec4 = Spec::new_flip_v();
    let image_spec = ImageSpec::new(vec![spec1, spec2, spec3, spec4]);
    let s: String = image_spec.borrow().into();
    let test_image = percent_encode(url.as_bytes(), NON_ALPHANUMERIC).to_string();
    println!("test url: http://localhost:3000/image/{}/{}", s, test_image);
}
