use std::{
    sync::Arc,
    task::{Context, Poll},
};

use async_trait::async_trait;
use axum::{
    extract::FromRequestParts,
    http::{
        header::{HeaderName, HeaderValue},
        request::{Parts, Request},
        StatusCode,
    },
};
use kube::{client::ClientBuilder, Client, Config};
use tower::{layer::Layer, Service};

#[derive(Clone)]
pub struct Impersonation<S> {
    inner: S,
    headers: Arc<Vec<(HeaderName, HeaderValue)>>,
}

impl<S, ReqBody> Service<Request<ReqBody>> for Impersonation<S>
where
    S: Service<Request<ReqBody>>,
{
    type Error = S::Error;
    type Future = S::Future;
    type Response = S::Response;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<ReqBody>) -> Self::Future {
        req.headers_mut().extend(self.headers.iter().cloned());
        self.inner.call(req)
    }
}

#[derive(Debug)]
pub struct ImpersonationLayer {
    headers: Arc<Vec<(HeaderName, HeaderValue)>>,
}

impl ImpersonationLayer {
    pub async fn client(&self) -> Result<Client, kube::Error> {
        let config = Config::infer().await.map_err(kube::Error::InferConfig)?;

        Ok(ClientBuilder::try_from(config)?.with_layer(self).build())
    }
}

impl<S> Layer<S> for ImpersonationLayer {
    type Service = Impersonation<S>;

    fn layer(&self, inner: S) -> Self::Service {
        Impersonation {
            inner,
            headers: self.headers.clone(),
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for ImpersonationLayer {
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let mut headers = Vec::new();

        for (header_name, header_value) in &parts.headers {
            if let Some(suffix) = header_name.as_str().strip_prefix("x-remote-") {
                headers.push((
                    HeaderName::from_lowercase(format!("impersonate-{suffix}").as_bytes())
                        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?,
                    header_value.clone(),
                ));
            }
        }

        Ok(ImpersonationLayer {
            headers: Arc::new(headers),
        })
    }
}
