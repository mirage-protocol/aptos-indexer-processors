// Copyright © Mirage Protocol

// This is required because a diesel macro makes clippy sad
#![allow(clippy::extra_unused_lifetimes)]
#![allow(clippy::unused_unit)]

use super::market_events::MarketEvent;
use crate::{
    models::object_models::v2_object_utils::ObjectAggregatedDataMapping,
    schema::{closed_limit_orders, market_activities, open_positions, closed_positions, open_tpsls, closed_tpsls, open_limit_orders, trade_datas},
};
use aptos_protos::transaction::v1::{
    transaction::TxnData, Event as EventPB, Transaction as TransactionPB,
};
use bigdecimal::{BigDecimal, Zero};
use chrono::NaiveDateTime;
use field_count::FieldCount;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, FieldCount, Identifiable, Insertable, Serialize)]
#[diesel(primary_key(transaction_version, event_index, event_sequence_number,))]
#[diesel(table_name = market_activities)]
pub struct MarketActivityModel {
    pub transaction_version: i64,
    pub event_creation_number: i64,
    pub event_sequence_number: i64,
    pub event_index: i64,

    pub market_id: String,
    pub event_type: String,

    pub position_id: Option<String>,
    pub id: Option<BigDecimal>,
    pub owner_addr: Option<String>,

    pub perp_price: Option<BigDecimal>,
    pub is_long: Option<bool>,
    pub margin_amount: Option<BigDecimal>,
    pub position_size: Option<BigDecimal>,
    pub fee: Option<BigDecimal>,
    pub pnl: Option<BigDecimal>,
    pub take_profit_price: Option<BigDecimal>,
    pub stop_loss_price: Option<BigDecimal>,
    pub trigger_price: Option<BigDecimal>,
    pub max_price_slippage: Option<BigDecimal>,
    pub is_increase: Option<bool>,
    pub triggers_above: Option<bool>,
    pub expiration: Option<BigDecimal>,
    pub trigger_payment_amount: Option<BigDecimal>,
    pub next_funding_rate: Option<BigDecimal>,
    pub transaction_timestamp: chrono::NaiveDateTime,
}

/// A simplified MarketActivity (excluded common fields) to reduce code duplication
struct MarketActivityHelper {
    pub market_id: String,
    pub event_type: String,

    pub position_id: Option<String>,
    pub id: Option<BigDecimal>,
    pub owner_addr: Option<String>,

    pub perp_price: Option<BigDecimal>,
    pub is_long: Option<bool>,
    pub margin_amount: Option<BigDecimal>,
    pub position_size: Option<BigDecimal>,
    pub fee: Option<BigDecimal>,
    pub pnl: Option<BigDecimal>,
    pub take_profit_price: Option<BigDecimal>,
    pub stop_loss_price: Option<BigDecimal>,
    pub trigger_price: Option<BigDecimal>,
    pub max_price_slippage: Option<BigDecimal>,
    pub is_increase: Option<bool>,
    pub triggers_above: Option<bool>,
    pub expiration: Option<BigDecimal>,
    pub trigger_payment_amount: Option<BigDecimal>,
    pub next_funding_rate: Option<BigDecimal>,
}

#[derive(Clone, Debug, Deserialize, FieldCount, Identifiable, Insertable, Serialize)]
#[diesel(primary_key(transaction_version, position_id))]
#[diesel(table_name = trade_datas)]
pub struct Trade {
    pub transaction_version: i64,

    pub market_id: String,
    pub position_id: String,
    pub owner_addr: String,

    pub is_long: bool,
    pub position_size: BigDecimal,
    pub price: BigDecimal,
    pub fee: BigDecimal,
    pub pnl: BigDecimal,

    pub transaction_timestamp: chrono::NaiveDateTime,
}

#[derive(Clone, Debug, Deserialize, FieldCount, Identifiable, Insertable, Serialize)]
#[diesel(primary_key(position_id))]
#[diesel(table_name = open_positions)]
pub struct OpenPosition {
    pub last_transaction_version: i64,

    pub market_id: String,
    pub position_id: String,

    pub transaction_timestamp: chrono::NaiveDateTime,
}

#[derive(Clone, Debug, Deserialize, FieldCount, Identifiable, Insertable, Serialize)]
#[diesel(primary_key(position_id))]
#[diesel(table_name = closed_positions)]
pub struct ClosedPosition {
    pub transaction_version: i64,

    pub market_id: String,
    pub position_id: String,

    pub transaction_timestamp: chrono::NaiveDateTime,
}

#[derive(Clone, Debug, Deserialize, FieldCount, Identifiable, Insertable, Serialize)]
#[diesel(primary_key(position_id))]
#[diesel(table_name = open_tpsls)]
pub struct OpenTpsl {
    pub last_transaction_version: i64,

    pub market_id: String,
    pub position_id: String,

    pub transaction_timestamp: chrono::NaiveDateTime,
}

#[derive(Clone, Debug, Deserialize, FieldCount, Identifiable, Insertable, Serialize)]
#[diesel(primary_key(position_id))]
#[diesel(table_name = closed_tpsls)]
pub struct ClosedTpsl {
    pub transaction_version: i64,

