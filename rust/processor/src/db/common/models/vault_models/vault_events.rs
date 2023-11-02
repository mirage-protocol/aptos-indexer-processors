// Copyright © Mirage Protocol

// This is required because a diesel macro makes clippy sad
#![allow(clippy::extra_unused_lifetimes)]

/**
 * This file defines deserialized vault_events module types as defined in mirage protocol module.
 */
use crate::{
    db::common::models::{token_v2_models::v2_token_utils::ResourceReference},
    utils::util::deserialize_from_string,
};
use anyhow::Context;
use aptos_protos::transaction::v1::Event;
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddCollateralEvent {
    pub collection: ResourceReference,
    pub vault: ResourceReference,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub collateral_amount: BigDecimal,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RemoveCollateralEvent {
    pub collection: ResourceReference,
    pub vault: ResourceReference,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub collateral_amount: BigDecimal,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BorrowEvent {
    pub collection: ResourceReference,
    pub vault: ResourceReference,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub borrow_amount: BigDecimal,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub fee_amount: BigDecimal,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RepayEvent {
    pub collection: ResourceReference,
    pub vault: ResourceReference,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub borrow_amount: BigDecimal,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub fee_amount: BigDecimal,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LiquidationEvent {
    pub collection: ResourceReference,
    pub vault: ResourceReference,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub collateral_amount: BigDecimal,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub borrow_amount: BigDecimal,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub protocol_liquidation_fee: BigDecimal,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub socialized_amount: BigDecimal,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub collateralization_rate_before: BigDecimal,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub collateralization_rate_after: BigDecimal,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InterestRateChangeEvent {
    pub collection: ResourceReference,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub new_interest_per_second: BigDecimal,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum VaultEvent {
    AddCollateralEvent(AddCollateralEvent),
    RemoveCollateralEvent(RemoveCollateralEvent),
    BorrowEvent(BorrowEvent),
    RepayEvent(RepayEvent),
    LiquidationEvent(LiquidationEvent),
    InterestRateChangeEvent(InterestRateChangeEvent),
}

impl VaultEvent {
    pub fn is_event_supported(event_type: &str, mirage_module_address: &str) -> bool {
        [
            format!("{}::vault::AddCollateralEvent", mirage_module_address),
            format!("{}::vault::RemoveCollateralEvent", mirage_module_address),
            format!("{}::vault::BorrowEvent", mirage_module_address),
            format!("{}::vault::RepayEvent", mirage_module_address),
            format!("{}::vault::LiquidationEvent", mirage_module_address),
            format!("{}::vault::InterestRateChangeEvent", mirage_module_address),
        ]
        .contains(&event_type.to_string())
    }

    pub fn from_event(event: &Event, txn_version: i64, mirage_module_address: &str) -> anyhow::Result<Option<Self>> {
        let type_str: String = event.type_str.clone();
        let data = event.data.as_str();

        if !Self::is_event_supported(type_str.as_str(), mirage_module_address) {
            return Ok(None);
        }

        match type_str.clone() {
            x if x == format!("{}::vault::AddCollateralEvent", mirage_module_address) => {
                serde_json::from_str(data).map(|inner| Some(VaultEvent::AddCollateralEvent(inner)))
            },
            x if x == format!("{}::vault::RemoveCollateralEvent", mirage_module_address) => {
                serde_json::from_str(data)
                    .map(|inner| Some(VaultEvent::RemoveCollateralEvent(inner)))
            },
            x if x == format!("{}::vault::BorrowEvent", mirage_module_address) => {
                serde_json::from_str(data).map(|inner| Some(VaultEvent::BorrowEvent(inner)))
            },
            x if x == format!("{}::vault::RepayEvent", mirage_module_address) => {
                serde_json::from_str(data).map(|inner| Some(VaultEvent::RepayEvent(inner)))
            },
            x if x == format!("{}::vault::LiquidationEvent", mirage_module_address) => {
                serde_json::from_str(data).map(|inner| Some(VaultEvent::LiquidationEvent(inner)))
            },
            x if x == format!("{}::vault::InterestRateChangeEvent", mirage_module_address) => {
                serde_json::from_str(data)
                    .map(|inner| Some(VaultEvent::InterestRateChangeEvent(inner)))
            },
            _ => Ok(None),
        }
        .context(format!(
            "version {} failed! failed to parse type {}, data {:?}",
            txn_version,
            type_str.clone(),
            data
        ))
    }
}
