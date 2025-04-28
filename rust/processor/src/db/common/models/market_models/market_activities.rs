// Copyright © Mirage Protocol

// This is required because a diesel macro makes clippy sad
#![allow(clippy::extra_unused_lifetimes)]
#![allow(clippy::unused_unit)]

use super::market_events::MarketEvent;
use crate::{
    schema::{
        current_limit_orders, current_positions, current_tpsls, market_activities, trade_datas,
    },
    utils::util::{parse_timestamp, standardize_address, ObjectOwnerMapping},
};
use aptos_protos::transaction::v1::{
    transaction::TxnData, Event as EventPB, Transaction as TransactionPB,
};
use bigdecimal::{BigDecimal, Zero};
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
    pub strategy_id: Option<String>,
    pub owner_addr: Option<String>,

    pub perp_price: Option<BigDecimal>,
    pub is_long: Option<bool>,
    pub margin_amount: Option<BigDecimal>,
    pub position_size: Option<BigDecimal>,
    pub fee: Option<BigDecimal>,
    pub protocol_fee: Option<BigDecimal>,
    pub pnl: Option<BigDecimal>,
    pub take_profit_price: Option<BigDecimal>,
    pub stop_loss_price: Option<BigDecimal>,
    pub trigger_price: Option<BigDecimal>,
    pub max_price_slippage: Option<BigDecimal>,
    pub is_decrease_only: Option<bool>,
    pub triggers_above: Option<bool>,
    pub expiration: Option<BigDecimal>,
    pub next_funding_rate: Option<BigDecimal>,
    pub transaction_timestamp: chrono::NaiveDateTime,
}

/// A simplified MarketActivity (excluded common fields) to reduce code duplication
struct MarketActivityHelper {
    pub market_id: String,
    pub event_type: String,

    pub position_id: Option<String>,
    pub strategy_id: Option<String>,
    pub owner_addr: Option<String>,

    pub perp_price: Option<BigDecimal>,
    pub is_long: Option<bool>,
    pub margin_amount: Option<BigDecimal>,
    pub position_size: Option<BigDecimal>,
    pub fee: Option<BigDecimal>,
    pub protocol_fee: Option<BigDecimal>,
    pub pnl: Option<BigDecimal>,
    pub take_profit_price: Option<BigDecimal>,
    pub stop_loss_price: Option<BigDecimal>,
    pub trigger_price: Option<BigDecimal>,
    pub max_price_slippage: Option<BigDecimal>,
    pub is_decrease_only: Option<bool>,
    pub triggers_above: Option<bool>,
    pub expiration: Option<BigDecimal>,
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
    pub event_type: String,

    pub transaction_timestamp: chrono::NaiveDateTime,
}

#[derive(Clone, Debug, Deserialize, FieldCount, Identifiable, Insertable, Serialize)]
#[diesel(primary_key(position_id))]
#[diesel(table_name = current_positions)]
pub struct CurrentPosition {
    pub last_transaction_version: i64,

    pub market_id: String,
    pub position_id: String,
    pub owner_addr: String,

    pub is_closed: bool,
    pub event_index: i64,

    pub transaction_timestamp: chrono::NaiveDateTime,
}

#[derive(Clone, Debug, Deserialize, FieldCount, Identifiable, Insertable, Serialize)]
#[diesel(primary_key(strategy_id))]
#[diesel(table_name = current_tpsls)]
pub struct CurrentTpsl {
    pub last_transaction_version: i64,

    pub market_id: String,
    pub strategy_id: String,
    pub position_id: String,
    pub owner_addr: String,

    pub is_closed: bool,
    pub event_index: i64,

    pub transaction_timestamp: chrono::NaiveDateTime,
}

#[derive(Clone, Debug, Deserialize, FieldCount, Identifiable, Insertable, Serialize)]
#[diesel(primary_key(strategy_id))]
#[diesel(table_name = current_limit_orders)]
pub struct CurrentLimitOrder {
    pub last_transaction_version: i64,

    pub market_id: String,
    pub strategy_id: String,
    pub position_id: String,
    pub owner_addr: String,

    pub is_closed: bool,
    pub event_index: i64,

    pub transaction_timestamp: chrono::NaiveDateTime,
}

