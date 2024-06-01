// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

// This is required because a diesel macro makes clippy sad
#![allow(clippy::extra_unused_lifetimes)]

use crate::{
    models::{
        default_models::move_resources::MoveResource, signed64::Signed64,
        token_v2_models::v2_token_utils::ResourceReference,
    },
    utils::util::deserialize_from_string,
};
use anyhow::{Context, Result};
use aptos_protos::transaction::v1::WriteResource;
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FeeInfo {
    #[serde(deserialize_with = "deserialize_from_string")]
    pub min_taker_fee: BigDecimal,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub max_taker_fee: BigDecimal,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub min_maker_fee: BigDecimal,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub max_maker_fee: BigDecimal,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub liquidation_fee: BigDecimal,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub referrer_fee: BigDecimal,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FundingInfo {
    #[serde(deserialize_with = "deserialize_from_string")]
    pub min_funding_rate: BigDecimal,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub max_funding_rate: BigDecimal,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub base_funding_rate: BigDecimal,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub funding_interval: BigDecimal,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MarketConfig {
    pub fees: FeeInfo,
    pub funding: FundingInfo,

    #[serde(deserialize_with = "deserialize_from_string")]
    pub max_oi: BigDecimal,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub max_oi_imbalance: BigDecimal,

    #[serde(deserialize_with = "deserialize_from_string")]
    pub maintenance_margin: BigDecimal,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub max_leverage: BigDecimal,

    #[serde(deserialize_with = "deserialize_from_string")]
    pub min_order_size: BigDecimal,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub max_order_size: BigDecimal,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MarketCollection {
    pub margin_token: ResourceReference,
    pub perp_symbol: String,
    pub margin_oracle: ResourceReference,
    pub perp_oracle: ResourceReference,

    #[serde(deserialize_with = "deserialize_from_string")]
    pub total_long_margin: BigDecimal,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub total_short_margin: BigDecimal,

    #[serde(deserialize_with = "deserialize_from_string")]
    pub long_oi: BigDecimal,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub short_oi: BigDecimal,

    pub long_funding_accumulated_per_unit: Signed64,
    pub short_funding_accumulated_per_unit: Signed64,
    pub total_long_funding_accumulated: Signed64,
    pub total_short_funding_accumulated: Signed64,

    pub next_funding_rate: Signed64,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub last_funding_round: BigDecimal,

    pub is_long_close_only: bool,
    pub is_short_close_only: bool,

    pub config: MarketConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Position {
    pub market: ResourceReference,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub opening_price: BigDecimal,
    pub is_long: bool,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub margin_amount: BigDecimal,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub position_size: BigDecimal,
    pub last_funding_accumulated: Signed64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TpSl {
    pub market: ResourceReference,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub take_profit_price: BigDecimal,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub stop_loss_price: BigDecimal,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub trigger_payment_amount: BigDecimal,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LimitOrder {
    #[serde(deserialize_with = "deserialize_from_string")]
    pub id: BigDecimal,
    pub is_increase: bool,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub position_size: BigDecimal,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub margin_amount: BigDecimal,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub trigger_price: BigDecimal,
    pub triggers_above: bool,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub trigger_payment_amount: BigDecimal,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub max_price_slippage: BigDecimal,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub expiration: BigDecimal,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LimitOrders {
    pub market: ResourceReference,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub next_id: BigDecimal,
    pub orders: Vec<LimitOrder>,
}

impl MarketCollection {
    /// Fungible asset is part of an object and we need to get the object first to get owner address
    pub fn from_write_resource(
        write_resource: &WriteResource,
        txn_version: i64,
        mirage_module_address: &str,
    ) -> anyhow::Result<Option<Self>> {
        let type_str = MoveResource::get_outer_type_from_write_resource(write_resource);
        if !MarketResource::is_resource_supported(type_str.as_str(), mirage_module_address) {
            return Ok(None);
        }
        let resource = MoveResource::from_write_resource(
            write_resource,
            0, // Placeholder, this isn't used anyway
            txn_version,
            0, // Placeholder, this isn't used anyway
        );

        if let MarketResource::MarketCollection(inner) =
            MarketResource::from_resource(&type_str, resource.data.as_ref().unwrap(), txn_version, mirage_module_address)?
        {
            Ok(Some(inner))
        } else {
            Ok(None)
        }
    }
}

impl Position {
    /// Fungible asset is part of an object and we need to get the object first to get owner address
    pub fn from_write_resource(
        write_resource: &WriteResource,
        txn_version: i64,
        mirage_module_address: &str,
    ) -> anyhow::Result<Option<Self>> {
        let type_str = MoveResource::get_outer_type_from_write_resource(write_resource);
        if !MarketResource::is_resource_supported(type_str.as_str(), mirage_module_address) {
            return Ok(None);
        }
        let resource = MoveResource::from_write_resource(
            write_resource,
            0, // Placeholder, this isn't used anyway
            txn_version,
            0, // Placeholder, this isn't used anyway
        );

        if let MarketResource::Position(inner) =
            MarketResource::from_resource(&type_str, resource.data.as_ref().unwrap(), txn_version, mirage_module_address)?
        {
            Ok(Some(inner))
        } else {
            Ok(None)
        }
    }
}

impl LimitOrders {
    /// Fungible asset is part of an object and we need to get the object first to get owner address
    pub fn from_write_resource(
        write_resource: &WriteResource,
        txn_version: i64,
        mirage_module_address: &str,
    ) -> anyhow::Result<Option<Self>> {
        let type_str = MoveResource::get_outer_type_from_write_resource(write_resource);
        if !MarketResource::is_resource_supported(type_str.as_str(), mirage_module_address) {
            return Ok(None);
        }
        let resource = MoveResource::from_write_resource(
            write_resource,
            0, // Placeholder, this isn't used anyway
            txn_version,
            0, // Placeholder, this isn't used anyway
        );

        if let MarketResource::LimitOrders(inner) =
            MarketResource::from_resource(&type_str, resource.data.as_ref().unwrap(), txn_version, mirage_module_address)?
        {
            Ok(Some(inner))
        } else {
            Ok(None)
        }
    }
}

impl TpSl {
    /// Fungible asset is part of an object and we need to get the object first to get owner address
    pub fn from_write_resource(
        write_resource: &WriteResource,
        txn_version: i64,
        mirage_module_address: &str
    ) -> anyhow::Result<Option<Self>> {
        let type_str = MoveResource::get_outer_type_from_write_resource(write_resource);
        if !MarketResource::is_resource_supported(type_str.as_str(), mirage_module_address) {
            return Ok(None);
        }
        let resource = MoveResource::from_write_resource(
            write_resource,
            0, // Placeholder, this isn't used anyway
            txn_version,
            0, // Placeholder, this isn't used anyway
        );

        if let MarketResource::TpSl(inner) =
            MarketResource::from_resource(&type_str, resource.data.as_ref().unwrap(), txn_version, mirage_module_address)?
        {
            Ok(Some(inner))
        } else {
            Ok(None)
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MarketResource {
    MarketCollection(MarketCollection),
    Position(Position),
    TpSl(TpSl),
    LimitOrders(LimitOrders),
}

impl MarketResource {
    pub fn is_resource_supported(data_type: &str, mirage_module_address: &str) -> bool {
        [
            format!("{}::market::Market", mirage_module_address),
            format!("{}::market::Position", mirage_module_address),
            format!("{}::market::TpSl", mirage_module_address),
            format!("{}::market::LimitOrders", mirage_module_address),
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
            x if x == format!("{}::market::Market", mirage_module_address) => {
                serde_json::from_value(data.clone())
                    .map(|inner| Some(Self::MarketCollection(inner)))
            },
            x if x == format!("{}::market::Position", mirage_module_address) => {
                serde_json::from_value(data.clone()).map(|inner| Some(Self::Position(inner)))
            },
            x if x == format!("{}::market::TpSl", mirage_module_address) => {
                serde_json::from_value(data.clone()).map(|inner| Some(Self::TpSl(inner)))
            },
            x if x == format!("{}::market::LimitOrders", mirage_module_address) => {
                serde_json::from_value(data.clone()).map(|inner| Some(Self::LimitOrders(inner)))
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
