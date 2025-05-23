#![allow(unused_imports)]
#![allow(unused_variables)]
pub mod common;
pub mod fluent_e2e;
pub mod rabbitmq_e2e;
pub mod v2_vreplicaset_e2e;
pub mod v2_vdeployment_e2e;
pub mod zookeeper_e2e;
pub mod v2_vstatefulset_admission_e2e;
pub mod v2_vreplicaset_admission_e2e;
pub mod v2_vdeployment_admission_e2e;

use common::Error;
use fluent_e2e::fluent_e2e_test;
use rabbitmq_e2e::{rabbitmq_e2e_test, rabbitmq_ephemeral_e2e_test, rabbitmq_scaling_e2e_test};
use std::str::FromStr;
use std::{env, sync::Arc};
use tracing::*;
use v2_vreplicaset_e2e::v2_vreplicaset_e2e_test;
use v2_vreplicaset_admission_e2e::v2_vreplicaset_admission_e2e_test;
use v2_vstatefulset_admission_e2e::v2_vstatefulset_admission_e2e_test;
use v2_vdeployment_admission_e2e::v2_vdeployment_admission_e2e_test;
use v2_vdeployment_e2e::v2_vdeployment_e2e_test;
use zookeeper_e2e::{zookeeper_e2e_test, zookeeper_ephemeral_e2e_test, zookeeper_scaling_e2e_test};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    let args: Vec<String> = env::args().collect();
    let cmd = args[1].clone();
    match cmd.as_str() {
        "zookeeper" => {
            info!("Running zookeeper end-to-end test");
            return zookeeper_e2e_test().await;
        }
        "zookeeper-scaling" => {
            info!("Running zookeeper end-to-end test for scaling");
            return zookeeper_scaling_e2e_test().await;
        }
        "zookeeper-ephemeral" => {
            info!("Running zookeeper end-to-end test for ephemeral storage");
            return zookeeper_ephemeral_e2e_test().await;
        }
        "rabbitmq" => {
            info!("Running rabbitmq end-to-end test");
            return rabbitmq_e2e_test().await;
        }
        "rabbitmq-scaling" => {
            info!("Running rabbitmq end-to-end test for scaling");
            return rabbitmq_scaling_e2e_test().await;
        }
        "rabbitmq-ephemeral" => {
            info!("Running rabbitmq end-to-end test for ephemeral storage");
            return rabbitmq_ephemeral_e2e_test().await;
        }
        "fluent" => {
            info!("Running fluent end-to-end test");
            return fluent_e2e_test().await;
        }
        "v2-vreplicaset" => {
            info!("Running v2-vreplicaset end-to-end test");
            return v2_vreplicaset_e2e_test().await;
        }
        "v2-vreplicaset-admission" => {
            info!("Running v2-vreplicaset-admission end-to-end test");
            return v2_vreplicaset_admission_e2e_test().await;
        }
        "v2-vstatefulset-admission" => {
            info!("Running v2-vstatefulset-admission end-to-end test");
            return v2_vstatefulset_admission_e2e_test().await;
        }
        "v2-vdeployment-admission" => {
            info!("Running v2-vdeployment-admission end-to-end test");
            return v2_vdeployment_admission_e2e_test().await;
        }
        "v2-vdeployment" => {
            info!("Running v2-vdeployment end-to-end test");
            return v2_vdeployment_e2e_test().await;
        }
        _ => {
            error!("Wrong command. Please specify the correct e2e test workload.");
            Ok(())
        }
    }
}