impl MarketActivityModel {
    pub fn from_transaction(
        transaction: &TransactionPB,
        object_owners: &ObjectOwnerMapping,
        market_module_address: &str,
    ) -> (
        Vec<Trade>,
        Vec<CurrentPosition>,
        Vec<CurrentTpsl>,
        Vec<CurrentLimitOrder>,
        Vec<MarketActivityModel>,
    ) {
        let mut trades: Vec<Trade> = Vec::new();
        let mut current_positions: Vec<CurrentPosition> = Vec::new();
        let mut current_tpsls: Vec<CurrentTpsl> = Vec::new();
        let mut current_limit_orders: Vec<CurrentLimitOrder> = Vec::new();
        let mut market_activities: Vec<MarketActivityModel> = Vec::new();

        // Extracts events and user request from genesis and user transactions. Other transactions won't have coin events
        let txn_data = transaction
            .txn_data
            .as_ref()
            .expect("Txn Data doesn't exit!");
        let (events, sender_address) = match txn_data {
            TxnData::User(inner) => {
                let user_request = inner
                    .request
                    .as_ref()
                    .expect("Sender is not present in user txn");

                let sender_address = standardize_address(&user_request.sender);
                (&inner.events, sender_address)
            }
            _ => return Default::default(),
        };

        // The rest are fields common to all transactions
        let txn_version = transaction.version as i64;
        let txn_timestamp = transaction
            .timestamp
            .as_ref()
            .expect("Transaction timestamp doesn't exist!");
        let txn_timestamp = parse_timestamp(txn_timestamp, txn_version);

        for (index, event) in events.iter().enumerate() {
            let maybe_market_event =
                MarketEvent::from_event(event, txn_version, market_module_address).unwrap();

            if let Some(market_event) = maybe_market_event {
                let (
                    market_activity,
                    maybe_trade,
                    maybe_current_position,
                    maybe_current_tpsl,
                    maybe_current_limit_order,
                ) = Self::from_parsed_event(
                    event,
                    &market_event,
                    txn_version,
                    txn_timestamp,
                    index as i64,
                    object_owners,
                    &sender_address,
                );
                market_activities.push(market_activity);

                if let Some(inner) = maybe_trade {
                    trades.push(inner);
                }

                if let Some(inner) = maybe_current_position {
                    current_positions.push(inner);
                }
                if let Some(inner) = maybe_current_tpsl {
                    current_tpsls.push(inner);
                }
                if let Some(inner) = maybe_current_limit_order {
                    current_limit_orders.push(inner);
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
            current_positions,
            current_tpsls,
            current_limit_orders,
            market_activities,
        )
    }

    fn get_owner_address(
        object_owners: &ObjectOwnerMapping,
        object_address: &str,
        txn_version: i64,
        sender_address: &str,
    ) -> String {
        object_owners
            .get(object_address)
            .map(|s| s.clone())
            .unwrap_or_else(|| {
                tracing::info!(
                    "Object owner not found for the object address {} in transaction version {}. Using sender address.",
                    object_address,
                    txn_version
                );
                sender_address.to_string()
            })
    }

    fn from_parsed_event(
        event: &EventPB,
        parsed_event: &MarketEvent,
        txn_version: i64,
        txn_timestamp: chrono::NaiveDateTime,
        event_index: i64,
        object_owners: &ObjectOwnerMapping,
        sender_address: &str,
    ) -> (
        MarketActivityModel,
        Option<Trade>,
        Option<CurrentPosition>,
        Option<CurrentTpsl>,
        Option<CurrentLimitOrder>,
    ) {
        let event_creation_number = event.key.as_ref().unwrap().creation_number as i64;
        let event_sequence_number = event.sequence_number as i64;

        let mut trade = None;

        let mut current_position = None;
        let mut current_tpsl = None;
        let mut current_limit_order = None;

        let market_activity_helper = match parsed_event {
            MarketEvent::UpdateFundingEvent(inner) => MarketActivityHelper {
                event_type: String::from("UpdateFundingEvent"),
                market_id: inner.market.get_reference_address(),
                position_id: None,
                strategy_id: None,
                owner_addr: None,
                perp_price: None,
                is_long: None,
                margin_amount: None,
                position_size: None,
                fee: None,
                protocol_fee: None,
                pnl: None,
                take_profit_price: None,
                stop_loss_price: None,
                trigger_price: None,
                max_price_slippage: None,
                is_decrease_only: None,
                triggers_above: None,
                expiration: None,
                next_funding_rate: Some(inner.next_funding_rate.to_bigdecimal()),
            },
            MarketEvent::OpenPositionEvent(inner) => {
                let owner_addr = Self::get_owner_address(
                    object_owners,
                    &inner.position.get_reference_address(),
                    txn_version,
                    sender_address,
                );

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
                    event_type: String::from("OpenPositionEvent"),
                    transaction_timestamp: txn_timestamp,
                });

                current_position = Some(CurrentPosition {
                    last_transaction_version: txn_version,
                    market_id: inner.market.get_reference_address(),
                    position_id: inner.position.get_reference_address(),
                    owner_addr: owner_addr.clone(),
                    is_closed: false,
                    event_index,
                    transaction_timestamp: txn_timestamp,
                });

                MarketActivityHelper {
                    event_type: String::from("OpenPositionEvent"),
                    market_id: inner.market.get_reference_address(),
                    position_id: Some(inner.position.get_reference_address()),
                    strategy_id: None,
                    owner_addr: Some(owner_addr.to_string()),
                    perp_price: Some(inner.opening_price.clone()),
                    is_long: Some(inner.is_long),
                    margin_amount: Some(inner.margin_amount.clone()),
                    position_size: Some(inner.position_size.clone()),
                    fee: Some(inner.fee.clone()),
                    protocol_fee: None,
                    pnl: None,
                    take_profit_price: None,
                    stop_loss_price: None,
                    trigger_price: None,
                    max_price_slippage: None,
                    is_decrease_only: None,
                    triggers_above: None,
                    expiration: None,
                    next_funding_rate: None,
                }
            },
            MarketEvent::ClosePositionEvent(inner) => {
                let owner_addr = Self::get_owner_address(
                    object_owners,
                    &inner.position.get_reference_address(),
                    txn_version,
                    sender_address,
                );

                trade = Some(Trade {
                    transaction_version: txn_version,
                    market_id: inner.market.get_reference_address(),
                    position_id: inner.position.get_reference_address(),
                    owner_addr: owner_addr.clone(),
                    is_long: inner.is_long,
                    position_size: inner.position_size.clone(),
                    price: inner.closing_price.clone(),
                    fee: inner.fee.clone(),
                    pnl: inner.pnl.to_bigdecimal(),
                    event_type: String::from("ClosePositionEvent"),
                    transaction_timestamp: txn_timestamp,
                });
                current_position = Some(CurrentPosition {
                    last_transaction_version: txn_version,
                    market_id: inner.market.get_reference_address(),
                    position_id: inner.position.get_reference_address(),
                    owner_addr: owner_addr.clone(),
                    is_closed: true,
                    event_index,
                    transaction_timestamp: txn_timestamp,
                });

                MarketActivityHelper {
                    event_type: String::from("ClosePositionEvent"),
                    market_id: inner.market.get_reference_address(),
                    position_id: Some(inner.position.get_reference_address()),
                    strategy_id: None,
                    owner_addr: Some(owner_addr.to_string()),
                    perp_price: Some(inner.closing_price.clone()),
                    is_long: Some(inner.is_long),
                    margin_amount: None,
                    position_size: Some(inner.position_size.clone()),
                    fee: Some(inner.fee.clone()),
                    protocol_fee: None,
                    pnl: Some(inner.pnl.to_bigdecimal()),
                    take_profit_price: None,
                    stop_loss_price: None,
                    trigger_price: None,
                    max_price_slippage: None,
                    is_decrease_only: None,
                    triggers_above: None,
                    expiration: None,
                    next_funding_rate: None,
                }
            },
            MarketEvent::IncreaseMarginEvent(inner) => {
                let owner_addr = Self::get_owner_address(
                    object_owners,
                    &inner.position.get_reference_address(),
                    txn_version,
                    sender_address,
                );

                current_position = Some(CurrentPosition {
                    last_transaction_version: txn_version,
                    market_id: inner.market.get_reference_address(),
                    position_id: inner.position.get_reference_address(),
                    owner_addr: owner_addr.clone(),
                    is_closed: false,
                    event_index,
                    transaction_timestamp: txn_timestamp,
                });
                MarketActivityHelper {
                    event_type: String::from("IncreaseMarginEvent"),
                    market_id: inner.market.get_reference_address(),
                    position_id: Some(inner.position.get_reference_address()),
                    strategy_id: None,
                    owner_addr: Some(owner_addr.to_string()),
                    perp_price: None,
                    is_long: None,
                    margin_amount: Some(inner.margin_amount.clone()),
                    position_size: None,
                    fee: None,
                    protocol_fee: None,
                    pnl: None,
                    take_profit_price: None,
                    stop_loss_price: None,
                    trigger_price: None,
                    max_price_slippage: None,
                    is_decrease_only: None,
                    triggers_above: None,
                    expiration: None,
                    next_funding_rate: None,
                }
            },
            MarketEvent::DecreaseMarginEvent(inner) => {
                let owner_addr = Self::get_owner_address(
                    object_owners,
                    &inner.position.get_reference_address(),
                    txn_version,
                    sender_address,
                );

                current_position = Some(CurrentPosition {
                    last_transaction_version: txn_version,
                    market_id: inner.market.get_reference_address(),
                    position_id: inner.position.get_reference_address(),
                    owner_addr: owner_addr.clone(),
                    is_closed: true,
                    event_index,
                    transaction_timestamp: txn_timestamp,
                });
                MarketActivityHelper {
                    event_type: String::from("DecreaseMarginEvent"),
                    market_id: inner.market.get_reference_address(),
                    position_id: Some(inner.position.get_reference_address()),
                    strategy_id: None,
                    owner_addr: Some(owner_addr.to_string()),
                    perp_price: None,
                    is_long: None,
                    margin_amount: Some(inner.margin_amount.clone()),
                    position_size: None,
                    fee: None,
                    protocol_fee: None,
                    pnl: None,
                    take_profit_price: None,
                    stop_loss_price: None,
                    trigger_price: None,
                    max_price_slippage: None,
                    is_decrease_only: None,
                    triggers_above: None,
                    expiration: None,
                    next_funding_rate: None,
                }
            },
            MarketEvent::IncreasePositionSizeEvent(inner) => {
                let owner_addr = Self::get_owner_address(
                    object_owners,
                    &inner.position.get_reference_address(),
                    txn_version,
                    sender_address,
                );

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
                    event_type: String::from("IncreasePositionSizeEvent"),
                    transaction_timestamp: txn_timestamp,
                });
                current_position = Some(CurrentPosition {
                    last_transaction_version: txn_version,
                    market_id: inner.market.get_reference_address(),
                    position_id: inner.position.get_reference_address(),
                    owner_addr: owner_addr.clone(),
                    is_closed: false,
                    event_index,
                    transaction_timestamp: txn_timestamp,
                });

                MarketActivityHelper {
                    event_type: String::from("IncreasePositionSizeEvent"),
                    market_id: inner.market.get_reference_address(),
                    position_id: Some(inner.position.get_reference_address()),
                    strategy_id: None,
                    owner_addr: Some(owner_addr.to_string()),
                    perp_price: Some(inner.new_opening_price.clone()),
                    is_long: None,
                    margin_amount: None,
                    position_size: Some(inner.amount.clone()),
                    fee: Some(inner.fee.clone()),
                    protocol_fee: None,
                    pnl: None,
                    take_profit_price: None,
                    stop_loss_price: None,
                    trigger_price: None,
                    max_price_slippage: None,
                    is_decrease_only: None,
                    triggers_above: None,
                    expiration: None,
                    next_funding_rate: None,
                }
            },
            MarketEvent::DecreasePositionSizeEvent(inner) => {
                let owner_addr = Self::get_owner_address(
                    object_owners,
                    &inner.position.get_reference_address(),
                    txn_version,
                    sender_address,
                );

                trade = Some(Trade {
                    transaction_version: txn_version,
                    market_id: inner.market.get_reference_address(),
                    position_id: inner.position.get_reference_address(),
                    owner_addr: owner_addr.clone(),
                    is_long: inner.is_long,
                    position_size: inner.amount.clone(),
                    price: inner.closing_price.clone(),
                    fee: inner.fee.clone(),
                    pnl: BigDecimal::zero(),
                    event_type: String::from("DecreasePositionSizeEvent"),
                    transaction_timestamp: txn_timestamp,
                });

                current_position = Some(CurrentPosition {
                    last_transaction_version: txn_version,
                    market_id: inner.market.get_reference_address(),
                    position_id: inner.position.get_reference_address(),
                    owner_addr: owner_addr.clone(),
                    is_closed: false,
                    event_index,
                    transaction_timestamp: txn_timestamp,
                });
                MarketActivityHelper {
                    event_type: String::from("DecreasePositionSizeEvent"),
                    market_id: inner.market.get_reference_address(),
                    position_id: Some(inner.position.get_reference_address()),
                    strategy_id: None,
                    owner_addr: Some(owner_addr.to_string()),
                    perp_price: Some(inner.closing_price.clone()),
                    is_long: None,
                    margin_amount: None,
                    position_size: Some(inner.amount.clone()),
                    fee: Some(inner.fee.clone()),
                    protocol_fee: None,
                    pnl: None,
                    take_profit_price: None,
                    stop_loss_price: None,
                    trigger_price: None,
                    max_price_slippage: None,
                    is_decrease_only: None,
                    triggers_above: None,
                    expiration: None,
                    next_funding_rate: None,
                }
            },
            MarketEvent::LiquidatePositionEvent(inner) => {
                let owner_addr = Self::get_owner_address(
                    object_owners,
                    &inner.position.get_reference_address(),
                    txn_version,
                    sender_address,
                );

                trade = Some(Trade {
                    transaction_version: txn_version,
                    market_id: inner.market.get_reference_address(),
                    position_id: inner.position.get_reference_address(),
                    owner_addr: owner_addr.clone(),
                    is_long: inner.is_long,
                    position_size: inner.position_size.clone(),
                    price: inner.closing_price.clone(),
                    fee: inner.fee.clone(),
                    pnl: BigDecimal::zero(),
                    event_type: String::from("LiquidatePositionEvent"),
                    transaction_timestamp: txn_timestamp,
                });
                current_position = Some(CurrentPosition {
                    last_transaction_version: txn_version,
                    market_id: inner.market.get_reference_address(),
                    position_id: inner.position.get_reference_address(),
                    owner_addr: owner_addr.clone(),
                    is_closed: true,
                    event_index,
                    transaction_timestamp: txn_timestamp,
                });

                MarketActivityHelper {
                    event_type: String::from("LiquidatePositionEvent"),
                    market_id: inner.market.get_reference_address(),
                    position_id: Some(inner.position.get_reference_address()),
                    strategy_id: None,
                    owner_addr: Some(owner_addr.to_string()),
                    perp_price: Some(inner.closing_price.clone()),
                    is_long: Some(inner.is_long),
                    margin_amount: Some(inner.remaining_maintenance_margin.clone()),
                    position_size: Some(inner.position_size.clone()),
                    fee: Some(inner.fee.clone()),
                    protocol_fee: None,
                    pnl: None,
                    take_profit_price: None,
                    stop_loss_price: None,
                    trigger_price: None,
                    max_price_slippage: None,
                    is_decrease_only: None,
                    triggers_above: None,
                    expiration: None,
                    next_funding_rate: None,
                }
            },
            MarketEvent::LiquidatePositionV2Event(inner) => {
                let owner_addr = Self::get_owner_address(
                    object_owners,
                    &inner.position.get_reference_address(),
                    txn_version,
                    sender_address,
                );

                trade = Some(Trade {
                    transaction_version: txn_version,
                    market_id: inner.market.get_reference_address(),
                    position_id: inner.position.get_reference_address(),
                    owner_addr: owner_addr.clone(),
                    is_long: inner.is_long,
                    position_size: inner.position_size.clone(),
                    price: inner.closing_price.clone(),
                    fee: inner.fee.clone(),
                    pnl: inner.pnl.to_bigdecimal(),
                    event_type: String::from("LiquidatePositionEvent"),
                    transaction_timestamp: txn_timestamp,
                });
                current_position = Some(CurrentPosition {
                    last_transaction_version: txn_version,
                    market_id: inner.market.get_reference_address(),
                    position_id: inner.position.get_reference_address(),
                    owner_addr: owner_addr.clone(),
                    is_closed: true,
                    event_index,
                    transaction_timestamp: txn_timestamp,
                });

                MarketActivityHelper {
                    event_type: String::from("LiquidatePositionEvent"),
                    market_id: inner.market.get_reference_address(),
                    position_id: Some(inner.position.get_reference_address()),
                    strategy_id: None,
                    owner_addr: Some(owner_addr.to_string()),
                    perp_price: Some(inner.closing_price.clone()),
                    is_long: Some(inner.is_long),
                    margin_amount: Some(inner.remaining_maintenance_margin.clone()),
                    position_size: Some(inner.position_size.clone()),
                    fee: Some(inner.fee.clone()),
                    protocol_fee: None,
                    pnl: Some(inner.pnl.to_bigdecimal()),
                    take_profit_price: None,
                    stop_loss_price: None,
                    trigger_price: None,
                    max_price_slippage: None,
                    is_decrease_only: None,
                    triggers_above: None,
                    expiration: None,
                    next_funding_rate: None,
                }
            },
 
