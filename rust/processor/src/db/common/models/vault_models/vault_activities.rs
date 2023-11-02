// Copyright © Mirage Protocol

// This is required because a diesel macro makes clippy sad
#![allow(clippy::extra_unused_lifetimes)]
#![allow(clippy::unused_unit)]

use super::vault_events::VaultEvent;
use crate::{
    schema::vault_activities,
    utils::util::{parse_timestamp, ObjectOwnerMapping},
};
use aptos_protos::transaction::v1::{
    transaction::TxnData, Event as EventPB, Transaction as TransactionPB,
};
use bigdecimal::BigDecimal;
use field_count::FieldCount;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, FieldCount, Identifiable, Insertable, Serialize)]
#[diesel(primary_key(transaction_version, event_index, event_sequence_number,))]
#[diesel(table_name = vault_activities)]
pub struct VaultActivityModel {
    pub transaction_version: i64,
    pub event_creation_number: i64,
    pub event_sequence_number: i64,
    pub event_index: i64,

    pub collection_id: String,
    pub event_type: String,

    pub vault_id: Option<String>,
    pub owner_addr: Option<String>,

    pub collateral_amount: Option<BigDecimal>,
    pub borrow_amount: Option<BigDecimal>,
    pub fee_amount: Option<BigDecimal>,
    pub socialized_amount: Option<BigDecimal>,
    pub collateralization_rate_before: Option<BigDecimal>,
    pub collateralization_rate_after: Option<BigDecimal>,
    pub new_interest_per_second: Option<BigDecimal>,

    pub transaction_timestamp: chrono::NaiveDateTime,
}

/// A simplified VaultActivity (excluded common fields) to reduce code duplication
struct VaultActivityHelper {
    pub event_type: String,
    pub collection_id: String,

    pub vault_id: Option<String>,
    pub owner_addr: Option<String>,

    pub collateral_amount: Option<BigDecimal>,
    pub borrow_amount: Option<BigDecimal>,
    pub fee_amount: Option<BigDecimal>,
    pub socialized_amount: Option<BigDecimal>,
    pub collateralization_rate_before: Option<BigDecimal>,
    pub collateralization_rate_after: Option<BigDecimal>,
    pub new_interest_per_second: Option<BigDecimal>,
}

impl VaultActivityModel {
    pub fn from_transaction(
        transaction: &TransactionPB,
        object_owners: &ObjectOwnerMapping,
        mirage_module_address: &str,
    ) -> Vec<Self> {
        let mut vault_activities: Vec<VaultActivityModel> = Vec::new();

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
            .expect("Transaction timestamp doesn't exist!");
        let txn_timestamp = parse_timestamp(txn_timestamp, txn_version);

        for (index, event) in events.iter().enumerate() {
            let maybe_vault_event = VaultEvent::from_event(event, txn_version, mirage_module_address).unwrap();

            if let Some(vault_event) = maybe_vault_event {
                vault_activities.push(Self::from_parsed_event(
                    event,
                    &vault_event,
                    txn_version,
                    txn_timestamp,
                    index as i64,
                    object_owners,
                ));
            }
        }
        vault_activities
    }

