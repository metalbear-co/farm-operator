#![feature(once_cell)]

use std::net::SocketAddr;

use axum::{response::IntoResponse, routing::get, Json, Router};
use axum_server::tls_rustls::RustlsConfig;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::{APIResource, APIResourceList};
use kube::Resource as _;

use crate::resources::{farmpod, llama};

mod resources;

async fn get_api_resources() -> impl IntoResponse {
    Json(APIResourceList {
        group_version: "farm.example.com/v1alpha".to_string(),
        resources: vec![
            APIResource {
                group: Some(llama::Llama::group(&()).into()),
                kind: llama::Llama::kind(&()).into(),
                name: llama::Llama::plural(&()).into(),
                namespaced: true,
                verbs: vec!["list".to_string(), "get".to_string()],
                ..Default::default()
            },
            APIResource {
                group: Some(farmpod::FarmPod::group(&()).into()),
                kind: farmpod::FarmPod::kind(&()).into(),
                name: farmpod::FarmPod::plural(&()).into(),
                namespaced: true,
                verbs: vec!["list".to_string()],
                ..Default::default()
            },
        ],
    })
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new()
        .route("/apis/farm.example.com/v1alpha", get(get_api_resources))
        .route(
            "/apis/farm.example.com/v1alpha/namespaces/:namespace/llamas",
            get(llama::list_llamas),
        )
        .route(
            "/apis/farm.example.com/v1alpha/namespaces/:namespace/llamas/:name",
            get(llama::get_llama),
        )
        .route(
            "/apis/farm.example.com/v1alpha/namespaces/:namespace/farmpods",
            get(farmpod::list_farmpods),
        );

    // We generate a self-signed certificate for example purposes in a proper service this should be
    // loaded from secret and CA for said cert should be defined in APIService uner `caBundle`
    let tls_cert = rcgen::generate_simple_self_signed(vec!["localhost".to_string()])?;
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
