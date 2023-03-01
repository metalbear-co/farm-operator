use std::net::SocketAddr;

use axum::{response::IntoResponse, routing::get, Json, Router};
use axum_server::tls_rustls::RustlsConfig;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::APIResourceList;

async fn get_api_resources() -> impl IntoResponse {
    Json(APIResourceList {
        group_version: "farm.example.com/v1alpha".to_owned(),
        resources: vec![],
    })
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new().route("/apis/farm.example.com/v1alpha", get(get_api_resources));

    // We generate a self-signed certificate for example purposes in a proper service this should be
    // loaded from secret and CA for said cert should be defined in APIService uner `caBundle`
    let tls_cert = rcgen::generate_simple_self_signed(vec!["localhost".to_owned()])?;
    let tls_config = RustlsConfig::from_der(
        vec![tls_cert.serialize_der()?],
        tls_cert.serialize_private_key_der(),
    )
    .await?;

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    println!("listening on {addr}");

    axum_server::bind_rustls(addr, tls_config)
        .serve(app.into_make_service())
        .await
        .map_err(anyhow::Error::from)
}