    pub market_id: String,
    pub position_id: String,

    pub transaction_timestamp: chrono::NaiveDateTime,
}

#[derive(Clone, Debug, Deserialize, FieldCount, Identifiable, Insertable, Serialize)]
#[diesel(primary_key(limit_order_id))]
#[diesel(table_name = open_limit_orders)]
pub struct OpenLimitOrder {
    pub last_transaction_version: i64,

    pub market_id: String,
    pub position_id: String,
    pub limit_order_id: BigDecimal,

    pub transaction_timestamp: chrono::NaiveDateTime,
}

#[derive(Clone, Debug, Deserialize, FieldCount, Identifiable, Insertable, Serialize)]
#[diesel(primary_key(limit_order_id))]
#[diesel(table_name = closed_limit_orders)]
pub struct ClosedLimitOrder {
    pub transaction_version: i64,

    pub market_id: String,
    pub position_id: String,
    pub limit_order_id: BigDecimal,

    pub transaction_timestamp: chrono::NaiveDateTime,
}

impl MarketActivityModel {
    pub fn from_transaction(
        transaction: &TransactionPB,
        object_metadatas: &ObjectAggregatedDataMapping,
        mirage_module_address: &str,
    ) -> (
        Vec<Trade>,
        Vec<OpenPosition>,
        Vec<ClosedPosition>,
        Vec<OpenTpsl>,
        Vec<ClosedTpsl>,
        Vec<OpenLimitOrder>,
        Vec<ClosedLimitOrder>,
        Vec<MarketActivityModel>,
    ) {
        let mut trades: Vec<Trade> = Vec::new();
        let mut open_positions: Vec<OpenPosition> = Vec::new();
        let mut closed_positions: Vec<ClosedPosition> = Vec::new();
        let mut open_tpsls: Vec<OpenTpsl> = Vec::new();
        let mut closed_tpsls: Vec<ClosedTpsl> = Vec::new();
        let mut open_limit_orders: Vec<OpenLimitOrder> = Vec::new();
        let mut closed_limit_orders: Vec<ClosedLimitOrder> = Vec::new();
        let mut market_activities: Vec<MarketActivityModel> = Vec::new();

        // Extracts events and user request from genesis and user transactions. Other transactions won't have coin events
        let txn_data = transaction
            .txn_data
            .as_ref()
            .expect("Txn Data doesn't exit!");
        let events = match txn_data {
            TxnData::Genesis(inner) => &inner.events,
            TxnData::User(inner) => &inner.events,
            _ => return Default::default(),
        };

        // The rest are fields common to all transactions
        let txn_version = transaction.version as i64;
        let txn_timestamp = transaction
            .timestamp
            .as_ref()
            .expect("Transaction timestamp doesn't exist!")
            .seconds;
        let txn_timestamp =
            NaiveDateTime::from_timestamp_opt(txn_timestamp, 0).expect("Txn Timestamp is invalid!");

        for (index, event) in events.iter().enumerate() {
            let maybe_market_event = MarketEvent::from_event(event, txn_version, mirage_module_address).unwrap();

            if let Some(market_event) = maybe_market_event {
                let (
                    market_activity,
                    maybe_trade,
                    maybe_open_position,
                    maybe_closed_position,
                    maybe_open_tpsl,
                    maybe_closed_tpsl,
                    maybe_open_limit_order,
                    maybe_closed_limit_order,
                ) = Self::from_parsed_event(
                    event,
                    &market_event,
                    txn_version,
                    txn_timestamp,
                    index as i64,
                    object_metadatas,
                );
                market_activities.push(market_activity);

                if let Some(inner) = maybe_trade {
                    trades.push(inner);
                }

                if let Some(inner) = maybe_open_position {
                    open_positions.push(inner);
                }
                if let Some(inner) = maybe_closed_position {
                    closed_positions.push(inner);
                }

                if let Some(inner) = maybe_open_tpsl {
                    open_tpsls.push(inner);
                }
                if let Some(inner) = maybe_closed_tpsl {
                    closed_tpsls.push(inner);
                }

                if let Some(inner) = maybe_open_limit_order {
                    open_limit_orders.push(inner);
                }

                if let Some(inner) = maybe_closed_limit_order {
                    closed_limit_orders.push(inner);
                }
            }
        }

        // LimitOrder::from_write_resource, returns a vector of every limit order in that users account.
        // We need to filter for new/modified limit orders.
        // let filtered_maybe_limit_orders =
        //     if let Some(orders) = maybe_limit_orders {
        //         Some(orders.into_iter()
        //             .filter(|maybe_order| {
        //                 open_limit_orders.iter().any(|open_order| open_order.id == maybe_order.id)
        //             })
        //             .collect::<Vec<LimitOrder>>()
        //         ) } else {
        //             // If maybe_limit_orders is None, we simply pass along None
        //             None
        //         };
        // if let Some(inner) = filtered_maybe_limit_orders {
        //     limit_orders.extend(inner);
        // }

        (
            trades,
            open_positions,
            closed_positions,
            open_tpsls,
            closed_tpsls,
            open_limit_orders,
            closed_limit_orders,
            market_activities,
        )
    }

