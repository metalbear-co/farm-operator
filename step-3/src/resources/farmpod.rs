use axum::{extract::Path, response::IntoResponse, Json};
use k8s_openapi::api::core::v1::Pod;
use kube::Api;

use crate::impersonation::ImpersonationLayer;

pub async fn list_farmpods(
    Path(namespace): Path<String>,
    impersonation: ImpersonationLayer,
) -> impl IntoResponse {
    let client = impersonation.client().await.expect("Client Creation Error");

    let pods = Api::<Pod>::namespaced(client, &namespace)
        .list(&Default::default())
        .await
        .expect("Falied to fetch pods");

    let items = pods
        .items
        .into_iter()
        .map(|mut value| {
            value.metadata.name = value.metadata.name.map(|name| format!("farm-{name}"));
            value
        })
        .collect::<Vec<_>>();

    Json(serde_json::json!({
        "apiVersion": "farm.example.com/v1alpha",
        "kind": "FarmPodList",
        "items": items,
        "metadata": pods.metadata
    }))
}
