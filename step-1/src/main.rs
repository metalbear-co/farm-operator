use std::net::SocketAddr;

use axum::{response::IntoResponse, routing::get, Json, Router};
use axum_server::tls_rustls::RustlsConfig;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::APIResourceList;
use tower_http::trace::TraceLayer;

async fn get_api_resources() -> impl IntoResponse {
    Json(APIResourceList {
        group_version: "farm.example.com/v1alpha".to_owned(),
        resources: vec![],
    })
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/apis/farm.example.com/v1alpha", get(get_api_resources))
        .layer(TraceLayer::new_for_http());

    let cert = rcgen::generate_simple_self_signed(vec!["localhost".to_owned()])?;

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let config = RustlsConfig::from_der(
        vec![cert.serialize_der()?],
        cert.serialize_private_key_der(),
    )
    .await?;

    println!("listening on {addr}");

    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await
        .map_err(anyhow::Error::from)
}