use std::{collections::HashMap, sync::LazyLock};

use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use chrono::Utc;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ListMeta;
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

type Lamas = HashMap<String, HashMap<String, Lama>>;

static STATIC_LAMAS: LazyLock<Lamas> = LazyLock::new(|| {
    serde_json::from_value(serde_json::json!({
        "default": {
            "dolly": {
                "metadata": {
                    "name": "dolly",
                    "namespace": "default",
                    "creationTimestamp": Utc::now(),
                },
                "spec": {
                    "height": 0.5,
                    "weight": 31.4
                }
            }
        }
    }))
    .expect("Could not create static lamas")
});

#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    group = "farm.example.com",
    version = "v1alpha",
    kind = "Lama",
    namespaced
)]
pub struct LamaSpec {
    pub weight: f32,
    pub height: f32,
}

pub async fn list_lamas(Path(namespace): Path<String>) -> impl IntoResponse {
    Json(serde_json::json!({
        "apiVersion": "farm.example.com/v1alpha",
        "kind": "LamaList",
        "items": &STATIC_LAMAS.get(&namespace).map(|lamas| lamas.values().collect::<Vec<_>>()).unwrap_or_default(),
        "metadata": ListMeta::default()
    }))
}

pub async fn get_lama(Path((namespace, name)): Path<(String, String)>) -> Response {
    if let Some(lama) = STATIC_LAMAS
        .get(&namespace)
        .and_then(|lamas| lamas.get(&name))
    {
        Json(lama).into_response()
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}
