use axum::{extract::Path, response::IntoResponse, Json};
use k8s_openapi::api::core::v1::Pod;
use kube::{Api, Client, CustomResource};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    group = "farm.example.com",
    version = "v1alpha",
    kind = "FarmPod",
    namespaced
)]
pub struct FarmPodSpec {
    pub containers: usize,
}

pub async fn list_farmpods(Path(namespace): Path<String>) -> impl IntoResponse {
    let client = Client::try_default().await.expect("Client Creation Error");

    let pods = Api::<Pod>::namespaced(client, &namespace)
        .list(&Default::default())
        .await
        .expect("Failed to fetch pods");

    let items = pods
        .items
        .into_iter()
        .map(|value| {
            let name = value
                .metadata
                .name
                .map(|name| format!("farm-{name}"))
                .unwrap_or_default();

            FarmPod::new(
                &name,
                FarmPodSpec {
                    containers: value
                        .spec
                        .map(|spec| spec.containers.len())
                        .unwrap_or_default(),
                },
            )
        })
        .collect::<Vec<_>>();

    Json(serde_json::json!({
        "apiVersion": "farm.example.com/v1alpha",
        "kind": "FarmPodList",
        "items": items,
        "metadata": pods.metadata
    }))
}
