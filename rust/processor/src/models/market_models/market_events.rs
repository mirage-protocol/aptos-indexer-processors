// Copyright © Mirage Protocol

// This is required because a diesel macro makes clippy sad
#![allow(clippy::extra_unused_lifetimes)]

/**
 * This file defines deserialized market_events module types as defined in mirage protocol module.
 */
use crate::{
    models::{signed64::Signed64, token_v2_models::v2_token_utils::ResourceReference},
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
    pub winnings: Signed64,
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
    #[serde(deserialize_with = "deserialize_from_string")]
    pub take_profit_price: BigDecimal,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub stop_loss_price: BigDecimal,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub trigger_payment_amount: BigDecimal,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateTpslEvent {
    pub market: ResourceReference,
    pub position: ResourceReference,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub take_profit_price: BigDecimal,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub stop_loss_price: BigDecimal,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CancelTpslEvent {
    pub market: ResourceReference,
    pub position: ResourceReference,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IncreaseTpslTriggerPaymentEvent {
    pub market: ResourceReference,
    pub position: ResourceReference,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub increase_amount: BigDecimal,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DecreaseTpslTriggerPaymentEvent {
    pub market: ResourceReference,
    pub position: ResourceReference,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub decrease_amount: BigDecimal,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TriggerTpslEvent {
    pub market: ResourceReference,
    pub position: ResourceReference,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub trigger_payment: BigDecimal,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlaceLimitOrderEvent {
    pub market: ResourceReference,
    pub position: ResourceReference,
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
pub struct UpdateLimitOrderEvent {
    pub market: ResourceReference,
    pub position: ResourceReference,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub id: BigDecimal,
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
pub struct CancelLimitOrderEvent {
    pub market: ResourceReference,
    pub position: ResourceReference,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub id: BigDecimal,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IncreaseLimitOrderTriggerPaymentEvent {
    pub market: ResourceReference,
    pub position: ResourceReference,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub id: BigDecimal,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub increase_amount: BigDecimal,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DecreaseLimitOrderTriggerPaymentEvent {
    pub market: ResourceReference,
    pub position: ResourceReference,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub id: BigDecimal,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub decrease_amount: BigDecimal,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TriggerLimitOrderEvent {
    pub market: ResourceReference,
    pub position: ResourceReference,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub id: BigDecimal,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub trigger_payment_amount: BigDecimal,
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
    IncreaseTpslTriggerPaymentEvent(IncreaseTpslTriggerPaymentEvent),
    DecreaseTpslTriggerPaymentEvent(DecreaseTpslTriggerPaymentEvent),
    TriggerTpslEvent(TriggerTpslEvent),
    PlaceLimitOrderEvent(PlaceLimitOrderEvent),
    UpdateLimitOrderEvent(UpdateLimitOrderEvent),
    CancelLimitOrderEvent(CancelLimitOrderEvent),
    IncreaseLimitOrderTriggerPaymentEvent(IncreaseLimitOrderTriggerPaymentEvent),
    DecreaseLimitOrderTriggerPaymentEvent(DecreaseLimitOrderTriggerPaymentEvent),
    TriggerLimitOrderEvent(TriggerLimitOrderEvent),
}

impl MarketEvent {
    pub fn is_event_supported(event_type: &str, mirage_module_address: &str) -> bool {
        [
            format!("{}::market::UpdateFundingEvent", mirage_module_address),
            format!("{}::market::OpenPositionEvent", mirage_module_address),
            format!("{}::market::ClosePositionEvent", mirage_module_address),
            format!("{}::market::IncreaseMarginEvent", mirage_module_address),
            format!("{}::market::DecreaseMarginEvent", mirage_module_address),
            format!("{}::market::IncreasePositionSizeEvent", mirage_module_address),
            format!("{}::market::DecreasePositionSizeEvent", mirage_module_address),
            format!("{}::market::LiquidatePositionEvent", mirage_module_address),
            format!("{}::market::PlaceTpslEvent", mirage_module_address),
            format!("{}::market::UpdateTpslEvent", mirage_module_address),
            format!("{}::market::CancelTpslEvent", mirage_module_address),
            format!("{}::market::IncreaseTpslTriggerPaymentEvent", mirage_module_address),
            format!("{}::market::DecreaseTpslTriggerPaymentEvent", mirage_module_address),
            format!("{}::market::TriggerTpslEvent", mirage_module_address),
            format!("{}::market::PlaceLimitOrderEvent", mirage_module_address),
            format!("{}::market::UpdateLimitOrderEvent", mirage_module_address),
            format!("{}::market::CancelLimitOrderEvent", mirage_module_address),
            format!(
                "{}::market::IncreaseLimitOrderTriggerPaymentEvent",
                mirage_module_address
            ),
            format!(
                "{}::market::DecreaseLimitOrderTriggerPaymentEvent",
                mirage_module_address
            ),
            format!("{}::market::TriggerLimitOrderEvent", mirage_module_address),
        ]
        .contains(&event_type.to_string())
    }

    pub fn from_event(event: &Event, txn_version: i64, mirage_module_address: &str) -> Result<Option<Self>> {
        let type_str: String = event.type_str.clone();
        let data = event.data.as_str();

        if !Self::is_event_supported(type_str.as_str(), mirage_module_address) {
            return Ok(None);
        }

        match type_str.clone() {
            x if x == format!("{}::market::UpdateFundingEvent", mirage_module_address) => {
                serde_json::from_str(data).map(|inner| Some(MarketEvent::UpdateFundingEvent(inner)))
            },
            x if x == format!("{}::market::OpenPositionEvent", mirage_module_address) => {
                serde_json::from_str(data).map(|inner| Some(MarketEvent::OpenPositionEvent(inner)))
            },
            x if x == format!("{}::market::ClosePositionEvent", mirage_module_address) => {
                serde_json::from_str(data).map(|inner| Some(MarketEvent::ClosePositionEvent(inner)))
            },
            x if x == format!("{}::market::IncreaseMarginEvent", mirage_module_address) => {
                serde_json::from_str(data)
                    .map(|inner| Some(MarketEvent::IncreaseMarginEvent(inner)))
            },
            x if x == format!("{}::market::DecreaseMarginEvent", mirage_module_address) => {
                serde_json::from_str(data)
                    .map(|inner| Some(MarketEvent::DecreaseMarginEvent(inner)))
            },
            x if x == format!("{}::market::IncreasePositionSizeEvent", mirage_module_address) => {
                serde_json::from_str(data)
                    .map(|inner| Some(MarketEvent::IncreasePositionSizeEvent(inner)))
            },
            x if x == format!("{}::market::DecreasePositionSizeEvent", mirage_module_address) => {
                serde_json::from_str(data)
                    .map(|inner| Some(MarketEvent::DecreasePositionSizeEvent(inner)))
            },
            x if x == format!("{}::market::LiquidatePositionEvent", mirage_module_address) => {
                serde_json::from_str(data)
                    .map(|inner| Some(MarketEvent::LiquidatePositionEvent(inner)))
            },
            x if x == format!("{}::market::PlaceTpslEvent", mirage_module_address) => {
                serde_json::from_str(data).map(|inner| Some(MarketEvent::PlaceTpslEvent(inner)))
            },
            x if x == format!("{}::market::UpdateTpslEvent", mirage_module_address) => {
                serde_json::from_str(data).map(|inner| Some(MarketEvent::UpdateTpslEvent(inner)))
            },
            x if x == format!("{}::market::CancelTpslEvent", mirage_module_address) => {
                serde_json::from_str(data).map(|inner| Some(MarketEvent::CancelTpslEvent(inner)))
            },
            x if x == format!("{}::market::IncreaseTpslTriggerPaymentEvent", mirage_module_address) => {
                serde_json::from_str(data)
                    .map(|inner| Some(MarketEvent::IncreaseTpslTriggerPaymentEvent(inner)))
            },
            x if x == format!("{}::market::DecreaseTpslTriggerPaymentEvent", mirage_module_address) => {
                serde_json::from_str(data)
                    .map(|inner| Some(MarketEvent::DecreaseTpslTriggerPaymentEvent(inner)))
            },
            x if x == format!("{}::market::TriggerTpslEvent", mirage_module_address) => {
                serde_json::from_str(data).map(|inner| Some(MarketEvent::TriggerTpslEvent(inner)))
            },
            x if x == format!("{}::market::PlaceLimitOrderEvent", mirage_module_address) => {
                serde_json::from_str(data)
                    .map(|inner| Some(MarketEvent::PlaceLimitOrderEvent(inner)))
            },
            x if x == format!("{}::market::UpdateLimitOrderEvent", mirage_module_address) => {
                serde_json::from_str(data)
                    .map(|inner| Some(MarketEvent::UpdateLimitOrderEvent(inner)))
            },
            x if x == format!("{}::market::CancelLimitOrderEvent", mirage_module_address) => {
                serde_json::from_str(data)
                    .map(|inner| Some(MarketEvent::CancelLimitOrderEvent(inner)))
            },
            x if x
                == format!(
                    "{}::market::IncreaseLimitOrderTriggerPaymentEvent",
                    mirage_module_address
                ) =>
            {
                serde_json::from_str(data)
                    .map(|inner| Some(MarketEvent::IncreaseLimitOrderTriggerPaymentEvent(inner)))
            },
            x if x
                == format!(
                    "{}::market::DecreaseLimitOrderTriggerPaymentEvent",
                    mirage_module_address
                ) =>
            {
                serde_json::from_str(data)
                    .map(|inner| Some(MarketEvent::DecreaseLimitOrderTriggerPaymentEvent(inner)))
            },
            x if x == format!("{}::market::TriggerLimitOrderEvent", mirage_module_address) => {
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
