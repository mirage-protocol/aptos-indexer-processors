// Copyright © Mirage Protocol

// This is required because a diesel macro makes clippy sad
#![allow(clippy::extra_unused_lifetimes)]

/**
 * This file defines deserialized market_events module types as defined in mirage protocol module.
 */
use crate::{
    db::common::models::{signed64::Signed64, token_v2_models::v2_token_utils::ResourceReference},
    utils::util::deserialize_from_string,
};
use anyhow::{Context, Result};
use aptos_protos::transaction::v1::Event;
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateFundingEvent {
    pub market: ResourceReference,
    pub next_funding_rate: Signed64,
    pub long_funding: Signed64,
    pub short_funding: Signed64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenPositionEvent {
    pub market: ResourceReference,
    pub position: ResourceReference,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub opening_price: BigDecimal,
    pub is_long: bool,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub margin_amount: BigDecimal,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub position_size: BigDecimal,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub fee: BigDecimal,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClosePositionEvent {
    pub market: ResourceReference,
    pub position: ResourceReference,
    pub is_long: bool,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub position_size: BigDecimal,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub closing_price: BigDecimal,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub fee: BigDecimal,
    pub pnl: Signed64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct IncreaseMarginEvent {
    pub market: ResourceReference,
    pub position: ResourceReference,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub margin_amount: BigDecimal,
}

#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct DecreaseMarginEvent {
    pub market: ResourceReference,
    pub position: ResourceReference,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub margin_amount: BigDecimal,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IncreasePositionSizeEvent {
    pub market: ResourceReference,
    pub position: ResourceReference,
    pub is_long: bool,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub amount: BigDecimal,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub new_opening_price: BigDecimal,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub fee: BigDecimal,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DecreasePositionSizeEvent {
    pub market: ResourceReference,
    pub position: ResourceReference,
    pub is_long: bool,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub amount: BigDecimal,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub closing_price: BigDecimal,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub fee: BigDecimal,
}

#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct LiquidatePositionEvent {
    pub market: ResourceReference,
    pub position: ResourceReference,
    pub is_long: bool,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub position_size: BigDecimal,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub closing_price: BigDecimal,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub liquidation_fee: BigDecimal,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub remaining_maintenance_margin: BigDecimal,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub protocol_fee: BigDecimal,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub closing_fee: BigDecimal,
    pub winnings: Signed64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlaceTpslEvent {
    pub market: ResourceReference,
    pub position: ResourceReference,
    pub tpsl: ResourceReference,
    pub is_long: bool,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub take_profit_price: BigDecimal,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub stop_loss_price: BigDecimal,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateTpslEvent {
    pub market: ResourceReference,
    pub position: ResourceReference,
    pub tpsl: ResourceReference,
    pub is_long: bool,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub take_profit_price: BigDecimal,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub stop_loss_price: BigDecimal,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CancelTpslEvent {
    pub market: ResourceReference,
    pub position: ResourceReference,
    pub tpsl: ResourceReference,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TriggerTpslEvent {
    pub market: ResourceReference,
    pub position: ResourceReference,
    pub tpsl: ResourceReference,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlaceLimitOrderEvent {
    pub market: ResourceReference,
    pub position: ResourceReference,
    pub limit_order: ResourceReference,
    pub is_decrease_only: bool,
    pub is_long: bool,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub position_size: BigDecimal,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub margin_amount: BigDecimal,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub trigger_price: BigDecimal,
    pub triggers_above: bool,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub max_price_slippage: BigDecimal,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub expiration: BigDecimal,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateLimitOrderEvent {
    pub market: ResourceReference,
    pub position: ResourceReference,
    pub limit_order: ResourceReference,
    pub is_decrease_only: bool,
    pub is_long: bool,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub position_size: BigDecimal,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub margin_amount: BigDecimal,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub trigger_price: BigDecimal,
    pub triggers_above: bool,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub max_price_slippage: BigDecimal,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub expiration: BigDecimal,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SettlePnlEvent {
    pub market: ResourceReference,
    pub position: ResourceReference,
    pub pnl: Signed64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CancelLimitOrderEvent {
    pub market: ResourceReference,
    pub position: ResourceReference,
    pub limit_order: ResourceReference,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TriggerLimitOrderEvent {
    pub market: ResourceReference,
    pub position: ResourceReference,
    pub limit_order: ResourceReference,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MarketEvent {
    UpdateFundingEvent(UpdateFundingEvent),
    OpenPositionEvent(OpenPositionEvent),
    ClosePositionEvent(ClosePositionEvent),
    IncreaseMarginEvent(IncreaseMarginEvent),
    DecreaseMarginEvent(DecreaseMarginEvent),
    IncreasePositionSizeEvent(IncreasePositionSizeEvent),
    DecreasePositionSizeEvent(DecreasePositionSizeEvent),
    LiquidatePositionEvent(LiquidatePositionEvent),
    PlaceTpslEvent(PlaceTpslEvent),
    UpdateTpslEvent(UpdateTpslEvent),
    CancelTpslEvent(CancelTpslEvent),
    TriggerTpslEvent(TriggerTpslEvent),
    PlaceLimitOrderEvent(PlaceLimitOrderEvent),
    UpdateLimitOrderEvent(UpdateLimitOrderEvent),
    CancelLimitOrderEvent(CancelLimitOrderEvent),
    TriggerLimitOrderEvent(TriggerLimitOrderEvent),
    SettlePnlEvent(SettlePnlEvent),
}

impl MarketEvent {
    pub fn is_event_supported(event_type: &str, market_module_address: &str) -> bool {
        [
            format!("{}::market::UpdateFundingEvent", market_module_address),
            format!("{}::market::OpenPositionEvent", market_module_address),
            format!("{}::market::ClosePositionEvent", market_module_address),
            format!("{}::market::IncreaseMarginEvent", market_module_address),
            format!("{}::market::DecreaseMarginEvent", market_module_address),
            format!(
                "{}::market::IncreasePositionSizeEvent",
                market_module_address
            ),
            format!(
                "{}::market::DecreasePositionSizeEvent",
                market_module_address
            ),
            format!("{}::market::LiquidatePositionEvent", market_module_address),
            format!("{}::market::SettlePnlEvent", market_module_address),
            format!("{}::tpsl::PlaceTpslEvent", market_module_address),
            format!("{}::tpsl::UpdateTpslEvent", market_module_address),
            format!("{}::tpsl::CancelTpslEvent", market_module_address),
            format!("{}::tpsl::TriggerTpslEvent", market_module_address),
            format!(
                "{}::limit_order::PlaceLimitOrderEvent",
                market_module_address
            ),
            format!(
                "{}::limit_order::UpdateLimitOrderEvent",
                market_module_address
            ),
            format!(
                "{}::limit_order::CancelLimitOrderEvent",
                market_module_address
            ),
            format!(
                "{}::limit_order::TriggerLimitOrderEvent",
                market_module_address
            ),
        ]
        .contains(&event_type.to_string())
    }

    pub fn from_event(
        event: &Event,
        txn_version: i64,
        market_module_address: &str,
    ) -> Result<Option<Self>> {
        let type_str: String = event.type_str.clone();
        let data = event.data.as_str();

        if !Self::is_event_supported(type_str.as_str(), market_module_address) {
            return Ok(None);
        }

        match type_str.clone() {
            x if x == format!("{}::market::UpdateFundingEvent", market_module_address) => {
                serde_json::from_str(data).map(|inner| Some(MarketEvent::UpdateFundingEvent(inner)))
            },
            x if x == format!("{}::market::OpenPositionEvent", market_module_address) => {
                serde_json::from_str(data).map(|inner| Some(MarketEvent::OpenPositionEvent(inner)))
            },
            x if x == format!("{}::market::ClosePositionEvent", market_module_address) => {
                serde_json::from_str(data).map(|inner| Some(MarketEvent::ClosePositionEvent(inner)))
            },
            x if x == format!("{}::market::IncreaseMarginEvent", market_module_address) => {
                serde_json::from_str(data)
                    .map(|inner| Some(MarketEvent::IncreaseMarginEvent(inner)))
            },
            x if x == format!("{}::market::DecreaseMarginEvent", market_module_address) => {
                serde_json::from_str(data)
                    .map(|inner| Some(MarketEvent::DecreaseMarginEvent(inner)))
            },
            x if x
                == format!(
                    "{}::market::IncreasePositionSizeEvent",
                    market_module_address
                ) =>
            {
                serde_json::from_str(data)
                    .map(|inner| Some(MarketEvent::IncreasePositionSizeEvent(inner)))
            },
            x if x
                == format!(
                    "{}::market::DecreasePositionSizeEvent",
                    market_module_address
                ) =>
            {
                serde_json::from_str(data)
                    .map(|inner| Some(MarketEvent::DecreasePositionSizeEvent(inner)))
            },
            x if x == format!("{}::market::LiquidatePositionEvent", market_module_address) => {
                serde_json::from_str(data)
                    .map(|inner| Some(MarketEvent::LiquidatePositionEvent(inner)))
            },
            x if x == format!("{}::market::SettlePnlEvent", market_module_address) => {
                serde_json::from_str(data).map(|inner| Some(MarketEvent::SettlePnlEvent(inner)))
            },
            x if x == format!("{}::tpsl::PlaceTpslEvent", market_module_address) => {
                serde_json::from_str(data).map(|inner| Some(MarketEvent::PlaceTpslEvent(inner)))
            },
            x if x == format!("{}::tpsl::UpdateTpslEvent", market_module_address) => {
                serde_json::from_str(data).map(|inner| Some(MarketEvent::UpdateTpslEvent(inner)))
            },
            x if x == format!("{}::tpsl::CancelTpslEvent", market_module_address) => {
                serde_json::from_str(data).map(|inner| Some(MarketEvent::CancelTpslEvent(inner)))
            },
            x if x == format!("{}::tpsl::TriggerTpslEvent", market_module_address) => {
                serde_json::from_str(data).map(|inner| Some(MarketEvent::TriggerTpslEvent(inner)))
            },
            x if x
                == format!(
                    "{}::limit_order::PlaceLimitOrderEvent",
                    market_module_address
                ) =>
            {
                serde_json::from_str(data)
                    .map(|inner| Some(MarketEvent::PlaceLimitOrderEvent(inner)))
            },
            x if x
                == format!(
                    "{}::limit_order::UpdateLimitOrderEvent",
                    market_module_address
                ) =>
            {
                serde_json::from_str(data)
                    .map(|inner| Some(MarketEvent::UpdateLimitOrderEvent(inner)))
            },
            x if x
                == format!(
                    "{}::limit_order::CancelLimitOrderEvent",
                    market_module_address
                ) =>
            {
                serde_json::from_str(data)
                    .map(|inner| Some(MarketEvent::CancelLimitOrderEvent(inner)))
            },
            x if x
                == format!(
                    "{}::limit_order::TriggerLimitOrderEvent",
                    market_module_address
                ) =>
            {
                serde_json::from_str(data)
                    .map(|inner| Some(MarketEvent::TriggerLimitOrderEvent(inner)))
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
