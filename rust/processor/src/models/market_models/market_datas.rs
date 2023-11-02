// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

// This is required because a diesel macro makes clippy sad
#![allow(clippy::extra_unused_lifetimes)]
#![allow(clippy::unused_unit)]

use super::market_utils::{LimitOrders, MarketCollection};
use crate::{
    models::{
        market_models::market_utils::{Position, TpSl},
        object_models::v2_object_utils::ObjectAggregatedDataMapping,
    },
    schema::{limit_order_datas, market_configs, market_datas, position_datas, tpsl_datas},
    utils::util::{bigdecimal_to_u64, parse_timestamp_secs, standardize_address},
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
    pub liquidation_fee: BigDecimal,
    pub referrer_fee: BigDecimal,

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
        mirage_module_address: &str,
    ) -> anyhow::Result<Option<(MarketCollectionModel, MarketConfigModel)>> {
        if let Some(inner) = &MarketCollection::from_write_resource(write_resource, txn_version, mirage_module_address)? {
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
                    liquidation_fee: inner.config.fees.liquidation_fee.clone(),
                    referrer_fee: inner.config.fees.referrer_fee.clone(),
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

    pub opening_price: BigDecimal,
    pub is_long: bool,
    pub margin_amount: BigDecimal,
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

    pub owner_addr: String,
    pub market_id: String,
    pub position_id: String,

    pub take_profit_price: BigDecimal,
    pub stop_loss_price: BigDecimal,
    pub trigger_payment_amount: BigDecimal,

    pub transaction_timestamp: chrono::NaiveDateTime,
}

impl PositionModel {
    pub fn get_from_write_resource(
        write_resource: &WriteResource,
        txn_version: i64,
        write_set_change_index: i64,
        txn_timestamp: chrono::NaiveDateTime,
        object_metadatas: &ObjectAggregatedDataMapping,
        mirage_module_address: &str,
    ) -> anyhow::Result<Option<Self>> {
        if let Some(inner) = &Position::from_write_resource(write_resource, txn_version, mirage_module_address)? {
            let asset_type = standardize_address(&write_resource.address.to_string());
            if let Some(object_metadata) = object_metadatas.get(&asset_type) {
                return Ok(Some(Self {
                    transaction_version: txn_version,
                    write_set_change_index,
                    owner_addr: object_metadata.object.object_core.get_owner_address(),
                    market_id: inner.market.get_reference_address(),
                    position_id: write_resource.address.clone(),
                    opening_price: inner.opening_price.clone(),
                    is_long: inner.is_long,
                    margin_amount: inner.margin_amount.clone(),
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
        object_metadatas: &ObjectAggregatedDataMapping,
        mirage_module_address: &str,
    ) -> anyhow::Result<Option<Self>> {
        if let Some(inner) = &TpSl::from_write_resource(write_resource, txn_version, mirage_module_address)? {
            let asset_type = standardize_address(&write_resource.address.to_string());
            if let Some(object_metadata) = object_metadatas.get(&asset_type) {
                return Ok(Some(Self {
                    transaction_version: txn_version,
                    write_set_change_index,
                    owner_addr: object_metadata.object.object_core.get_owner_address(),
                    market_id: inner.market.get_reference_address(),
                    position_id: write_resource.address.clone(),
                    take_profit_price: inner.take_profit_price.clone(),
                    stop_loss_price: inner.stop_loss_price.clone(),
                    trigger_payment_amount: inner.trigger_payment_amount.clone(),
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

    pub market_id: String,
    pub position_id: String,
    pub owner_addr: String,
    pub limit_order_id: BigDecimal,

    pub is_increase: bool,
    pub position_size: BigDecimal,
    pub margin: BigDecimal,
    pub trigger_price: BigDecimal,
    pub triggers_above: bool,
    pub trigger_payment: BigDecimal,
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
        object_metadatas: &ObjectAggregatedDataMapping,
        mirage_module_address: &str,
    ) -> anyhow::Result<Option<Vec<LimitOrderModel>>> {
        if let Some(inner) = &LimitOrders::from_write_resource(write_resource, txn_version, mirage_module_address)? {
            let asset_type = standardize_address(&write_resource.address.to_string());
            if let Some(object_metadata) = object_metadatas.get(&asset_type) {
                let mut result = Vec::new();
                result.reserve_exact(inner.orders.len());

                for order in &inner.orders {
                    result.push(LimitOrderModel {
                        transaction_version: txn_version,
                        write_set_change_index,
                        owner_addr: object_metadata.object.object_core.get_owner_address(),
                        market_id: inner.market.get_reference_address(),
                        position_id: write_resource.address.clone(),
                        limit_order_id: order.id.clone(),
                        is_increase: order.is_increase,
                        position_size: order.position_size.clone(),
                        margin: order.margin_amount.clone(),
                        trigger_price: order.trigger_price.clone(),
                        triggers_above: order.triggers_above,
                        trigger_payment: order.trigger_payment_amount.clone(),
                        max_price_slippage: order.max_price_slippage.clone(),
                        expiration: order.expiration.clone(),
                        transaction_timestamp: txn_timestamp,
                    })
                }
                return Ok(Some(result));
            } else {
                // ObjectCore should not be missing, returning from entire function early
                return Ok(None);
            }
        }
        Ok(None)
    }
}