    fn from_parsed_event(
        event: &EventPB,
        parsed_event: &VaultEvent,
        txn_version: i64,
        txn_timestamp: chrono::NaiveDateTime,
        event_index: i64,
        object_owners: &ObjectOwnerMapping,
    ) -> Self {
        let event_creation_number = event.key.as_ref().unwrap().creation_number as i64;
        let event_sequence_number = event.sequence_number as i64;

        let vault_activity_helper = match parsed_event {
            VaultEvent::AddCollateralEvent(inner) => {
                let owner_addr = object_owners.get(&inner.vault.get_reference_address());

                VaultActivityHelper {
                    event_type: String::from("AddCollateralEvent"),
                    collection_id: inner.collection.get_reference_address(),
                    vault_id: Some(inner.vault.get_reference_address()),
                    owner_addr: owner_addr.cloned(),
                    collateral_amount: Some(inner.collateral_amount.clone()),
                    borrow_amount: None,
                    fee_amount: None,
                    socialized_amount: None,
                    collateralization_rate_before: None,
                    collateralization_rate_after: None,
                    new_interest_per_second: None,
                }
            },
            VaultEvent::RemoveCollateralEvent(inner) => {
                let owner_addr = object_owners.get(&inner.vault.get_reference_address());

                VaultActivityHelper {
                    event_type: String::from("RemoveCollateralEvent"),
                    collection_id: inner.collection.get_reference_address(),
                    vault_id: Some(inner.vault.get_reference_address()),
                    owner_addr: owner_addr.cloned(),
                    collateral_amount: Some(inner.collateral_amount.clone()),
                    borrow_amount: None,
                    fee_amount: None,
                    socialized_amount: None,
                    collateralization_rate_before: None,
                    collateralization_rate_after: None,
                    new_interest_per_second: None,
                }
            },
            VaultEvent::BorrowEvent(inner) => {
                let owner_addr = object_owners.get(&inner.vault.get_reference_address());

                VaultActivityHelper {
                    event_type: String::from("BorrowEvent"),
                    collection_id: inner.collection.get_reference_address(),
                    vault_id: Some(inner.vault.get_reference_address()),
                    owner_addr: owner_addr.cloned(),
                    collateral_amount: None,
                    borrow_amount: Some(inner.borrow_amount.clone()),
                    fee_amount: Some(inner.fee_amount.clone()),
                    socialized_amount: None,
                    collateralization_rate_before: None,
                    collateralization_rate_after: None,
                    new_interest_per_second: None,
                }
            },
            VaultEvent::RepayEvent(inner) => {
                let owner_addr = object_owners.get(&inner.vault.get_reference_address());

                VaultActivityHelper {
                    event_type: String::from("RepayEvent"),
                    collection_id: inner.collection.get_reference_address(),
                    vault_id: Some(inner.vault.get_reference_address()),
                    owner_addr: owner_addr.cloned(),
                    collateral_amount: None,
                    borrow_amount: Some(inner.borrow_amount.clone()),
                    fee_amount: Some(inner.fee_amount.clone()),
                    socialized_amount: None,
                    collateralization_rate_before: None,
                    collateralization_rate_after: None,
                    new_interest_per_second: None,
                }
            },
            VaultEvent::LiquidationEvent(inner) => {
                let owner_addr = object_owners.get(&inner.vault.get_reference_address());

                VaultActivityHelper {
                    event_type: String::from("LiquidationEvent"),
                    collection_id: inner.collection.get_reference_address(),
                    vault_id: Some(inner.vault.get_reference_address()),
                    owner_addr: owner_addr.cloned(),
                    collateral_amount: Some(inner.collateral_amount.clone()),
                    borrow_amount: Some(inner.borrow_amount.clone()),
                    fee_amount: Some(inner.protocol_liquidation_fee.clone()),
                    socialized_amount: Some(inner.socialized_amount.clone()),
                    collateralization_rate_before: Some(
                        inner.collateralization_rate_before.clone(),
                    ),
                    collateralization_rate_after: Some(inner.collateralization_rate_after.clone()),
                    new_interest_per_second: None,
                }
            },
            VaultEvent::InterestRateChangeEvent(inner) => VaultActivityHelper {
                event_type: String::from("InterestRateChangeEvent"),
                collection_id: inner.collection.get_reference_address(),
                vault_id: None,
                owner_addr: None,
                collateral_amount: None,
                borrow_amount: None,
                fee_amount: None,
                socialized_amount: None,
                collateralization_rate_before: None,
                collateralization_rate_after: None,
                new_interest_per_second: Some(inner.new_interest_per_second.clone()),
            },
        };

        Self {
            transaction_version: txn_version,
            event_creation_number,
            event_sequence_number,
            event_type: vault_activity_helper.event_type,
            event_index,
            collection_id: vault_activity_helper.collection_id,
            vault_id: vault_activity_helper.vault_id,
            owner_addr: vault_activity_helper.owner_addr,
            collateral_amount: vault_activity_helper.collateral_amount,
            borrow_amount: vault_activity_helper.borrow_amount,
            fee_amount: vault_activity_helper.fee_amount,
            socialized_amount: vault_activity_helper.socialized_amount,
            collateralization_rate_before: vault_activity_helper.collateralization_rate_before,
            collateralization_rate_after: vault_activity_helper.collateralization_rate_after,
            new_interest_per_second: vault_activity_helper.new_interest_per_second,
            transaction_timestamp: txn_timestamp,
        }
    }
}
