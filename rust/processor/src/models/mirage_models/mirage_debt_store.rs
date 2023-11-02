// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

// This is required because a diesel macro makes clippy sad
#![allow(clippy::extra_unused_lifetimes)]
#![allow(clippy::unused_unit)]

use crate::{
    models::mirage_models::mirage_utils::MirageDebtStore, schema::mirage_debt_store_datas,
    utils::util::standardize_address,
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
    pub asset_type: String,
    pub debt_elastic: BigDecimal,
    pub debt_base: BigDecimal,
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
        if let Some(inner) = &MirageDebtStore::from_write_resource(write_resource, txn_version, mirage_module_address)? {
            // the new coin type
            let asset_type = standardize_address(&write_resource.address.to_string());
            return Ok(Some(Self {
                transaction_version: txn_version,
                write_set_change_index,
                asset_type: asset_type.clone(),
                transaction_timestamp: txn_timestamp,
                debt_elastic: inner.debt.elastic.clone(),
                debt_base: inner.debt.base.clone(),
            }));
        }
        Ok(None)
    }
}
