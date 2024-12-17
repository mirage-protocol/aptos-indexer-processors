// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

// This is required because a diesel macro makes clippy sad
#![allow(clippy::extra_unused_lifetimes)]
#![allow(clippy::unused_unit)]

use super::market_utils::{LimitOrder, MarketCollection};
use crate::{
    db::common::models::market_models::market_utils::{Position, TpSl, StrategyObjectMapping},
    schema::{limit_order_datas, market_configs, market_datas, position_datas, tpsl_datas},
    utils::util::{bigdecimal_to_u64, parse_timestamp_secs, standardize_address, ObjectOwnerMapping},
};
use aptos_protos::transaction::v1::WriteResource;
use bigdecimal::BigDecimal;
use diesel::prelude::*;
use field_count::FieldCount;
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Deserialize, FieldCount, Identifiable, Insertable, Serialize)]
#[diesel(primary_key(transaction_version, write_set_change_index))]
#[diesel(table_name = market_configs)]
pub struct MarketConfigModel {
    pub transaction_version: i64,
    pub write_set_change_index: i64,

    pub market_id: String,
    pub margin_token_id: String,
    pub perp_symbol: String,

    pub min_taker_fee: BigDecimal,
    pub max_taker_fee: BigDecimal,
    pub min_maker_fee: BigDecimal,
    pub max_maker_fee: BigDecimal,

    pub min_funding_rate: BigDecimal,
    pub max_funding_rate: BigDecimal,
    pub base_funding_rate: BigDecimal,
    pub funding_interval: BigDecimal,

    pub max_oi: BigDecimal,
    pub max_oi_imbalance: BigDecimal,

    pub maintenance_margin: BigDecimal,
    pub max_leverage: BigDecimal,
    pub min_order_size: BigDecimal,
    pub max_order_size: BigDecimal,
    pub min_margin_amount: BigDecimal,

    pub transaction_timestamp: chrono::NaiveDateTime,
}

#[derive(Clone, Debug, Deserialize, FieldCount, Identifiable, Insertable, Serialize)]
#[diesel(primary_key(transaction_version, write_set_change_index))]
#[diesel(table_name = market_datas)]
pub struct MarketCollectionModel {
    pub transaction_version: i64,
    pub write_set_change_index: i64,

    pub market_id: String,
    pub margin_token_id: String,
    pub perp_symbol: String,

    pub total_long_margin: BigDecimal,
    pub total_short_margin: BigDecimal,

    pub long_oi: BigDecimal,
    pub short_oi: BigDecimal,

    pub long_funding_accumulated_per_unit: BigDecimal,
    pub short_funding_accumulated_per_unit: BigDecimal,
    pub total_long_funding_accumulated: BigDecimal,
    pub total_short_funding_accumulated: BigDecimal,

    pub next_funding_rate: BigDecimal,
    pub last_funding_round: chrono::NaiveDateTime,

    pub is_long_close_only: bool,
    pub is_short_close_only: bool,

    pub transaction_timestamp: chrono::NaiveDateTime,
}