    fn from_parsed_event(
        event: &EventPB,
        parsed_event: &MarketEvent,
        txn_version: i64,
        txn_timestamp: chrono::NaiveDateTime,
        event_index: i64,
        object_metadatas: &ObjectAggregatedDataMapping,
    ) -> (
        MarketActivityModel,
        Option<Trade>,
        Option<OpenPosition>,
        Option<ClosedPosition>,
        Option<OpenTpsl>,
        Option<ClosedTpsl>,
        Option<OpenLimitOrder>,
        Option<ClosedLimitOrder>,
    ) {
        let event_creation_number = event.key.as_ref().unwrap().creation_number as i64;
        let event_sequence_number = event.sequence_number as i64;

        let mut trade = None;

        let mut open_position = None;
        let mut closed_position = None;

        let mut open_tpsl = None;
        let mut closed_tpsl = None;

        let mut open_limit_order = None;
        let mut closed_limit_order = None;

        let market_activity_helper = match parsed_event {
            MarketEvent::UpdateFundingEvent(inner) => MarketActivityHelper {
                event_type: String::from("UpdateFundingEvent"),
                market_id: inner.market.get_reference_address(),
                position_id: None,
                id: None,
                owner_addr: None,
                perp_price: None,
                is_long: None,
                margin_amount: None,
                position_size: None,
                fee: None,
                pnl: None,
                take_profit_price: None,
                stop_loss_price: None,
                trigger_price: None,
                max_price_slippage: None,
                is_increase: None,
                triggers_above: None,
                expiration: None,
                trigger_payment_amount: None,
                next_funding_rate: Some(inner.next_funding_rate.to_bigdecimal()),
            },
            MarketEvent::OpenPositionEvent(inner) => {
                let owner_addr = if let Some(object_metadata) =
                    object_metadatas.get(&inner.position.get_reference_address())
                {
                    let owner_addr = object_metadata.object.object_core.get_owner_address();
                    trade = Some(Trade {
                        transaction_version: txn_version,
                        market_id: inner.market.get_reference_address(),
                        position_id: inner.position.get_reference_address(),
                        owner_addr: owner_addr.clone(),
                        is_long: inner.is_long,
                        position_size: inner.position_size.clone(),
                        price: inner.opening_price.clone(),
                        fee: inner.fee.clone(),
                        pnl: BigDecimal::zero(),
                        transaction_timestamp: txn_timestamp,
                    });
                    Some(owner_addr)
                } else {
                    None
                };

                open_position = Some(OpenPosition {
                    last_transaction_version: txn_version,
                    market_id: inner.market.get_reference_address(),
                    position_id: inner.position.get_reference_address(),
                    transaction_timestamp: txn_timestamp,
                });
                MarketActivityHelper {
                    event_type: String::from("OpenPositionEvent"),
                    market_id: inner.market.get_reference_address(),
                    position_id: Some(inner.position.get_reference_address()),
                    id: None,
                    owner_addr,
                    perp_price: Some(inner.opening_price.clone()),
                    is_long: Some(inner.is_long),
                    margin_amount: Some(inner.margin_amount.clone()),
                    position_size: Some(inner.position_size.clone()),
                    fee: Some(inner.fee.clone()),
                    pnl: None,
                    take_profit_price: None,
                    stop_loss_price: None,
                    trigger_price: None,
                    max_price_slippage: None,
                    is_increase: Some(true),
                    triggers_above: None,
                    expiration: None,
                    trigger_payment_amount: None,
                    next_funding_rate: None,
                }
            },
            MarketEvent::ClosePositionEvent(inner) => {
                let owner_addr = if let Some(object_metadata) =
                    object_metadatas.get(&inner.position.get_reference_address())
                {
                    let owner_addr = object_metadata.object.object_core.get_owner_address();
                    trade = Some(Trade {
                        transaction_version: txn_version,
                        market_id: inner.market.get_reference_address(),
                        position_id: inner.position.get_reference_address(),
                        owner_addr: owner_addr.clone(),
                        is_long: inner.is_long,
                        position_size: inner.position_size.clone(),
                        price: inner.closing_price.clone(),
                        fee: inner.fee.clone(),
                        pnl: inner.winnings.to_bigdecimal(),
                        transaction_timestamp: txn_timestamp,
                    });
                    Some(owner_addr)
                } else {
                    None
                };

                closed_position = Some(ClosedPosition {
                    transaction_version: txn_version,
                    market_id: inner.market.get_reference_address(),
                    position_id: inner.position.get_reference_address(),
                    transaction_timestamp: txn_timestamp,
                });
                MarketActivityHelper {
                    event_type: String::from("ClosePositionEvent"),
                    market_id: inner.market.get_reference_address(),
                    position_id: Some(inner.position.get_reference_address()),
                    id: None,
                    owner_addr,
                    perp_price: Some(inner.closing_price.clone()),
                    is_long: Some(inner.is_long),
                    margin_amount: None,
                    position_size: Some(inner.position_size.clone()),
                    fee: Some(inner.fee.clone()),
                    pnl: Some(inner.winnings.to_bigdecimal()),
                    take_profit_price: None,
                    stop_loss_price: None,
                    trigger_price: None,
                    max_price_slippage: None,
                    is_increase: Some(false),
                    triggers_above: None,
                    expiration: None,
                    trigger_payment_amount: None,
                    next_funding_rate: None,
                }
            },
            MarketEvent::IncreaseMarginEvent(inner) => {
                let owner_addr = if let Some(object_metadata) =
                    object_metadatas.get(&inner.position.get_reference_address())
                {
                    let owner_addr = object_metadata.object.object_core.get_owner_address();
                    Some(owner_addr)
                } else {
                    None
                };

                open_position = Some(OpenPosition {
                    last_transaction_version: txn_version,
                    market_id: inner.market.get_reference_address(),
                    position_id: inner.position.get_reference_address(),
                    transaction_timestamp: txn_timestamp,
                });
                MarketActivityHelper {
                    event_type: String::from("IncreaseMarginEvent"),
                    market_id: inner.market.get_reference_address(),
                    position_id: Some(inner.position.get_reference_address()),
                    id: None,
                    owner_addr,
                    perp_price: None,
                    is_long: None,
                    margin_amount: Some(inner.margin_amount.clone()),
                    position_size: None,
                    fee: None,
                    pnl: None,
                    take_profit_price: None,
                    stop_loss_price: None,
                    trigger_price: None,
                    max_price_slippage: None,
                    is_increase: None,
                    triggers_above: None,
                    expiration: None,
                    trigger_payment_amount: None,
                    next_funding_rate: None,
                }
            },
            MarketEvent::DecreaseMarginEvent(inner) => {
                let owner_addr = if let Some(object_metadata) =
                    object_metadatas.get(&inner.position.get_reference_address())
                {
                    let owner_addr = object_metadata.object.object_core.get_owner_address();
                    Some(owner_addr)
                } else {
                    None
                };

                open_position = Some(OpenPosition {
                    last_transaction_version: txn_version,
                    market_id: inner.market.get_reference_address(),
                    position_id: inner.position.get_reference_address(),
                    transaction_timestamp: txn_timestamp,
                });
                MarketActivityHelper {
                    event_type: String::from("DecreaseMarginEvent"),
                    market_id: inner.market.get_reference_address(),
                    position_id: Some(inner.position.get_reference_address()),
                    id: None,
                    owner_addr,
                    perp_price: None,
                    is_long: None,
                    margin_amount: Some(inner.margin_amount.clone()),
                    position_size: None,
                    fee: None,
                    pnl: None,
                    take_profit_price: None,
                    stop_loss_price: None,
                    trigger_price: None,
                    max_price_slippage: None,
                    is_increase: None,
                    triggers_above: None,
                    expiration: None,
                    trigger_payment_amount: None,
                    next_funding_rate: None,
                }
            },
            MarketEvent::IncreasePositionSizeEvent(inner) => {
                let owner_addr = if let Some(object_metadata) =
                    object_metadatas.get(&inner.position.get_reference_address())
                {
                    let owner_addr = object_metadata.object.object_core.get_owner_address();
                    trade = Some(Trade {
                        transaction_version: txn_version,
                        market_id: inner.market.get_reference_address(),
                        position_id: inner.position.get_reference_address(),
                        owner_addr: owner_addr.clone(),
                        is_long: inner.is_long,
                        position_size: inner.amount.clone(),
                        price: inner.new_opening_price.clone(),
                        fee: inner.fee.clone(),
                        pnl: BigDecimal::zero(),
                        transaction_timestamp: txn_timestamp,
                    });
                    Some(owner_addr)
                } else {
                    None
                };

                open_position = Some(OpenPosition {
                    last_transaction_version: txn_version,
                    market_id: inner.market.get_reference_address(),
                    position_id: inner.position.get_reference_address(),
                    transaction_timestamp: txn_timestamp,
                });
                MarketActivityHelper {
                    event_type: String::from("IncreasePositionSizeEvent"),
                    market_id: inner.market.get_reference_address(),
                    position_id: Some(inner.position.get_reference_address()),
                    id: None,
                    owner_addr,
                    perp_price: Some(inner.new_opening_price.clone()),
                    is_long: None,
                    margin_amount: None,
                    position_size: Some(inner.amount.clone()),
                    fee: Some(inner.fee.clone()),
                    pnl: None,
                    take_profit_price: None,
                    stop_loss_price: None,
                    trigger_price: None,
                    max_price_slippage: None,
                    is_increase: None,
                    triggers_above: None,
                    expiration: None,
                    trigger_payment_amount: None,
                    next_funding_rate: None,
                }
            },
            MarketEvent::DecreasePositionSizeEvent(inner) => {
                let owner_addr = if let Some(object_metadata) =
                    object_metadatas.get(&inner.position.get_reference_address())
                {
                    let owner_addr = object_metadata.object.object_core.get_owner_address();
                    trade = Some(Trade {
                        transaction_version: txn_version,
                        market_id: inner.market.get_reference_address(),
                        position_id: inner.position.get_reference_address(),
                        owner_addr: owner_addr.clone(),
                        is_long: inner.is_long,
                        position_size: inner.amount.clone(),
                        price: inner.new_opening_price.clone(),
                        fee: inner.fee.clone(),
                        pnl: BigDecimal::zero(),
                        transaction_timestamp: txn_timestamp,
                    });
                    Some(owner_addr)
                } else {
                    None
                };

                open_position = Some(OpenPosition {
                    last_transaction_version: txn_version,
                    market_id: inner.market.get_reference_address(),
                    position_id: inner.position.get_reference_address(),
                    transaction_timestamp: txn_timestamp,
                });
                MarketActivityHelper {
                    event_type: String::from("DecreasePositionSizeEvent"),
                    market_id: inner.market.get_reference_address(),
                    position_id: Some(inner.position.get_reference_address()),
                    id: None,
                    owner_addr,
                    perp_price: Some(inner.new_opening_price.clone()),
                    is_long: None,
                    margin_amount: None,
                    position_size: Some(inner.amount.clone()),
                    fee: Some(inner.fee.clone()),
                    pnl: None,
                    take_profit_price: None,
                    stop_loss_price: None,
                    trigger_price: None,
                    max_price_slippage: None,
                    is_increase: None,
                    triggers_above: None,
                    expiration: None,
                    trigger_payment_amount: None,
                    next_funding_rate: None,
                }
            },
            MarketEvent::LiquidatePositionEvent(inner) => {
                let owner_addr = if let Some(object_metadata) =
                    object_metadatas.get(&inner.position.get_reference_address())
                {
                    let owner_addr = object_metadata.object.object_core.get_owner_address();
                    Some(owner_addr)
                } else {
                    None
                };

                // LiquidatePosition generates a close position event
                MarketActivityHelper {
                    event_type: String::from("LiquidatePositionEvent"),
                    market_id: inner.market.get_reference_address(),
                    position_id: Some(inner.position.get_reference_address()),
                    id: None,
                    owner_addr,
                    perp_price: None,
                    is_long: None,
                    margin_amount: None,
                    position_size: None,
                    fee: None,
                    pnl: None,
                    take_profit_price: None,
                    stop_loss_price: None,
                    trigger_price: None,
                    max_price_slippage: None,
                    is_increase: None,
                    triggers_above: None,
                    expiration: None,
                    trigger_payment_amount: None,
                    next_funding_rate: None,
                }
            },
            MarketEvent::PlaceTpslEvent(inner) => {
                let owner_addr = if let Some(object_metadata) =
                    object_metadatas.get(&inner.position.get_reference_address())
                {
                    let owner_addr = object_metadata.object.object_core.get_owner_address();
                    Some(owner_addr)
                } else {
                    None
                };

                open_tpsl = Some(OpenTpsl {
                    last_transaction_version: txn_version,
                    market_id: inner.market.get_reference_address(),
                    position_id: inner.position.get_reference_address(),
                    transaction_timestamp: txn_timestamp,
                });
                MarketActivityHelper {
                    event_type: String::from("PlaceTpslEvent"),
                    market_id: inner.market.get_reference_address(),
                    position_id: Some(inner.position.get_reference_address()),
                    id: None,
                    owner_addr,
                    perp_price: None,
                    is_long: None,
                    margin_amount: None,
                    position_size: None,
                    fee: None,
                    pnl: None,
                    take_profit_price: Some(inner.take_profit_price.clone()),
                    stop_loss_price: Some(inner.stop_loss_price.clone()),
                    trigger_price: None,
                    max_price_slippage: None,
                    is_increase: None,
                    triggers_above: None,
                    expiration: None,
                    trigger_payment_amount: Some(inner.trigger_payment_amount.clone()),
                    next_funding_rate: None,
                }
            },
            MarketEvent::UpdateTpslEvent(inner) => {
                let owner_addr = if let Some(object_metadata) =
                    object_metadatas.get(&inner.position.get_reference_address())
                {
                    let owner_addr = object_metadata.object.object_core.get_owner_address();
                    Some(owner_addr)
                } else {
                    None
                };

                open_tpsl = Some(OpenTpsl {
                    last_transaction_version: txn_version,
                    market_id: inner.market.get_reference_address(),
                    position_id: inner.position.get_reference_address(),
                    transaction_timestamp: txn_timestamp,
                });
                MarketActivityHelper {
                    event_type: String::from("UpdateTpslEvent"),
                    market_id: inner.market.get_reference_address(),
                    position_id: Some(inner.position.get_reference_address()),
                    id: None,
                    owner_addr,
                    perp_price: None,
                    is_long: None,
                    margin_amount: None,
                    position_size: None,
                    fee: None,
                    pnl: None,
                    take_profit_price: Some(inner.take_profit_price.clone()),
                    stop_loss_price: Some(inner.stop_loss_price.clone()),
                    trigger_price: None,
                    max_price_slippage: None,
                    is_increase: None,
                    triggers_above: None,
                    expiration: None,
                    trigger_payment_amount: None,
                    next_funding_rate: None,
                }
            },
            MarketEvent::CancelTpslEvent(inner) => {
                let owner_addr = if let Some(object_metadata) =
                    object_metadatas.get(&inner.position.get_reference_address())
                {
                    let owner_addr = object_metadata.object.object_core.get_owner_address();
                    Some(owner_addr)
                } else {
                    None
                };

                closed_tpsl = Some(ClosedTpsl {
                    transaction_version: txn_version,
                    market_id: inner.market.get_reference_address(),
                    position_id: inner.position.get_reference_address(),
                    transaction_timestamp: txn_timestamp,
                });
                MarketActivityHelper {
                    event_type: String::from("CancelTpslEvent"),
                    market_id: inner.market.get_reference_address(),
                    position_id: Some(inner.position.get_reference_address()),
                    id: None,
                    owner_addr,
                    perp_price: None,
                    is_long: None,
                    margin_amount: None,
                    position_size: None,
                    fee: None,
                    pnl: None,
                    take_profit_price: None,
                    stop_loss_price: None,
                    trigger_price: None,
                    max_price_slippage: None,
                    is_increase: None,
                    triggers_above: None,
                    expiration: None,
                    trigger_payment_amount: None,
                    next_funding_rate: None,
                }
            },
            MarketEvent::IncreaseTpslTriggerPaymentEvent(inner) => {
                let owner_addr = if let Some(object_metadata) =
                    object_metadatas.get(&inner.position.get_reference_address())
                {
                    let owner_addr = object_metadata.object.object_core.get_owner_address();
                    Some(owner_addr)
                } else {
                    None
                };

                open_tpsl = Some(OpenTpsl {
                    last_transaction_version: txn_version,
                    market_id: inner.market.get_reference_address(),
                    position_id: inner.position.get_reference_address(),
                    transaction_timestamp: txn_timestamp,
                });
                MarketActivityHelper {
                    event_type: String::from("IncreaseTpslTriggerPaymentEvent"),
                    market_id: inner.market.get_reference_address(),
                    position_id: Some(inner.position.get_reference_address()),
                    id: None,
                    owner_addr,
                    perp_price: None,
                    is_long: None,
                    margin_amount: None,
                    position_size: None,
                    fee: None,
                    pnl: None,
                    take_profit_price: None,
                    stop_loss_price: None,
                    trigger_price: None,
                    max_price_slippage: None,
                    is_increase: None,
                    triggers_above: None,
                    expiration: None,
                    trigger_payment_amount: Some(inner.increase_amount.clone()),
                    next_funding_rate: None,
                }
            },
            MarketEvent::DecreaseTpslTriggerPaymentEvent(inner) => {
                let owner_addr = if let Some(object_metadata) =
                    object_metadatas.get(&inner.position.get_reference_address())
                {
                    let owner_addr = object_metadata.object.object_core.get_owner_address();
                    Some(owner_addr)
                } else {
                    None
                };

                open_tpsl = Some(OpenTpsl {
                    last_transaction_version: txn_version,
                    market_id: inner.market.get_reference_address(),
                    position_id: inner.position.get_reference_address(),
                    transaction_timestamp: txn_timestamp,
                });
                MarketActivityHelper {
                    event_type: String::from("DecreaseTpslTriggerPaymentEvent"),
                    market_id: inner.market.get_reference_address(),
                    position_id: Some(inner.position.get_reference_address()),
                    id: None,
                    owner_addr,
                    perp_price: None,
                    is_long: None,
                    margin_amount: None,
                    position_size: None,
                    fee: None,
                    pnl: None,
                    take_profit_price: None,
                    stop_loss_price: None,
                    trigger_price: None,
                    max_price_slippage: None,
                    is_increase: None,
                    triggers_above: None,
                    expiration: None,
                    trigger_payment_amount: Some(inner.decrease_amount.clone()),
                    next_funding_rate: None,
                }
            },
            MarketEvent::TriggerTpslEvent(inner) => {
                let owner_addr = if let Some(object_metadata) =
                    object_metadatas.get(&inner.position.get_reference_address())
                {
                    let owner_addr = object_metadata.object.object_core.get_owner_address();
                    Some(owner_addr)
                } else {
                    None
                };

                closed_tpsl = Some(ClosedTpsl {
                    transaction_version: txn_version,
                    market_id: inner.market.get_reference_address(),
                    position_id: inner.position.get_reference_address(),
                    transaction_timestamp: txn_timestamp,
                });
                MarketActivityHelper {
                    event_type: String::from("TriggerTpslEvent"),
                    market_id: inner.market.get_reference_address(),
                    position_id: Some(inner.position.get_reference_address()),
                    id: None,
                    owner_addr,
                    perp_price: None,
                    is_long: None,
                    margin_amount: None,
                    position_size: None,
                    fee: None,
                    pnl: None,
                    take_profit_price: None,
                    stop_loss_price: None,
                    trigger_price: None,
                    max_price_slippage: None,
                    is_increase: None,
                    triggers_above: None,
                    expiration: None,
                    trigger_payment_amount: Some(inner.trigger_payment.clone()),
                    next_funding_rate: None,
                }
            },
            MarketEvent::PlaceLimitOrderEvent(inner) => {
                let owner_addr = if let Some(object_metadata) =
                    object_metadatas.get(&inner.position.get_reference_address())
                {
                    let owner_addr = object_metadata.object.object_core.get_owner_address();
                    Some(owner_addr)
                } else {
                    None
                };

                open_limit_order = Some(OpenLimitOrder {
                    last_transaction_version: txn_version,
                    market_id: inner.market.get_reference_address(),
                    position_id: inner.position.get_reference_address(),
                    limit_order_id: inner.id.clone(),
                    transaction_timestamp: txn_timestamp,
                });
                MarketActivityHelper {
                    event_type: String::from("PlaceLimitOrderEvent"),
                    market_id: inner.market.get_reference_address(),
                    position_id: Some(inner.position.get_reference_address()),
                    id: Some(inner.id.clone()),
                    owner_addr,
                    perp_price: None,
                    is_long: None,
                    margin_amount: Some(inner.margin_amount.clone()),
                    position_size: Some(inner.position_size.clone()),
                    fee: None,
                    pnl: None,
                    take_profit_price: None,
                    stop_loss_price: None,
                    trigger_price: Some(inner.trigger_price.clone()),
                    max_price_slippage: Some(inner.max_price_slippage.clone()),
                    is_increase: Some(inner.is_increase),
                    triggers_above: Some(inner.triggers_above),
                    expiration: Some(inner.expiration.clone()),
                    trigger_payment_amount: Some(inner.trigger_payment_amount.clone()),
                    next_funding_rate: None,
                }
            },
            MarketEvent::UpdateLimitOrderEvent(inner) => {
                let owner_addr = if let Some(object_metadata) =
                    object_metadatas.get(&inner.position.get_reference_address())
                {
                    let owner_addr = object_metadata.object.object_core.get_owner_address();
                    Some(owner_addr)
                } else {
                    None
                };

                open_limit_order = Some(OpenLimitOrder {
                    last_transaction_version: txn_version,
                    market_id: inner.market.get_reference_address(),
                    position_id: inner.position.get_reference_address(),
                    limit_order_id: inner.id.clone(),
                    transaction_timestamp: txn_timestamp,
                });
                MarketActivityHelper {
                    event_type: String::from("UpdateLimitOrderEvent"),
                    market_id: inner.market.get_reference_address(),
                    position_id: Some(inner.position.get_reference_address()),
                    id: Some(inner.id.clone()),
                    owner_addr,
                    perp_price: None,
                    is_long: None,
                    margin_amount: Some(inner.margin_amount.clone()),
                    position_size: Some(inner.position_size.clone()),
                    fee: None,
                    pnl: None,
                    take_profit_price: None,
                    stop_loss_price: None,
                    trigger_price: Some(inner.trigger_price.clone()),
                    max_price_slippage: Some(inner.max_price_slippage.clone()),
                    is_increase: None,
                    triggers_above: Some(inner.triggers_above),
                    expiration: Some(inner.expiration.clone()),
                    trigger_payment_amount: Some(inner.trigger_payment_amount.clone()),
                    next_funding_rate: None,
                }
            },
            MarketEvent::CancelLimitOrderEvent(inner) => {
                let owner_addr = if let Some(object_metadata) =
                    object_metadatas.get(&inner.position.get_reference_address())
                {
                    let owner_addr = object_metadata.object.object_core.get_owner_address();
                    Some(owner_addr)
                } else {
                    None
                };

                closed_limit_order = Some(ClosedLimitOrder {
                    transaction_version: txn_version,
                    market_id: inner.market.get_reference_address(),
                    position_id: inner.position.get_reference_address(),
                    limit_order_id: inner.id.clone(),
                    transaction_timestamp: txn_timestamp,
                });
                MarketActivityHelper {
                    event_type: String::from("CancelLimitOrderEvent"),
                    market_id: inner.market.get_reference_address(),
                    position_id: Some(inner.position.get_reference_address()),
                    id: Some(inner.id.clone()),
                    owner_addr,
                    perp_price: None,
                    is_long: None,
                    margin_amount: None,
                    position_size: None,
                    fee: None,
                    pnl: None,
                    take_profit_price: None,
                    stop_loss_price: None,
                    trigger_price: None,
                    max_price_slippage: None,
                    is_increase: None,
                    triggers_above: None,
                    expiration: None,
                    trigger_payment_amount: None,
                    next_funding_rate: None,
                }
            },
            MarketEvent::TriggerLimitOrderEvent(inner) => {
                let owner_addr = if let Some(object_metadata) =
                    object_metadatas.get(&inner.position.get_reference_address())
                {
                    let owner_addr = object_metadata.object.object_core.get_owner_address();
                    Some(owner_addr)
                } else {
                    None
                };

                closed_limit_order = Some(ClosedLimitOrder {
                    transaction_version: txn_version,
                    market_id: inner.market.get_reference_address(),
                    position_id: inner.position.get_reference_address(),
                    limit_order_id: inner.id.clone(),
                    transaction_timestamp: txn_timestamp,
                });
                MarketActivityHelper {
                    event_type: String::from("TriggerLimitOrderEvent"),
                    market_id: inner.market.get_reference_address(),
                    position_id: Some(inner.position.get_reference_address()),
                    id: Some(inner.id.clone()),
                    owner_addr,
                    perp_price: None,
                    is_long: None,
                    margin_amount: None,
                    position_size: None,
                    fee: None,
                    pnl: None,
                    take_profit_price: None,
                    stop_loss_price: None,
                    trigger_price: None,
                    max_price_slippage: None,
                    is_increase: None,
                    triggers_above: None,
                    expiration: None,
                    trigger_payment_amount: Some(inner.trigger_payment_amount.clone()),
                    next_funding_rate: None,
                }
            },
            MarketEvent::IncreaseLimitOrderTriggerPaymentEvent(inner) => {
                let owner_addr = if let Some(object_metadata) =
                    object_metadatas.get(&inner.position.get_reference_address())
                {
                    let owner_addr = object_metadata.object.object_core.get_owner_address();
                    Some(owner_addr)
                } else {
                    None
                };

                open_limit_order = Some(OpenLimitOrder {
                    last_transaction_version: txn_version,
                    market_id: inner.market.get_reference_address(),
                    position_id: inner.position.get_reference_address(),
                    limit_order_id: inner.id.clone(),
                    transaction_timestamp: txn_timestamp,
                });
                MarketActivityHelper {
                    event_type: String::from("IncreaseLimitOrderTriggerPaymentEvent"),
                    market_id: inner.market.get_reference_address(),
                    position_id: Some(inner.position.get_reference_address()),
                    id: Some(inner.id.clone()),
                    owner_addr,
                    perp_price: None,
                    is_long: None,
                    margin_amount: None,
                    position_size: None,
                    fee: None,
                    pnl: None,
                    take_profit_price: None,
                    stop_loss_price: None,
                    trigger_price: None,
                    max_price_slippage: None,
                    is_increase: None,
                    triggers_above: None,
                    expiration: None,
                    trigger_payment_amount: Some(inner.increase_amount.clone()),
                    next_funding_rate: None,
                }
            },
            MarketEvent::DecreaseLimitOrderTriggerPaymentEvent(inner) => {
                let owner_addr = if let Some(object_metadata) =
                    object_metadatas.get(&inner.position.get_reference_address())
                {
                    let owner_addr = object_metadata.object.object_core.get_owner_address();
                    Some(owner_addr)
                } else {
                    None
                };

                open_limit_order = Some(OpenLimitOrder {
                    last_transaction_version: txn_version,
                    market_id: inner.market.get_reference_address(),
                    position_id: inner.position.get_reference_address(),
                    limit_order_id: inner.id.clone(),
                    transaction_timestamp: txn_timestamp,
                });
                MarketActivityHelper {
                    event_type: String::from("DecreaseLimitOrderTriggerPaymentEvent"),
                    market_id: inner.market.get_reference_address(),
                    position_id: Some(inner.position.get_reference_address()),
                    id: Some(inner.id.clone()),
                    owner_addr,
                    perp_price: None,
                    is_long: None,
                    margin_amount: None,
                    position_size: None,
                    fee: None,
                    pnl: None,
                    take_profit_price: None,
                    stop_loss_price: None,
                    trigger_price: None,
                    max_price_slippage: None,
                    is_increase: None,
                    triggers_above: None,
                    expiration: None,
                    trigger_payment_amount: Some(inner.decrease_amount.clone()),
                    next_funding_rate: None,
                }
            },
        };
        (
            MarketActivityModel {
                transaction_version: txn_version,
                event_creation_number,
                event_sequence_number,
                event_type: market_activity_helper.event_type,
                event_index,
                market_id: market_activity_helper.market_id,
                position_id: market_activity_helper.position_id,
                id: market_activity_helper.id,
                owner_addr: market_activity_helper.owner_addr,
                perp_price: market_activity_helper.perp_price,
                is_long: market_activity_helper.is_long,
                margin_amount: market_activity_helper.margin_amount,
                position_size: market_activity_helper.position_size,
                fee: market_activity_helper.fee,
                pnl: market_activity_helper.pnl,
                take_profit_price: market_activity_helper.take_profit_price,
                stop_loss_price: market_activity_helper.stop_loss_price,
                trigger_price: market_activity_helper.trigger_price,
                max_price_slippage: market_activity_helper.max_price_slippage,
                is_increase: market_activity_helper.is_increase,
                triggers_above: market_activity_helper.triggers_above,
                expiration: market_activity_helper.expiration,
                trigger_payment_amount: market_activity_helper.trigger_payment_amount,
                next_funding_rate: market_activity_helper.next_funding_rate,
                transaction_timestamp: txn_timestamp,
            },
            trade,
            open_position,
            closed_position,
            open_tpsl,
            closed_tpsl,
            open_limit_order,
            closed_limit_order,
        )
    }
}
