// Copyright © Mirage Protocol

use crate::{
    models::{
        default_models::move_resources::MoveResource,
        rebase::{Base, Rebase},
        token_v2_models::v2_token_utils::ResourceReference,
    },
    utils::util::deserialize_from_string,
};
use anyhow::{Context, Result};
use aptos_protos::transaction::v1::WriteResource;
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VaultCollectionConfig {
    #[serde(deserialize_with = "deserialize_from_string")]
    pub interest_per_second: BigDecimal,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub initial_collateralization_rate: BigDecimal,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub maintenance_collateralization_rate: BigDecimal,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub liquidation_multiplier: BigDecimal,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub borrow_fee: BigDecimal,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub protocol_liquidation_fee: BigDecimal,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VaultCollection {
    pub collateral_token: ResourceReference,
    pub borrow_token: ResourceReference,
    pub collateral_oracle: ResourceReference,
    pub borrow_oracle: ResourceReference,

    #[serde(deserialize_with = "deserialize_from_string")]
    pub total_collateral: BigDecimal,
    pub borrow: Rebase,
    pub global_debt_part: Base,
    pub config: VaultCollectionConfig,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub last_interest_payment: BigDecimal,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub cached_exchange_rate: BigDecimal,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub last_interest_update: BigDecimal,
    pub is_emergency: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Vault {
    pub collection: ResourceReference,
    pub collateral: ResourceReference,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub collateral_amount: BigDecimal,
    pub borrow_part: Base,
}

impl VaultCollection {
    /// Fungible asset is part of an object and we need to get the object first to get owner address
    pub fn from_write_resource(
        write_resource: &WriteResource,
        txn_version: i64,
        mirage_module_address: &str,
    ) -> anyhow::Result<Option<Self>> {
        let type_str = MoveResource::get_outer_type_from_write_resource(write_resource);
        if !VaultModuleResource::is_resource_supported(type_str.as_str(), mirage_module_address) {
            return Ok(None);
        }
        let resource = MoveResource::from_write_resource(
            write_resource,
            0, // Placeholder, this isn't used anyway
            txn_version,
            0, // Placeholder, this isn't used anyway
        );

        if let VaultModuleResource::VaultCollection(inner) = VaultModuleResource::from_resource(
            &type_str,
            resource.data.as_ref().unwrap(),
            txn_version,
            mirage_module_address,
        )? {
            Ok(Some(inner))
        } else {
            Ok(None)
        }
    }
}

impl Vault {
    /// Fungible asset is part of an object and we need to get the object first to get owner address
    pub fn from_write_resource(
        write_resource: &WriteResource,
        txn_version: i64,
        mirage_module_address: &str,
    ) -> anyhow::Result<Option<Self>> {
        let type_str = MoveResource::get_outer_type_from_write_resource(write_resource);
        if !VaultModuleResource::is_resource_supported(type_str.as_str(), mirage_module_address) {
            return Ok(None);
        }
        let resource = MoveResource::from_write_resource(
            write_resource,
            0, // Placeholder, this isn't used anyway
            txn_version,
            0, // Placeholder, this isn't used anyway
        );

        if let VaultModuleResource::Vault(inner) = VaultModuleResource::from_resource(
            &type_str,
            resource.data.as_ref().unwrap(),
            txn_version,
            mirage_module_address,
        )? {
            Ok(Some(inner))
        } else {
            Ok(None)
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum VaultModuleResource {
    VaultCollection(VaultCollection),
    Vault(Vault),
}

impl VaultModuleResource {
    pub fn is_resource_supported(data_type: &str, mirage_module_address: &str) -> bool {
        [
            format!("{}::vault::VaultCollection", mirage_module_address),
            format!("{}::vault::Vault", mirage_module_address),
        ]
        .contains(&data_type.to_string())
    }

    pub fn from_resource(
        data_type: &str,
        data: &serde_json::Value,
        txn_version: i64,
        mirage_module_address: &str,
    ) -> Result<Self> {
        match data_type {
            x if x == format!("{}::vault::VaultCollection", mirage_module_address) => {
                serde_json::from_value(data.clone()).map(|inner| Some(Self::VaultCollection(inner)))
            },
            x if x == format!("{}::vault::Vault", mirage_module_address) => {
                serde_json::from_value(data.clone()).map(|inner| Some(Self::Vault(inner)))
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