impl MarketCollectionModel {
    /// Fungible asset is part of an object and we need to get the object first to get owner address
    pub fn from_write_resource(
        write_resource: &WriteResource,
        write_set_change_index: i64,
        txn_version: i64,
        txn_timestamp: chrono::NaiveDateTime,
        market_module_address: &str,
    ) -> anyhow::Result<Option<(MarketCollectionModel, MarketConfigModel)>> {
        if let Some(inner) = &MarketCollection::from_write_resource(write_resource, txn_version, market_module_address)? {
            // the new coin type
            let collection_id = standardize_address(&write_resource.address.to_string());

            return Ok(Some((
                MarketCollectionModel {
                    transaction_version: txn_version,
                    write_set_change_index,
                    market_id: collection_id.clone(),
                    margin_token_id: inner.margin_token.get_reference_address(),
                    perp_symbol: inner.perp_symbol.clone(),
                    total_long_margin: inner.total_long_margin.clone(),
                    total_short_margin: inner.total_short_margin.clone(),
                    long_oi: inner.long_oi.clone(),
                    short_oi: inner.short_oi.clone(),
                    long_funding_accumulated_per_unit: inner
                        .long_funding_accumulated_per_unit
                        .to_bigdecimal(),
                    short_funding_accumulated_per_unit: inner
                        .short_funding_accumulated_per_unit
                        .to_bigdecimal(),
                    total_long_funding_accumulated: inner
                        .total_long_funding_accumulated
                        .to_bigdecimal(),
                    total_short_funding_accumulated: inner
                        .total_short_funding_accumulated
                        .to_bigdecimal(),
                    next_funding_rate: inner.next_funding_rate.to_bigdecimal(),
                    last_funding_round: parse_timestamp_secs(
                        bigdecimal_to_u64(&inner.last_funding_round),
                        txn_version,
                    ),
                    is_long_close_only: inner.is_long_close_only,
                    is_short_close_only: inner.is_short_close_only,
                    transaction_timestamp: txn_timestamp,
                },
                MarketConfigModel {
                    transaction_version: txn_version,
                    write_set_change_index,
                    market_id: collection_id,
                    margin_token_id: inner.margin_token.get_reference_address(),
                    perp_symbol: inner.perp_symbol.clone(),
                    min_taker_fee: inner.config.fees.min_taker_fee.clone(),
                    max_taker_fee: inner.config.fees.max_taker_fee.clone(),
                    min_maker_fee: inner.config.fees.min_maker_fee.clone(),
                    max_maker_fee: inner.config.fees.max_maker_fee.clone(),
                    min_funding_rate: inner.config.funding.min_funding_rate.clone(),
                    max_funding_rate: inner.config.funding.max_funding_rate.clone(),
                    base_funding_rate: inner.config.funding.base_funding_rate.clone(),
                    funding_interval: inner.config.funding.funding_interval.clone(),
                    max_oi: inner.config.max_oi.clone(),
                    max_oi_imbalance: inner.config.max_oi_imbalance.clone(),
                    maintenance_margin: inner.config.maintenance_margin.clone(),
                    max_leverage: inner.config.max_leverage.clone(),
                    min_order_size: inner.config.min_order_size.clone(),
                    max_order_size: inner.config.max_order_size.clone(),
                    min_margin_amount: inner.config.min_margin_amount.clone(),
                    transaction_timestamp: txn_timestamp,
                },
            )));
        };
        Ok(None)
    }
}

#[derive(Clone, Debug, Deserialize, FieldCount, Identifiable, Insertable, Serialize)]
#[diesel(primary_key(transaction_version, write_set_change_index))]
#[diesel(table_name = position_datas)]
pub struct PositionModel {
    pub transaction_version: i64,
    pub write_set_change_index: i64,

    pub owner_addr: String,
    pub market_id: String,
    pub position_id: String,

    pub last_settled_price: BigDecimal,
    pub last_open_timestamp: BigDecimal,
    pub side: String,
    pub margin_amount: BigDecimal,
    pub total_strategy_margin: BigDecimal,
    pub position_size: BigDecimal,
    pub last_funding_accumulated: BigDecimal,

    pub transaction_timestamp: chrono::NaiveDateTime,
}

#[derive(Clone, Debug, Deserialize, FieldCount, Identifiable, Insertable, Serialize)]
#[diesel(primary_key(transaction_version, write_set_change_index))]
#[diesel(table_name = tpsl_datas)]
pub struct TpSlModel {
    pub transaction_version: i64,
    pub write_set_change_index: i64,

    pub position_id: String,
    pub strategy_id: String,

    pub take_profit_price: BigDecimal,
    pub stop_loss_price: BigDecimal,

    pub transaction_timestamp: chrono::NaiveDateTime,
}

