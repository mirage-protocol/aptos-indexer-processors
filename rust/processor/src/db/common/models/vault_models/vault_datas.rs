// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

// This is required because a diesel macro makes clippy sad
#![allow(clippy::extra_unused_lifetimes)]
#![allow(clippy::unused_unit)]

use crate::{
    db::common::models::vault_models::vault_utils::{Vault, VaultCollection},
    schema::{vault_collection_configs, vault_collection_datas, vault_datas},
    utils::util::{
        bigdecimal_to_u64, parse_timestamp_secs, standardize_address, ObjectOwnerMapping,
    },
};
use aptos_protos::transaction::v1::WriteResource;
use bigdecimal::BigDecimal;
use diesel::prelude::*;
use field_count::FieldCount;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, FieldCount, Identifiable, Insertable, Serialize)]
#[diesel(primary_key(transaction_version, write_set_change_index))]
#[diesel(table_name = vault_collection_configs)]
pub struct VaultConfigModel {
    pub transaction_version: i64,
    pub write_set_change_index: i64,

    pub collection_id: String,
    pub collateral_token_id: String,
    pub borrow_token_id: String,

    pub interest_per_second: BigDecimal,
    pub initial_collateralization_rate: BigDecimal,
    pub maintenance_collateralization_rate: BigDecimal,
    pub liquidation_multiplier: BigDecimal,
    pub borrow_fee: BigDecimal,
    pub protocol_liquidation_fee: BigDecimal,
    pub min_collateral_amount: BigDecimal,
    pub max_collection_debt_amount: BigDecimal,
    pub liquidation_rate_limiter_max_outflow: BigDecimal,

    pub transaction_timestamp: chrono::NaiveDateTime,
}

#[derive(Clone, Debug, Deserialize, FieldCount, Identifiable, Insertable, Serialize)]
#[diesel(primary_key(transaction_version, write_set_change_index))]
#[diesel(table_name = vault_collection_datas)]
pub struct VaultCollectionModel {
    pub transaction_version: i64,
    pub write_set_change_index: i64,

    pub collection_id: String,
    pub collateral_token_id: String,
    pub borrow_token_id: String,

    pub total_collateral: BigDecimal,
    pub borrow_elastic: BigDecimal,

    pub borrow_base: BigDecimal,
    pub global_debt_part: BigDecimal,
    pub last_interest_payment: chrono::NaiveDateTime,
    pub cached_exchange_rate: BigDecimal,
    pub last_interest_update: chrono::NaiveDateTime,
    pub is_emergency: bool,

    pub liquidation_rate_limiter_prev_qty: BigDecimal,
    pub liquidation_rate_limiter_cur_qty: BigDecimal,
    pub liquidation_rate_limiter_window_start: chrono::NaiveDateTime,

    pub transaction_timestamp: chrono::NaiveDateTime,
}

#[derive(Clone, Debug, Deserialize, FieldCount, Identifiable, Insertable, Serialize)]
#[diesel(primary_key(transaction_version, write_set_change_index))]
#[diesel(table_name = vault_datas)]
pub struct VaultModel {
    pub transaction_version: i64,
    pub write_set_change_index: i64,

    pub owner_addr: String,
    pub collection_id: String,
    pub vault_id: String,

    pub collateral_amount: BigDecimal,
    pub borrow_part: BigDecimal,

    pub transaction_timestamp: chrono::NaiveDateTime,
}

impl VaultCollectionModel {
    /// Fungible asset is part of an object and we need to get the object first to get owner address
    pub fn from_write_resource(
        write_resource: &WriteResource,
        write_set_change_index: i64,
        txn_version: i64,
        txn_timestamp: chrono::NaiveDateTime,
        mirage_module_address: &str,
    ) -> anyhow::Result<Option<(VaultCollectionModel, VaultConfigModel)>> {
        if let Some(inner) = &VaultCollection::from_write_resource(
            write_resource,
            txn_version,
            mirage_module_address,
        )? {
            // the new coin type
            let collection_id = standardize_address(&write_resource.address.to_string());

            return Ok(Some((
                VaultCollectionModel {
                    transaction_version: txn_version,
                    write_set_change_index,
                    collection_id: collection_id.clone(),
                    collateral_token_id: inner.collateral_token.get_reference_address(),
                    borrow_token_id: inner.borrow_token.get_reference_address(),
                    total_collateral: inner.total_collateral.clone(),
                    borrow_elastic: inner.borrow.elastic.clone(),
                    borrow_base: inner.borrow.base.clone(),
                    global_debt_part: inner.global_debt_part.amount.clone(),
                    last_interest_payment: parse_timestamp_secs(
                        bigdecimal_to_u64(&inner.last_interest_payment),
                        txn_version,
                    ),
                    cached_exchange_rate: inner.cached_exchange_rate.clone(),
                    last_interest_update: parse_timestamp_secs(
                        bigdecimal_to_u64(&inner.last_interest_update),
                        txn_version,
                    ),
                    is_emergency: inner.is_emergency,
                    liquidation_rate_limiter_prev_qty: inner.liquidation_rate_limiter.prev_qty.clone(),
                    liquidation_rate_limiter_cur_qty: inner.liquidation_rate_limiter.cur_qty.clone(),
                    liquidation_rate_limiter_window_start: parse_timestamp_secs(
                        bigdecimal_to_u64(&inner.liquidation_rate_limiter.window_start_sec),
                        txn_version,
                    ),
                    transaction_timestamp: txn_timestamp,
                },
                VaultConfigModel {
                    transaction_version: txn_version,
                    write_set_change_index,
                    collection_id,
                    collateral_token_id: inner.collateral_token.get_reference_address(),
                    borrow_token_id: inner.borrow_token.get_reference_address(),
                    interest_per_second: inner.config.interest_per_second.clone(),
                    initial_collateralization_rate: inner
                        .config
                        .initial_collateralization_rate
                        .clone(),
                    maintenance_collateralization_rate: inner
                        .config
                        .maintenance_collateralization_rate
                        .clone(),
                    liquidation_multiplier: inner.config.liquidation_multiplier.clone(),
                    borrow_fee: inner.config.borrow_fee.clone(),
                    protocol_liquidation_fee: inner.config.protocol_liquidation_fee.clone(),
                    min_collateral_amount: inner.config.min_collateral_amount.clone(),
                    max_collection_debt_amount: inner.config.max_collection_debt_amount.clone(),
                    liquidation_rate_limiter_max_outflow: inner.liquidation_rate_limiter.config.max_outflow.clone(),
                    transaction_timestamp: txn_timestamp,
                },
            )));
        };
        Ok(None)
    }
}

impl VaultModel {
    pub fn get_from_write_resource(
        write_resource: &WriteResource,
        txn_version: i64,
        write_set_change_index: i64,
        txn_timestamp: chrono::NaiveDateTime,
        object_owners: &ObjectOwnerMapping,
        mirage_module_address: &str,
    ) -> anyhow::Result<Option<Self>> {
        if let Some(inner) =
            &Vault::from_write_resource(write_resource, txn_version, mirage_module_address)?
        {
            let vault_id = standardize_address(&write_resource.address.to_string());
            if let Some(owner_addr) = object_owners.get(&vault_id) {
                return Ok(Some(Self {
                    transaction_version: txn_version,
                    write_set_change_index,
                    owner_addr: owner_addr.clone(),
                    collection_id: inner.collection.get_reference_address(),
                    vault_id,
                    collateral_amount: inner.collateral_amount.clone(),
                    borrow_part: inner.borrow_part.amount.clone(),
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
