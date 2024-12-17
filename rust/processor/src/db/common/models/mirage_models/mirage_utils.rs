// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

// This is required because a diesel macro makes clippy sad
#![allow(clippy::extra_unused_lifetimes)]

use crate::{
    db::common::models::rebase::Rebase,
    db::postgres::models::default_models::move_resources::MoveResource,
    utils::util::deserialize_from_string,
};
use anyhow::{Context, Result};
use aptos_protos::transaction::v1::WriteResource;
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MirageResource {
    MirageDebtStore(MirageDebtStore),
    FeeStore(FeeStore),
}

impl MirageResource {
    pub fn is_resource_supported(data_type: &str, mirage_module_address: &str) -> bool {
        [
            format!("{}::mirage::MirageDebtStore", mirage_module_address),
            format!("{}::fee_manager::FeeStore", mirage_module_address),
        ]
        .contains(&data_type.to_string())
    }

    pub fn from_resource(
        data_type: &str,
        data: &serde_json::Value,
        txn_version: i64,
        mirage_module_address: &str
    ) -> Result<Self> {
        match data_type {
            x if x == format!("{}::mirage::MirageDebtStore", mirage_module_address) => {
                serde_json::from_value(data.clone()).map(|inner| Some(Self::MirageDebtStore(inner)))
            },
            x if x == format!("{}::fee_manager::FeeStore", mirage_module_address) => {
                serde_json::from_value(data.clone()).map(|inner| Some(Self::FeeStore(inner)))
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

        if let MirageResource::MirageDebtStore(inner) =
            MirageResource::from_resource(&type_str, resource.data.as_ref().unwrap(), txn_version, mirage_module_address)?
        {
            Ok(Some(inner))
        } else {
            Ok(None)
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FeeStore {
    #[serde(deserialize_with = "deserialize_from_string")]
    pub net_accumulated_fees: BigDecimal,
}

impl FeeStore {
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

        if let MirageResource::FeeStore(inner) =
            MirageResource::from_resource(&type_str, resource.data.as_ref().unwrap(), txn_version, mirage_module_address)?
        {
            Ok(Some(inner))
        } else {
            Ok(None)
        }
    }
}
