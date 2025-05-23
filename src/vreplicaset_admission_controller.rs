#![allow(unused_imports)]

pub mod external_api;
pub mod kubernetes_api_objects;
pub mod kubernetes_cluster;
pub mod reconciler;
pub mod shim_layer;
pub mod state_machine;
pub mod temporal_logic;
#[path = "controller_examples/v_replica_set_controller/mod.rs"]
pub mod v_replica_set_controller;
pub mod vstd_ext;

use crate::v_replica_set_controller::{
    exec::validator::validate_replicas,
    trusted::exec_types::VReplicaSet,
};
use crate::kubernetes_api_objects::exec::dynamic::DynamicObject;
use deps_hack::anyhow::Result;
use deps_hack::kube::CustomResourceExt;
use deps_hack::serde_yaml;
use deps_hack::tokio;
// use deps_hack::tracing::{error, info};
use deps_hack::tracing_subscriber;
use shim_layer::controller_runtime::run_controller;
use std::env;
use deps_hack::kube::core::{
    admission::{AdmissionRequest, AdmissionResponse, AdmissionReview},
    DynamicObject as KubeDynamicObject, ResourceExt,
};
use crate::kubernetes_api_objects::exec::resource::ResourceWrapper;
use deps_hack::tracing::*;
use deps_hack::warp::*;
// use deps_hack::warp::{reply, Filter, Reply};
use std::convert::Infallible;

pub async fn validate_handler(
    body: AdmissionReview<KubeDynamicObject>,
) -> Result<impl Reply, Infallible> {
    let req: AdmissionRequest<_> = match body.try_into() {
        Ok(req) => req,
        Err(err) => {
            error!("invalid request: {}", err.to_string());
            return Ok(reply::json(
                &AdmissionResponse::invalid(err.to_string()).into_review(),
            ));
        }
    };

    let mut res = AdmissionResponse::from(&req);
    if let Some(obj) = req.object {
        let name = obj.name_any();
        let local_obj = DynamicObject::from_kube(obj.clone());

        // Use unmarshal function to convert DynamicObject to VReplicaSet
        let vrs_result = VReplicaSet::unmarshal(local_obj);
        if let Ok(vrs) = vrs_result {
            res = match validate_replicas(&vrs) {
                Ok(()) => {
                    info!("accepted: {:?} on resource {}", req.operation, name);
                    res
                }
                Err(err) => {
                    warn!("denied: {:?} on {} ({})", req.operation, name, err);
                    res.deny(err.to_string())
                }
            };
        } else {
            warn!("Failed to unmarshal VReplicaSet");
            res = res.deny("Failed to unmarshal VReplicaSet".to_string());
        }
        // if vrs_result.is_err() {
        //     res.deny("Failed to unmarshal VReplicaSet".to_string())
        // }

        // // Get the VReplicaSet instance and its spec
        // let vrs = vrs_result.unwrap();
    };
    Ok(reply::json(&res.into_review()))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let routes = path("validate")
        .and(body::json())
        .and_then(validate_handler)
        .with(trace::request());

    serve(post().and(routes))
        .tls()
        .cert_path("/certs/tls.crt")
        .key_path("/certs/tls.key")
        .run(([0, 0, 0, 0], 8443))
        .await;
}