impl PositionModel {
    pub fn get_from_write_resource(
        write_resource: &WriteResource,
        txn_version: i64,
        write_set_change_index: i64,
        txn_timestamp: chrono::NaiveDateTime,
        object_owners: &ObjectOwnerMapping,
        market_module_address: &str,
    ) -> anyhow::Result<Option<Self>> {
        if let Some(inner) = &Position::from_write_resource(write_resource, txn_version, market_module_address)? {
            let position_id = standardize_address(&write_resource.address.to_string());
            if let Some(owner_addr) = object_owners.get(&position_id) {
                return Ok(Some(Self {
                    transaction_version: txn_version,
                    write_set_change_index,
                    owner_addr: owner_addr.clone(),
                    market_id: inner.market.get_reference_address(),
                    position_id,
                    last_settled_price: inner.last_settled_price.clone(),
                    last_open_timestamp: inner.last_open_timestamp.clone(),
                    side: inner.side.to_string(),
                    margin_amount: inner.margin_amount.clone(),
                    total_strategy_margin: inner.total_strategy_margin_amount.clone(),
                    position_size: inner.position_size.clone(),
                    last_funding_accumulated: inner.last_funding_accumulated.to_bigdecimal(),
                    transaction_timestamp: txn_timestamp,
                }));
            } else {
                // ObjectCore should not be missing, returning from entire function early
                return Ok(None);
            }
        }
        Ok(None)
    }
}

impl TpSlModel {
    pub fn get_from_write_resource(
        write_resource: &WriteResource,
        txn_version: i64,
        write_set_change_index: i64,
        txn_timestamp: chrono::NaiveDateTime,
        object_owners: &ObjectOwnerMapping,
        market_module_address: &str,
    ) -> anyhow::Result<Option<Self>> {
        if let Some(inner) = &TpSl::from_write_resource(write_resource, txn_version, market_module_address)? {
            let strategy_id = standardize_address(&write_resource.address.to_string());
            if let Some(position_id) = object_owners.get(&strategy_id) {
                return Ok(Some(Self {
                    transaction_version: txn_version,
                    write_set_change_index,
                    strategy_id: strategy_id.clone(),
                    position_id: position_id.clone(),
                    take_profit_price: inner.take_profit_price.clone(),
                    stop_loss_price: inner.stop_loss_price.clone(),
                    transaction_timestamp: txn_timestamp,
                }));
            } else {
                // ObjectCore should not be missing, returning from entire function early
                return Ok(None);
            }
        }
        Ok(None)
    }
}

#[derive(Clone, Debug, Deserialize, FieldCount, Identifiable, Insertable, Serialize)]
#[diesel(primary_key(transaction_version, write_set_change_index))]
#[diesel(table_name = limit_order_datas)]
pub struct LimitOrderModel {
    pub transaction_version: i64,
    pub write_set_change_index: i64,

    pub position_id: String,
    pub strategy_id: String,

    pub is_decrease_only: bool,
    pub position_size: BigDecimal,
    pub is_long: bool,
    pub margin: BigDecimal,
    pub trigger_price: BigDecimal,
    pub triggers_above: bool,
    pub max_price_slippage: BigDecimal,
    pub expiration: BigDecimal,

    pub transaction_timestamp: chrono::NaiveDateTime,
}


impl LimitOrderModel {
    pub fn get_from_write_resource(
        write_resource: &WriteResource,
        txn_version: i64,
        write_set_change_index: i64,
        txn_timestamp: chrono::NaiveDateTime,
        object_owners: &ObjectOwnerMapping,
        strategy_objects: &StrategyObjectMapping,
        market_module_address: &str,
    ) -> anyhow::Result<Option<LimitOrderModel>> {
        if let Some(inner) = &LimitOrder::from_write_resource(write_resource, txn_version, market_module_address)? {
            let strategy_id = standardize_address(&write_resource.address.to_string());
            if let (Some(position_id), Some(strategy)) = (object_owners.get(&strategy_id), strategy_objects.get(&strategy_id)) {
                return Ok(Some(LimitOrderModel {
                        transaction_version: txn_version,
                        write_set_change_index,
                        position_id: position_id.clone(), 
                        strategy_id: strategy_id.clone(),
                        is_decrease_only: inner.is_decrease_only,
                        position_size: inner.position_size.clone(),
                        is_long: inner.is_long,
                        margin: strategy.strategy_margin_amount.clone(),
                        trigger_price: inner.trigger_price.clone(),
                        triggers_above: inner.triggers_above,
                        max_price_slippage: inner.max_price_slippage.clone(),
                        expiration: inner.expiration.clone(),
                        transaction_timestamp: txn_timestamp,
                }))
            } else {
                // ObjectCore and Strategy should not be missing, returning from entire function early
                return Ok(None);
            }
        }
        Ok(None)
    }
}
