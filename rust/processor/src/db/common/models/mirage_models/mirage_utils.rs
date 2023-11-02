// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

// This is required because a diesel macro makes clippy sad
#![allow(clippy::extra_unused_lifetimes)]

use crate::{
    db::common::models::{rate_limiter::RateLimiter, rebase::Rebase},
    db::common::models::default_models::move_resources::MoveResource,
};
use anyhow::{Context, Result};
use aptos_protos::transaction::v1::WriteResource;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MirageResource {
    MirageDebtStore(MirageDebtStore),
}

impl MirageResource {
    pub fn is_resource_supported(data_type: &str, mirage_module_address: &str) -> bool {
        [format!(
            "{}::mirage::MirageDebtStore",
            mirage_module_address
        )]
        .contains(&data_type.to_string())
    }

    pub fn from_resource(
        data_type: &str,
        data: &serde_json::Value,
        txn_version: i64,
        mirage_module_address: &str,
    ) -> Result<Self> {
        match data_type {
            x if x == format!("{}::mirage::MirageDebtStore", mirage_module_address) => {
                serde_json::from_value(data.clone()).map(|inner| Some(Self::MirageDebtStore(inner)))
            },
            _ => Ok(None),
        }
        .context(format!(
            "version {} failed! failed to parse type {}, data {:?}",
            txn_version, data_type, data
        ))?
        .context(format!(
            "Resource unsupported! Call is_resource_supported first. version {} type {}",
            txn_version, data_type
        ))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MirageDebtStore {
    pub debt: Rebase,
    pub burn_rate_limiter: RateLimiter,
    pub mint_rate_limiter: RateLimiter,
}

impl MirageDebtStore {
    pub fn from_write_resource(
        write_resource: &WriteResource,
        txn_version: i64,
        mirage_module_address: &str,
    ) -> anyhow::Result<Option<Self>> {
        let type_str = MoveResource::get_outer_type_from_write_resource(write_resource);
        if !MirageResource::is_resource_supported(type_str.as_str(), mirage_module_address) {
            return Ok(None);
        }
        let resource = MoveResource::from_write_resource(
            write_resource,
            0, // Placeholder, this isn't used anyway
            txn_version,
            0, // Placeholder, this isn't used anyway
        );

        match MirageResource::from_resource(
            &type_str,
            resource.data.as_ref().unwrap(),
            txn_version,
            mirage_module_address,
        )? {
            MirageResource::MirageDebtStore(inner) => Ok(Some(inner)),
        }
    }
}
