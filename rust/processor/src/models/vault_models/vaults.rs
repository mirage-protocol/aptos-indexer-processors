// Copyright © Mirage Protocol

use crate::models::mirage::{hash_types, trunc_type};
use super::vault_utils::{VaultModuleResource};

use crate::schema::vaults;

use aptos_protos::transaction::v1::WriteResource;

use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use field_count::FieldCount;

#[derive(Debug, Deserialize, FieldCount, Identifiable, Insertable, Serialize)]
#[diesel(primary_key(collateral_type, borrow_type))]
#[diesel(table_name = vaults)]
pub struct Vault {
    pub transaction_version: i64,
    pub collateral_type: String,
    pub borrow_type: String,
    pub type_hash: String,
    pub total_collateral: BigDecimal,
    pub borrow_elastic: BigDecimal,
    pub borrow_base: BigDecimal,
    pub global_debt_part: BigDecimal,
    pub interest_per_second: BigDecimal,
    pub last_interest_payment: BigDecimal,
    pub collateralization_rate: BigDecimal,
    pub liquidation_multiplier: BigDecimal,
	pub borrow_fee: BigDecimal,
    pub distribution_part: BigDecimal,
	pub cached_exchange_rate: BigDecimal,
	pub last_interest_update: BigDecimal,
	pub is_emergency: bool,
    pub transaction_timestamp: chrono::NaiveDateTime,
}

impl Vault {
    pub fn from_write_resource(
        write_resource: &WriteResource,
        txn_version: i64,
        txn_timestamp: chrono::NaiveDateTime,
    ) -> anyhow::Result<Option<Self>> {
        match &VaultModuleResource::from_write_resource(write_resource, txn_version)? {
            Some(VaultModuleResource::VaultResource(inner)) => {
                let collateral_type = &write_resource.data.typ.generic_type_params[0].to_string();
                let borrow_type = &write_resource.data.typ.generic_type_params[1].to_string();

                Ok(Some(Self {
                    transaction_version: txn_version,
                    type_hash: hash_types(collateral_type, borrow_type),
                    collateral_type: trunc_type(collateral_type),
                    borrow_type: trunc_type(borrow_type),
                    total_collateral: inner.total_collateral.clone(),
                    borrow_elastic: inner.borrow.elastic.clone(),
                    borrow_base: inner.borrow.base.clone(),
                    global_debt_part: inner.global_debt_part.amount.clone(),
                    interest_per_second: inner.interest_per_second.clone(),
                    last_interest_payment: inner.last_interest_payment.clone(),
                    collateralization_rate: inner.collateralization_rate.clone(),
                    liquidation_multiplier: inner.liquidation_multiplier.clone(),
                    borrow_fee: inner.borrow_fee.clone(),
                    distribution_part: inner.distribution_part.clone(),
                    cached_exchange_rate: inner.cached_exchange_rate.clone(),
                    last_interest_update: inner.last_interest_update.clone(),
                    is_emergency: inner.is_emergency,
                    transaction_timestamp: txn_timestamp,
                }))
            },
            _ => Ok(None)
        }
    }
}
