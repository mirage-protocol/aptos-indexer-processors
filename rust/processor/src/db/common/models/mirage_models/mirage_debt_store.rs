// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

// This is required because a diesel macro makes clippy sad
#![allow(clippy::extra_unused_lifetimes)]
#![allow(clippy::unused_unit)]

use crate::{
    db::common::models::mirage_models::mirage_utils::MirageDebtStore,
    schema::mirage_debt_store_datas, utils::util::{bigdecimal_to_u64, parse_timestamp_secs, standardize_address},
};
use aptos_protos::transaction::v1::WriteResource;
use bigdecimal::BigDecimal;
use diesel::prelude::*;
use field_count::FieldCount;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, FieldCount, Identifiable, Insertable, Serialize)]
#[diesel(primary_key(transaction_version, write_set_change_index))]
#[diesel(table_name = mirage_debt_store_datas)]
pub struct MirageDebtStoreModel {
    pub transaction_version: i64,
    pub write_set_change_index: i64,
    pub object_address: String,
    pub debt_elastic: BigDecimal,
    pub debt_base: BigDecimal,
    pub burn_prev_qty: BigDecimal,
    pub burn_cur_qty: BigDecimal,
    pub burn_window_start: chrono::NaiveDateTime,
    pub burn_window_duration_sec: BigDecimal,
    pub burn_max_outflow: BigDecimal,
    pub mint_prev_qty: BigDecimal,
    pub mint_cur_qty: BigDecimal,
    pub mint_window_start: chrono::NaiveDateTime,
    pub mint_window_duration_sec: BigDecimal,
    pub mint_max_outflow: BigDecimal,
    pub transaction_timestamp: chrono::NaiveDateTime,
}

impl MirageDebtStoreModel {
    /// Fungible asset is part of an object and we need to get the object first to get owner address
    pub fn from_write_resource(
        write_resource: &WriteResource,
        write_set_change_index: i64,
        txn_version: i64,
        txn_timestamp: chrono::NaiveDateTime,
        mirage_module_address: &str,
    ) -> anyhow::Result<Option<Self>> {
        if let Some(inner) = &MirageDebtStore::from_write_resource(
            write_resource,
            txn_version,
            mirage_module_address,
        )? {
            // the new coin type
            let object_address = standardize_address(&write_resource.address.to_string());
            return Ok(Some(Self {
                transaction_version: txn_version,
                write_set_change_index,
                object_address,
                debt_elastic: inner.debt.elastic.clone(),
                debt_base: inner.debt.base.clone(),
                burn_prev_qty: inner.burn_rate_limiter.prev_qty.clone(),
                burn_cur_qty: inner.burn_rate_limiter.cur_qty.clone(),
                burn_window_start: parse_timestamp_secs(bigdecimal_to_u64(&inner.burn_rate_limiter.window_start_sec), txn_version),
                burn_window_duration_sec: inner.burn_rate_limiter.config.window_duration_sec.clone(),
                burn_max_outflow: inner.burn_rate_limiter.config.max_outflow.clone(),
                mint_prev_qty: inner.mint_rate_limiter.prev_qty.clone(),
                mint_cur_qty: inner.mint_rate_limiter.cur_qty.clone(),
                mint_window_start: parse_timestamp_secs(bigdecimal_to_u64(&inner.mint_rate_limiter.window_start_sec), txn_version),
                mint_window_duration_sec: inner.mint_rate_limiter.config.window_duration_sec.clone(),
                mint_max_outflow: inner.mint_rate_limiter.config.max_outflow.clone(),
                transaction_timestamp: txn_timestamp,
            }));
        }
        Ok(None)
    }
}
