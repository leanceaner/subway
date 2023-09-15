use jsonrpsee::{
    core::{JsonValue, StringError},
    types::ErrorObjectOwned,
    PendingSubscriptionSink,
};

use crate::{
    config::{RpcMethod, RpcSubscription},
    middleware::{Middleware, MiddlewareBuilder},
    utils::TypeRegistryRef,
};

pub mod methods;
pub mod subscriptions;

#[derive(Debug)]
pub struct CallRequest {
    pub method: String,
    pub params: Vec<JsonValue>,
}

impl CallRequest {
    pub fn new(method: impl ToString, params: Vec<JsonValue>) -> Self {
        Self {
            method: method.to_string(),
            params,
        }
    }
}

pub type CallResult = Result<JsonValue, ErrorObjectOwned>;

pub async fn create_method_middleware(
    name: &str,
    method: &RpcMethod,
    extensions: &TypeRegistryRef,
) -> Option<Box<dyn Middleware<CallRequest, CallResult>>> {
    use methods::*;

    match name {
        "response" => response::ResponseMiddleware::build(method, extensions).await,
        "upstream" => upstream::UpstreamMiddleware::build(method, extensions).await,
        "cache" => cache::CacheMiddleware::build(method, extensions).await,
        "block_tag" => block_tag::BlockTagMiddleware::build(method, extensions).await,
        "inject_params" => inject_params::InjectParamsMiddleware::build(method, extensions).await,
        _ => panic!("Unknown method middleware: {}", name),
    }
}

#[derive(Debug)]
pub struct SubscriptionRequest {
    pub subscribe: String,
    pub params: Vec<JsonValue>,
    pub unsubscribe: String,
    pub sink: PendingSubscriptionSink,
}

pub type SubscriptionResult = Result<(), StringError>;

pub async fn create_subscription_middleware(
    name: &str,
    method: &RpcSubscription,
    extensions: &TypeRegistryRef,
) -> Option<Box<dyn Middleware<SubscriptionRequest, SubscriptionResult>>> {
    use subscriptions::*;

    match name {
        "upstream" => upstream::UpstreamMiddleware::build(method, extensions).await,
        "merge_subscription" => {
            merge_subscription::MergeSubscriptionMiddleware::build(method, extensions).await
        }
        _ => panic!("Unknown subscription middleware: {}", name),
    }
}