            MarketEvent::PlaceTpslEvent(inner) => {
                let owner_addr = Self::get_owner_address(
                    object_owners,
                    &inner.position.get_reference_address(),
                    txn_version,
                    sender_address,
                );

                current_tpsl = Some(CurrentTpsl {
                    last_transaction_version: txn_version,
                    market_id: inner.market.get_reference_address(),
                    strategy_id: inner.tpsl.get_reference_address(),
                    position_id: inner.position.get_reference_address(),
                    owner_addr: owner_addr.clone(),
                    is_closed: false,
                    event_index,
                    transaction_timestamp: txn_timestamp,
                });
                MarketActivityHelper {
                    event_type: String::from("PlaceTpslEvent"),
                    market_id: inner.market.get_reference_address(),
                    position_id: Some(inner.position.get_reference_address()),
                    strategy_id: Some(inner.tpsl.get_reference_address()),
                    owner_addr: Some(owner_addr.to_string()),
                    perp_price: None,
                    is_long: Some(inner.is_long),
                    margin_amount: None,
                    position_size: None,
                    fee: None,
                    protocol_fee: None,
                    pnl: None,
                    take_profit_price: Some(inner.take_profit_price.clone()),
                    stop_loss_price: Some(inner.stop_loss_price.clone()),
                    trigger_price: None,
                    max_price_slippage: None,
                    is_decrease_only: None,
                    triggers_above: None,
                    expiration: None,
                    next_funding_rate: None,
                }
            },
            MarketEvent::UpdateTpslEvent(inner) => {
                let owner_addr = Self::get_owner_address(
                    object_owners,
                    &inner.position.get_reference_address(),
                    txn_version,
                    sender_address,
                );

                current_tpsl = Some(CurrentTpsl {
                    last_transaction_version: txn_version,
                    market_id: inner.market.get_reference_address(),
                    position_id: inner.position.get_reference_address(),
                    strategy_id: inner.tpsl.get_reference_address(),
                    owner_addr: owner_addr.clone(),
                    is_closed: false,
                    event_index,
                    transaction_timestamp: txn_timestamp,
                });

                MarketActivityHelper {
                    event_type: String::from("UpdateTpslEvent"),
                    market_id: inner.market.get_reference_address(),
                    position_id: Some(inner.position.get_reference_address()),
                    strategy_id: Some(inner.tpsl.get_reference_address()),
                    owner_addr: Some(owner_addr.to_string()),
                    perp_price: None,
                    is_long: Some(inner.is_long),
                    margin_amount: None,
                    position_size: None,
                    fee: None,
                    protocol_fee: None,
                    pnl: None,
                    take_profit_price: Some(inner.take_profit_price.clone()),
                    stop_loss_price: Some(inner.stop_loss_price.clone()),
                    trigger_price: None,
                    max_price_slippage: None,
                    is_decrease_only: None,
                    triggers_above: None,
                    expiration: None,
                    next_funding_rate: None,
                }
            },
            MarketEvent::CancelTpslEvent(inner) => {
                let owner_addr = Self::get_owner_address(
                    object_owners,
                    &inner.position.get_reference_address(),
                    txn_version,
                    sender_address,
                );

                current_tpsl = Some(CurrentTpsl {
                    last_transaction_version: txn_version,
                    market_id: inner.market.get_reference_address(),
                    position_id: inner.position.get_reference_address(),
                    strategy_id: inner.tpsl.get_reference_address(),
                    owner_addr: owner_addr.clone(),
                    is_closed: true,
                    event_index,
                    transaction_timestamp: txn_timestamp,
                });
                MarketActivityHelper {
                    event_type: String::from("CancelTpslEvent"),
                    market_id: inner.market.get_reference_address(),
                    position_id: Some(inner.position.get_reference_address()),
                    strategy_id: Some(inner.tpsl.get_reference_address()),
                    owner_addr: Some(owner_addr.to_string()),
                    perp_price: None,
                    is_long: None,
                    margin_amount: None,
                    position_size: None,
                    fee: None,
                    protocol_fee: None,
                    pnl: None,
                    take_profit_price: None,
                    stop_loss_price: None,
                    trigger_price: None,
                    max_price_slippage: None,
                    is_decrease_only: None,
                    triggers_above: None,
                    expiration: None,
                    next_funding_rate: None,
                }
            },
            MarketEvent::TriggerTpslEvent(inner) => {
                let owner_addr = Self::get_owner_address(
                    object_owners,
                    &inner.position.get_reference_address(),
                    txn_version,
                    sender_address,
                );
                current_tpsl = Some(CurrentTpsl {
                    last_transaction_version: txn_version,
                    market_id: inner.market.get_reference_address(),
                    position_id: inner.position.get_reference_address(),
                    strategy_id: inner.tpsl.get_reference_address(),
                    owner_addr: owner_addr.clone(),
                    is_closed: true,
                    event_index,
                    transaction_timestamp: txn_timestamp,
                });

                MarketActivityHelper {
                    event_type: String::from("TriggerTpslEvent"),
                    market_id: inner.market.get_reference_address(),
                    position_id: Some(inner.position.get_reference_address()),
                    strategy_id: Some(inner.tpsl.get_reference_address()),
                    owner_addr: Some(owner_addr.to_string()),
                    perp_price: None,
                    is_long: None,
                    margin_amount: None,
                    position_size: None,
                    fee: None,
                    protocol_fee: None,
                    pnl: None,
                    take_profit_price: None,
                    stop_loss_price: None,
                    trigger_price: None,
                    max_price_slippage: None,
                    is_decrease_only: None,
                    triggers_above: None,
                    expiration: None,
                    next_funding_rate: None,
                }
            },
            MarketEvent::PlaceLimitOrderEvent(inner) => {
                let owner_addr = Self::get_owner_address(
                    object_owners,
                    &inner.position.get_reference_address(),
                    txn_version,
                    sender_address,
                );
                current_limit_order = Some(CurrentLimitOrder {
                    last_transaction_version: txn_version,
                    market_id: inner.market.get_reference_address(),
                    position_id: inner.position.get_reference_address(),
                    strategy_id: inner.limit_order.get_reference_address(),
                    owner_addr: owner_addr.clone(),
                    is_closed: false,
                    event_index,
                    transaction_timestamp: txn_timestamp,
                });

                MarketActivityHelper {
                    event_type: String::from("PlaceLimitOrderEvent"),
                    market_id: inner.market.get_reference_address(),
                    position_id: Some(inner.position.get_reference_address()),
                    strategy_id: Some(inner.limit_order.get_reference_address()),
                    owner_addr: Some(owner_addr.to_string()),
                    perp_price: None,
                    is_long: None,
                    margin_amount: Some(inner.margin_amount.clone()),
                    position_size: Some(inner.position_size.clone()),
                    fee: None,
                    protocol_fee: None,
                    pnl: None,
                    take_profit_price: None,
                    stop_loss_price: None,
                    trigger_price: Some(inner.trigger_price.clone()),
                    max_price_slippage: Some(inner.max_price_slippage.clone()),
                    is_decrease_only: Some(inner.is_decrease_only),
                    triggers_above: Some(inner.triggers_above),
                    expiration: Some(inner.expiration.clone()),
                    next_funding_rate: None,
                }
            },
            MarketEvent::UpdateLimitOrderEvent(inner) => {
                let owner_addr = Self::get_owner_address(
                    object_owners,
                    &inner.position.get_reference_address(),
                    txn_version,
                    sender_address,
                );

                current_limit_order = Some(CurrentLimitOrder {
                    last_transaction_version: txn_version,
                    market_id: inner.market.get_reference_address(),
                    position_id: inner.position.get_reference_address(),
                    strategy_id: inner.limit_order.get_reference_address(),
                    owner_addr: owner_addr.clone(),
                    is_closed: false,
                    event_index,
                    transaction_timestamp: txn_timestamp,
                });

                MarketActivityHelper {
                    event_type: String::from("UpdateLimitOrderEvent"),
                    market_id: inner.market.get_reference_address(),
                    position_id: Some(inner.position.get_reference_address()),
                    strategy_id: Some(inner.limit_order.get_reference_address()),
                    owner_addr: Some(owner_addr.to_string()),
                    perp_price: None,
                    is_long: Some(inner.is_long),
                    margin_amount: None,
                    position_size: Some(inner.position_size.clone()),
                    fee: None,
                    protocol_fee: None,
                    pnl: None,
                    take_profit_price: None,
                    stop_loss_price: None,
                    trigger_price: Some(inner.trigger_price.clone()),
                    max_price_slippage: Some(inner.max_price_slippage.clone()),
                    is_decrease_only: Some(inner.is_decrease_only),
                    triggers_above: Some(inner.triggers_above),
                    expiration: Some(inner.expiration.clone()),
                    next_funding_rate: None,
                }
            },
            MarketEvent::CancelLimitOrderEvent(inner) => {
                let owner_addr = Self::get_owner_address(
                    object_owners,
                    &inner.position.get_reference_address(),
                    txn_version,
                    sender_address,
                );

                current_limit_order = Some(CurrentLimitOrder {
                    last_transaction_version: txn_version,
                    market_id: inner.market.get_reference_address(),
                    position_id: inner.position.get_reference_address(),
                    strategy_id: inner.limit_order.get_reference_address(),
                    owner_addr: owner_addr.clone(),
                    is_closed: true,
                    event_index,
                    transaction_timestamp: txn_timestamp,
                });
                MarketActivityHelper {
                    event_type: String::from("CancelLimitOrderEvent"),
                    market_id: inner.market.get_reference_address(),
                    position_id: Some(inner.position.get_reference_address()),
                    strategy_id: Some(inner.limit_order.get_reference_address()),
                    owner_addr: Some(owner_addr.to_string()),
                    perp_price: None,
                    is_long: None,
                    margin_amount: None,
                    position_size: None,
                    fee: None,
                    protocol_fee: None,
                    pnl: None,
                    take_profit_price: None,
                    stop_loss_price: None,
                    trigger_price: None,
                    max_price_slippage: None,
                    is_decrease_only: None,
                    triggers_above: None,
                    expiration: None,
                    next_funding_rate: None,
                }
            },
            MarketEvent::IncreaseLimitOrderMarginEvent(inner) => {
                let owner_addr = Self::get_owner_address(
                    object_owners,
                    &inner.position.get_reference_address(),
                    txn_version,
                    sender_address,
                );

                current_limit_order = Some(CurrentLimitOrder {
                    last_transaction_version: txn_version,
                    market_id: inner.market.get_reference_address(),
                    position_id: inner.position.get_reference_address(),
                    strategy_id: inner.limit_order.get_reference_address(),
                    owner_addr: owner_addr.clone(),
                    is_closed: false,
                    event_index,
                    transaction_timestamp: txn_timestamp,
                });

                MarketActivityHelper {
                    event_type: String::from("IncreaseLimitOrderMarginEvent"),
                    market_id: inner.market.get_reference_address(),
                    position_id: Some(inner.position.get_reference_address()),
                    strategy_id: Some(inner.limit_order.get_reference_address()),
                    owner_addr: Some(owner_addr.to_string()),
                    perp_price: None,
                    is_long: None,
                    margin_amount: Some(inner.margin_amount.clone()),
                    position_size: None,
                    fee: None,
                    protocol_fee: None,
                    pnl: None,
                    take_profit_price: None,
                    stop_loss_price: None,
                    trigger_price: None,
                    max_price_slippage: None,
                    is_decrease_only: None,
                    triggers_above: None,
                    expiration: None,
                    next_funding_rate: None,
                }
            },
            MarketEvent::DecreaseLimitOrderMarginEvent(inner) => {
                let owner_addr = Self::get_owner_address(
                    object_owners,
                    &inner.position.get_reference_address(),
                    txn_version,
                    sender_address,
                );

                current_limit_order = Some(CurrentLimitOrder {
                    last_transaction_version: txn_version,
                    market_id: inner.market.get_reference_address(),
                    position_id: inner.position.get_reference_address(),
                    strategy_id: inner.limit_order.get_reference_address(),
                    owner_addr: owner_addr.clone(),
                    is_closed: true,
                    event_index,
                    transaction_timestamp: txn_timestamp,
                });
                MarketActivityHelper {
                    event_type: String::from("DecreaseLimitOrderMarginEvent"),
                    market_id: inner.market.get_reference_address(),
                    position_id: Some(inner.position.get_reference_address()),
                    strategy_id: Some(inner.limit_order.get_reference_address()),
                    owner_addr: Some(owner_addr.to_string()),
                    perp_price: None,
                    is_long: None,
                    margin_amount: Some(inner.margin_amount.clone()),
                    position_size: None,
                    fee: None,
                    protocol_fee: None,
                    pnl: None,
                    take_profit_price: None,
                    stop_loss_price: None,
                    trigger_price: None,
                    max_price_slippage: None,
                    is_decrease_only: None,
                    triggers_above: None,
                    expiration: None,
                    next_funding_rate: None,
                }
            },
            MarketEvent::TriggerLimitOrderEvent(inner) => {
                let owner_addr = Self::get_owner_address(
                    object_owners,
                    &inner.position.get_reference_address(),
                    txn_version,
                    sender_address,
                );
                current_limit_order = Some(CurrentLimitOrder {
                    last_transaction_version: txn_version,
                    market_id: inner.market.get_reference_address(),
                    position_id: inner.position.get_reference_address(),
                    strategy_id: inner.limit_order.get_reference_address(),
                    owner_addr: owner_addr.clone(),
                    is_closed: true,
                    event_index,
                    transaction_timestamp: txn_timestamp,
                });
                MarketActivityHelper {
                    event_type: String::from("TriggerLimitOrderEvent"),
                    market_id: inner.market.get_reference_address(),
                    position_id: Some(inner.position.get_reference_address()),
                    strategy_id: Some(inner.limit_order.get_reference_address()),
                    owner_addr: Some(owner_addr.to_string()),
                    perp_price: None,
                    is_long: None,
                    margin_amount: None,
                    position_size: None,
                    fee: None,
                    protocol_fee: None,
                    pnl: None,
                    take_profit_price: None,
                    stop_loss_price: None,
                    trigger_price: None,
                    max_price_slippage: None,
                    is_decrease_only: None,
                    triggers_above: None,
                    expiration: None,
                    next_funding_rate: None,
                }
            },
            MarketEvent::SettlePnlEvent(inner) => {
                let owner_addr = Self::get_owner_address(
                    object_owners,
                    &inner.position.get_reference_address(),
                    txn_version,
                    sender_address,
                );

                MarketActivityHelper {
                    event_type: String::from("TriggerLimitOrderEvent"),
                    market_id: inner.market.get_reference_address(),
                    position_id: Some(inner.position.get_reference_address()),
                    strategy_id: None,
                    owner_addr: Some(owner_addr.to_string()),
                    perp_price: None,
                    is_long: None,
                    margin_amount: None,
                    position_size: None,
                    fee: None,
                    protocol_fee: None,
                    pnl: Some(inner.pnl.to_bigdecimal()),
                    take_profit_price: None,
                    stop_loss_price: None,
                    trigger_price: None,
                    max_price_slippage: None,
                    is_decrease_only: None,
                    triggers_above: None,
                    expiration: None,
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
                strategy_id: market_activity_helper.strategy_id,
                owner_addr: market_activity_helper.owner_addr,
                perp_price: market_activity_helper.perp_price,
                is_long: market_activity_helper.is_long,
                margin_amount: market_activity_helper.margin_amount,
                position_size: market_activity_helper.position_size,
                fee: market_activity_helper.fee,
                protocol_fee: market_activity_helper.protocol_fee,
                pnl: market_activity_helper.pnl,
                take_profit_price: market_activity_helper.take_profit_price,
                stop_loss_price: market_activity_helper.stop_loss_price,
                trigger_price: market_activity_helper.trigger_price,
                max_price_slippage: market_activity_helper.max_price_slippage,
                is_decrease_only: market_activity_helper.is_decrease_only,
                triggers_above: market_activity_helper.triggers_above,
                expiration: market_activity_helper.expiration,
                next_funding_rate: market_activity_helper.next_funding_rate,
                transaction_timestamp: txn_timestamp,
            },
            trade,
            current_position,
            current_tpsl,
            current_limit_order,
        )
    }
}
