
use super::{ProcessorName, ProcessorTrait, DefaultProcessingResult};
use crate::{
    db::common::models::{
       market_models::{
            market_activities::{MarketActivityModel, CurrentLimitOrder, CurrentPosition, CurrentTpsl, Trade},
            market_datas::{
                LimitOrderModel, MarketCollectionModel, MarketConfigModel, PositionModel, TpSlModel,
            },
            market_utils::{Strategy, StrategyObjectMapping}
        },
        mirage_models::{fee_store::FeeStoreModel, mirage_debt_store::MirageDebtStoreModel},
        object_models::v2_object_utils::ObjectWithMetadata,
        token_v2_models::v2_token_utils::V2TokenEvent,
        vault_models::{
            vault_activities::VaultActivityModel,
            vault_datas::{VaultCollectionModel, VaultConfigModel, VaultModel},
        },
    },
    schema,
    utils::{
        counters::PROCESSOR_UNKNOWN_TYPE_COUNT,
        database::{execute_in_chunks, get_config_table_chunk_size, ArcDbPool},
        util::{parse_timestamp, standardize_address, ObjectOwnerMapping},
    },
    gap_detectors::ProcessingResult,
};

use aptos_types::account_address::{create_resource_address, AccountAddress};
use ahash::AHashMap;
use anyhow::bail;
use aptos_protos::transaction::v1::{transaction::TxnData, write_set_change::Change, Transaction};
use async_trait::async_trait;
use core::hash::Hash;
use diesel::{pg::Pg, query_builder::QueryFragment, upsert::excluded, ExpressionMethods};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use tracing::error;
use crate::db::postgres::models::resources::FromWriteResource;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MirageProcessorConfig {
    pub deployer_address: String,
}

pub struct MirageProcessor {
    connection_pool: ArcDbPool,
    config: MirageProcessorConfig,
    per_table_chunk_sizes: AHashMap<String, usize>,
}

impl MirageProcessor {
    pub fn new(
        connection_pool: ArcDbPool,
        config: MirageProcessorConfig,
        per_table_chunk_sizes: AHashMap<String, usize>,
    ) -> Self {
        Self {
            connection_pool,
            config,
            per_table_chunk_sizes,
        }
    }
}

impl Debug for MirageProcessor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state = &self.connection_pool.state();
        write!(
            f,
            "MirageProcessor {{ connections: {:?}  idle_connections: {:?} deployer address: {}}}",
            state.connections, state.idle_connections, self.config.deployer_address
        )
    }
}

pub async fn insert_to_db(
    conn: ArcDbPool,
    name: &'static str,
    start_version: u64,
    end_version: u64,
    mirage_debt_stores: &[MirageDebtStoreModel],
    fee_stores: &[FeeStoreModel],
    vault_collection_datas: &[VaultCollectionModel],
    vault_configs: &[VaultConfigModel],
    vault_datas: &[VaultModel],
    vault_activities: &[VaultActivityModel],
    market_collection_datas: &[MarketCollectionModel],
    market_configs: &[MarketConfigModel],
    position_datas: &[PositionModel],
    tpsl_datas: &[TpSlModel],
    limit_order_datas: &[LimitOrderModel],
    trades: &[Trade],
    current_positions: &[CurrentPosition],
    current_tpsls: &[CurrentTpsl],
    current_limit_orders: &[CurrentLimitOrder],
    market_activities: &[MarketActivityModel],
    per_table_chunk_sizes: &AHashMap<String, usize>,
) -> Result<(), diesel::result::Error> {
    tracing::trace!(
        name = name,
        start_version = start_version,
        end_version = end_version,
        "Inserting to db",
    );

    let cfd = execute_in_chunks(
        conn.clone(),
        insert_mirage_debt_store_query,
        mirage_debt_stores,
        get_config_table_chunk_size::<MirageDebtStoreModel>(
            "mirage_debt_store_datas",
            per_table_chunk_sizes,
        ),
    );
    let cfs = execute_in_chunks(
        conn.clone(),
        insert_fee_store_query,
        fee_stores,
        get_config_table_chunk_size::<FeeStoreModel>("fee_store_datas", per_table_chunk_sizes),
    );
    let vcd = execute_in_chunks(
        conn.clone(),
        insert_vault_collection_datas_query,
        vault_collection_datas,
        get_config_table_chunk_size::<VaultCollectionModel>(
            "vault_collection_datas",
            per_table_chunk_sizes,
        ),
    );
    let vc = execute_in_chunks(
        conn.clone(),
        insert_vault_configs_query,
        vault_configs,
        get_config_table_chunk_size::<VaultCollectionModel>("vault_configs", per_table_chunk_sizes),
    );
    let vd = execute_in_chunks(
        conn.clone(),
        insert_vault_datas_configs_query,
        vault_datas,
        get_config_table_chunk_size::<VaultModel>("vault_datas", per_table_chunk_sizes),
    );
    let va = execute_in_chunks(
        conn.clone(),
        insert_vault_activities_query,
        vault_activities,
        get_config_table_chunk_size::<VaultActivityModel>(
            "vault_activities",
            per_table_chunk_sizes,
        ),
    );
    let mcd = execute_in_chunks(
        conn.clone(),
        insert_market_collection_datas_query,
        market_collection_datas,
        get_config_table_chunk_size::<MarketCollectionModel>(
            "market_collection_datas",
            per_table_chunk_sizes,
        ),
    );
    let mc = execute_in_chunks(
        conn.clone(),
        insert_market_configs_query,
        market_configs,
        get_config_table_chunk_size::<MarketConfigModel>("market_configs", per_table_chunk_sizes),
    );
    let pd = execute_in_chunks(
        conn.clone(),
        insert_position_datas_configs_query,
        position_datas,
        get_config_table_chunk_size::<PositionModel>("position_datas", per_table_chunk_sizes),
    );
    let tpd = execute_in_chunks(
        conn.clone(),
        insert_tpsl_datas_configs_query,
        tpsl_datas,
        get_config_table_chunk_size::<TpSlModel>("tpsl_datas", per_table_chunk_sizes),
    );
    let lod = execute_in_chunks(
        conn.clone(),
        insert_limit_order_datas_query,
        limit_order_datas,
        get_config_table_chunk_size::<LimitOrderModel>("limit_order_datas", per_table_chunk_sizes),
    );
    let td = execute_in_chunks(
        conn.clone(),
        insert_trade_datas_query,
        trades,
        get_config_table_chunk_size::<Trade>("trade_datas", per_table_chunk_sizes),
    );
    let cp = execute_in_chunks(
        conn.clone(),
        insert_current_positions_query,
        current_positions,
        get_config_table_chunk_size::<CurrentPosition>("current_position", per_table_chunk_sizes),
    );
    let ctp = execute_in_chunks(
        conn.clone(),
        insert_current_tpsls_query,
        current_tpsls,
        get_config_table_chunk_size::<CurrentTpsl>("current_tpsls", per_table_chunk_sizes),
    );
    let cl = execute_in_chunks(
        conn.clone(),
        insert_current_limit_orders_query,
        current_limit_orders,
        get_config_table_chunk_size::<CurrentLimitOrder>("current_limit_orders", per_table_chunk_sizes),
    );
    let ma = execute_in_chunks(
        conn.clone(),
        insert_market_activities_query,
        market_activities,
        get_config_table_chunk_size::<MarketActivityModel>(
            "market_activities",
            per_table_chunk_sizes,
        ),
    );

    let (
        cfd_res,
        cfs_res,
        vcd_res,
        vc_res,
        vd_res,
        va_res,
        mcd_res,
        mc_res,
        pd_res,
        tpd_res,
        lod_res,
        td_res,
        cp_res,
        ctp_res,
        cl_res,
        ma_res,
    ) = tokio::join!(cfd, cfs, vcd, vc, vd, va, mcd, mc, pd, tpd, lod, td, cp, ctp, cl, ma);

    for res in [
        cfd_res, cfs_res, vcd_res, vc_res, vd_res, va_res, mcd_res, mc_res, pd_res, tpd_res,
        lod_res, td_res, cp_res, ctp_res, cl_res, ma_res,
    ] {
        res?;
    }

    Ok(())
}

fn insert_fee_store_query(
    items_to_insert: Vec<FeeStoreModel>,
) -> (
    impl QueryFragment<Pg> + diesel::query_builder::QueryId + Send,
    Option<&'static str>,
) {
    use schema::fee_store_datas::dsl::*;

    (
        diesel::insert_into(schema::fee_store_datas::table)
            .values(items_to_insert)
            .on_conflict((transaction_version, write_set_change_index))
            .do_update()
            .set((
                net_accumulated_fees.eq(excluded(net_accumulated_fees)),
                inserted_at.eq(excluded(inserted_at)),
            )),
        None,
    )
}

fn insert_mirage_debt_store_query(
    items_to_insert: Vec<MirageDebtStoreModel>,
) -> (
    impl QueryFragment<Pg> + diesel::query_builder::QueryId + Send,
    Option<&'static str>,
) {
    use schema::mirage_debt_store_datas::dsl::*;

    (
        diesel::insert_into(schema::mirage_debt_store_datas::table)
            .values(items_to_insert)
            .on_conflict((transaction_version, write_set_change_index))
            .do_update()
            .set((
                debt_elastic.eq(excluded(debt_elastic)),
                debt_base.eq(excluded(debt_base)),
                inserted_at.eq(excluded(inserted_at)),
            )),
        None,
    )
}

fn insert_vault_activities_query(
    items_to_insert: Vec<VaultActivityModel>,
) -> (
    impl QueryFragment<Pg> + diesel::query_builder::QueryId + Send,
    Option<&'static str>,
) {
    use schema::vault_activities::dsl::*;

    (
        diesel::insert_into(schema::vault_activities::table)
            .values(items_to_insert)
            .on_conflict((
                transaction_version,
                event_creation_number,
                event_sequence_number,
                event_index,
            ))
            .do_nothing(),
        None,
    )
}

fn insert_market_collection_datas_query(
    items_to_insert: Vec<MarketCollectionModel>,
) -> (
    impl QueryFragment<Pg> + diesel::query_builder::QueryId + Send,
    Option<&'static str>,
) {
    use schema::market_datas::dsl::*;

    (
        diesel::insert_into(schema::market_datas::table)
            .values(items_to_insert)
            .on_conflict((transaction_version, write_set_change_index))
            .do_nothing(),
        None,
    )
}

fn insert_market_configs_query(
    items_to_insert: Vec<MarketConfigModel>,
) -> (
    impl QueryFragment<Pg> + diesel::query_builder::QueryId + Send,
    Option<&'static str>,
) {
    use schema::market_configs::dsl::*;

    (
        diesel::insert_into(schema::market_configs::table)
            .values(items_to_insert)
            .on_conflict((transaction_version, write_set_change_index))
            .do_nothing(),
        None,
    )
}

fn insert_position_datas_configs_query(
    items_to_insert: Vec<PositionModel>,
) -> (
    impl QueryFragment<Pg> + diesel::query_builder::QueryId + Send,
    Option<&'static str>,
) {
    use schema::position_datas::dsl::*;

    (
        diesel::insert_into(schema::position_datas::table)
            .values(items_to_insert)
            .on_conflict((transaction_version, write_set_change_index))
            .do_nothing(),
        None,
    )
}

fn insert_tpsl_datas_configs_query(
    items_to_insert: Vec<TpSlModel>,
) -> (
    impl QueryFragment<Pg> + diesel::query_builder::QueryId + Send,
    Option<&'static str>,
) {
    use schema::tpsl_datas::dsl::*;

    (
        diesel::insert_into(schema::tpsl_datas::table)
            .values(items_to_insert)
            .on_conflict((transaction_version, write_set_change_index))
            .do_nothing(),
        None,
    )
}

fn insert_trade_datas_query(
    items_to_insert: Vec<Trade>,
) -> (
    impl QueryFragment<Pg> + diesel::query_builder::QueryId + Send,
    Option<&'static str>,
) {
    use schema::trade_datas::dsl::*;
    (
        diesel::insert_into(schema::trade_datas::table)
            .values(items_to_insert)
            .on_conflict((position_id, transaction_version))
            .do_nothing(),
        None,
    )
}

fn insert_current_positions_query(
    items_to_insert: Vec<CurrentPosition>,
) -> (
    impl QueryFragment<Pg> + diesel::query_builder::QueryId + Send,
    Option<&'static str>,
) {
    use schema::current_positions::dsl::*;
    (
        diesel::insert_into(schema::current_positions::table)
            .values(items_to_insert)
            .on_conflict(position_id)
            .do_update()
            .set((
                transaction_timestamp.eq(excluded(transaction_timestamp)),
                last_transaction_version.eq(excluded(last_transaction_version)),
                is_closed.eq(excluded(is_closed)),
                event_index.eq(excluded(event_index)),
                inserted_at.eq(excluded(inserted_at)),
            )),
        Some("WHERE current_positions.last_transaction_version < excluded.last_transaction_version 
            OR (current_positions.last_transaction_version = excluded.last_transaction_version 
                AND current_positions.event_index <= excluded.event_index)")
    )
}

fn insert_current_tpsls_query(
    items_to_insert: Vec<CurrentTpsl>,
) -> (
    impl QueryFragment<Pg> + diesel::query_builder::QueryId + Send,
    Option<&'static str>,
) {
    use schema::current_tpsls::dsl::*;
    (
        diesel::insert_into(schema::current_tpsls::table)
            .values(items_to_insert)
            .on_conflict(position_id)
            .do_update()
            .set((
                transaction_timestamp.eq(excluded(transaction_timestamp)),
                last_transaction_version.eq(excluded(last_transaction_version)),
                is_closed.eq(excluded(is_closed)),
                event_index.eq(excluded(event_index)),
                inserted_at.eq(excluded(inserted_at)),
            )),
        Some("WHERE current_tpsls.last_transaction_version < excluded.last_transaction_version 
            OR (current_tpsls.last_transaction_version = excluded.last_transaction_version 
                AND current_tpsls.event_index <= excluded.event_index)")
    )
}

fn insert_current_limit_orders_query(
    items_to_insert: Vec<CurrentLimitOrder>,
) -> (
    impl QueryFragment<Pg> + diesel::query_builder::QueryId + Send,
    Option<&'static str>,
) {
    use schema::current_limit_orders::dsl::*;
    (
        diesel::insert_into(schema::current_limit_orders::table)
            .values(items_to_insert)
            .on_conflict(strategy_id)
            .do_update()
            .set((
                transaction_timestamp.eq(excluded(transaction_timestamp)),
                last_transaction_version.eq(excluded(last_transaction_version)),
                is_closed.eq(excluded(is_closed)),
                event_index.eq(excluded(event_index)),
                inserted_at.eq(excluded(inserted_at)),
            )),
        Some("WHERE current_limit_orders.last_transaction_version < excluded.last_transaction_version 
            OR (current_limit_orders.last_transaction_version = excluded.last_transaction_version 
                AND current_limit_orders.event_index <= excluded.event_index)")
    )
}

fn insert_market_activities_query(
    items_to_insert: Vec<MarketActivityModel>,
) -> (
    impl QueryFragment<Pg> + diesel::query_builder::QueryId + Send,
    Option<&'static str>,
) {
    use schema::market_activities::dsl::*;

    (
        diesel::insert_into(schema::market_activities::table)
            .values(items_to_insert)
            .on_conflict((
                transaction_version,
                event_creation_number,
                event_sequence_number,
                event_index,
            ))
            .do_nothing(),
        None,
    )
}

fn insert_vault_collection_datas_query(
    items_to_insert: Vec<VaultCollectionModel>,
) -> (
    impl QueryFragment<Pg> + diesel::query_builder::QueryId + Send,
    Option<&'static str>,
) {
    use schema::vault_collection_datas::dsl::*;

    (
        diesel::insert_into(schema::vault_collection_datas::table)
            .values(items_to_insert)
            .on_conflict((transaction_version, write_set_change_index))
            .do_nothing(),
        None,
    )
}

fn insert_vault_configs_query(
    items_to_insert: Vec<VaultConfigModel>,
) -> (
    impl QueryFragment<Pg> + diesel::query_builder::QueryId + Send,
    Option<&'static str>,
) {
    use schema::vault_collection_configs::dsl::*;

    (
        diesel::insert_into(schema::vault_collection_configs::table)
            .values(items_to_insert)
            .on_conflict((transaction_version, write_set_change_index))
            .do_nothing(),
        None,
    )
}

fn insert_vault_datas_configs_query(
    items_to_insert: Vec<VaultModel>,
) -> (
    impl QueryFragment<Pg> + diesel::query_builder::QueryId + Send,
    Option<&'static str>,
) {
    use schema::vault_datas::dsl::*;

    (
        diesel::insert_into(schema::vault_datas::table)
            .values(items_to_insert)
            .on_conflict((transaction_version, write_set_change_index))
            .do_nothing(),
        None,
    )
}

fn insert_limit_order_datas_query(
    items_to_insert: Vec<LimitOrderModel>,
) -> (
    impl QueryFragment<Pg> + diesel::query_builder::QueryId + Send,
    Option<&'static str>,
) {
    use schema::limit_order_datas::dsl::*;

    (
        diesel::insert_into(schema::limit_order_datas::table)
            .values(items_to_insert)
            .on_conflict((transaction_version, write_set_change_index))
            .do_nothing(),
        None,
    )
}

#[async_trait]
impl ProcessorTrait for MirageProcessor {
    fn name(&self) -> &'static str {
        ProcessorName::MirageProcessor.into()
    }

    async fn process_transactions(
        &self,
        transactions: Vec<Transaction>,
        start_version: u64,
        end_version: u64,
        _: Option<u64>,
    ) -> anyhow::Result<ProcessingResult> {
        let processing_start = std::time::Instant::now();
        let last_transaction_timestamp = transactions.last().unwrap().timestamp.clone();

        let (
            mirage_debt_stores,
            fee_stores,
            vault_collection_datas,
            vault_configs,
            vault_datas,
            vault_activities,
            market_collection_datas,
            market_configs,
            position_datas,
            tpsl_datas,
            limit_orders,
            trades,
            current_positions,
            current_tpsls,
            current_limit_orders,
            market_activities,
        ) = parse_mirage_protocol(&transactions, &self.config.deployer_address).await;

        let processing_duration_in_secs = processing_start.elapsed().as_secs_f64();
        let db_insertion_start = std::time::Instant::now();

        let tx_result = insert_to_db(
            self.get_pool(),
            self.name(),
            start_version,
            end_version,
            &mirage_debt_stores,
            &fee_stores,
            &vault_collection_datas,
            &vault_configs,
            &vault_datas,
            &vault_activities,
            &market_collection_datas,
            &market_configs,
            &position_datas,
            &tpsl_datas,
            &limit_orders,
            &trades,
            &current_positions,
            &current_tpsls,
            &current_limit_orders,
            &market_activities,
            &self.per_table_chunk_sizes,
        )
        .await;

        let db_insertion_duration_in_secs = db_insertion_start.elapsed().as_secs_f64();
        match tx_result {
            Ok(_) => Ok(ProcessingResult::DefaultProcessingResult(
                DefaultProcessingResult {
                    start_version,
                    end_version,
                    processing_duration_in_secs,
                    db_insertion_duration_in_secs,
                    last_transaction_timestamp,
                },
            )),
            Err(err) => {
                error!(
                    start_version = start_version,
                    end_version = end_version,
                    processor_name = self.name(),
                    "[Parser] Error inserting transactions to db: {:?}",
                    err
                );
                bail!(format!("Error inserting transactions to db. Processor {}. Start {}. End {}. Error {:?}", self.name(), start_version, end_version, err))
            },
        }

    }

    fn connection_pool(&self) -> &ArcDbPool {
        &self.connection_pool
    }
}

pub async fn parse_mirage_protocol(
    transactions: &[Transaction],
    deployer_address: &str,
) -> (
    Vec<MirageDebtStoreModel>,
    Vec<FeeStoreModel>,
    Vec<VaultCollectionModel>,
    Vec<VaultConfigModel>,
    Vec<VaultModel>,
    Vec<VaultActivityModel>,
    Vec<MarketCollectionModel>,
    Vec<MarketConfigModel>,
    Vec<PositionModel>,
    Vec<TpSlModel>,
    Vec<LimitOrderModel>,
    Vec<Trade>,
    Vec<CurrentPosition>,
    Vec<CurrentTpsl>,
    Vec<CurrentLimitOrder>,
    Vec<MarketActivityModel>,
) {
    let deployer_account_address = AccountAddress::from_hex(deployer_address).expect("Failed to parse deployer address");
    let mirage_module_address = &create_resource_address(deployer_account_address, "MIRAGE".as_bytes()).to_standard_string();
    let market_module_address = &create_resource_address(deployer_account_address, "MIRAGE_MARKET".as_bytes()).to_standard_string();

    let mut mirage_debt_stores = vec![];
    let mut fee_stores = vec![];

    let mut vault_collection_datas = vec![];
    let mut vault_configs = vec![];
    let mut vault_datas = vec![];
    let mut all_vault_activities = vec![];

    let mut all_limit_orders = vec![];

    let mut market_datas = vec![];
    let mut market_configs = vec![];
    let mut position_datas = vec![];
    let mut tpsl_datas = vec![];

    let mut all_trades: Vec<Trade> = vec![];
    let mut all_current_positions: AHashMap<String, CurrentPosition> = AHashMap::new();
    let mut all_current_tpsls: AHashMap<String, CurrentTpsl> = AHashMap::new();
    let mut all_current_limit_orders: AHashMap<String, CurrentLimitOrder> = AHashMap::new();
    let mut all_market_activities: Vec<MarketActivityModel> = vec![];

    // Helper function to update the latest transaction in the HashMap
    fn update_latest<T, K>(map: &mut AHashMap<K, T>, items: Vec<T>, get_id: impl Fn(&T) -> K) where K: Hash, K: std::cmp::Eq {
        for item in items {
            let id = get_id(&item);
            map.insert(id, item);
        }
    }

    for txn in transactions {
        // first pass get object owners and strategy objects
        let mut object_owners: ObjectOwnerMapping = AHashMap::new();
        let mut strategy_objects: StrategyObjectMapping = AHashMap::new();

        let txn_version = txn.version;
        let txn_data = match txn.txn_data.as_ref() {
            Some(data) => data,
            None => {
                PROCESSOR_UNKNOWN_TYPE_COUNT
                    .with_label_values(&["Mirage Processor"])
                    .inc();
                tracing::warn!(
                    transaction_version = txn_version,
                    "Transaction data doesn't exist"
                );
                continue;
            },
        };

        if let TxnData::User(txn_inner) = txn_data {
            let txn_version = txn.version as i64;
            let txn_timestamp = parse_timestamp(txn.timestamp.as_ref().unwrap(), txn_version);
            let transaction_info = txn.info.as_ref().expect("Transaction info doesn't exist!");


            // First pass to get all the object owners from the write_set
            for wsc in transaction_info.changes.iter() {
                if let Change::WriteResource(wr) = wsc.change.as_ref().unwrap() {
                    if let Some(object) =
                        ObjectWithMetadata::from_write_resource(wr).unwrap()
                    {
                        object_owners.insert(standardize_address(&wr.address.to_string()), object.object_core.get_owner_address());
                    }
                    if let Some(strategy) =
                        Strategy::from_write_resource(wr, txn_version, market_module_address).unwrap()
                    {
                        strategy_objects.insert(standardize_address(&wr.address.to_string()), strategy);
                    }

                }
            }

            // Second pass to get all the object owners from Token burn events
            for event in txn_inner.events.iter() {
                if let Ok(Some(V2TokenEvent::Burn(burn_event))) = V2TokenEvent::from_event(event.type_str.as_str(), &event.data, txn_version) {
                    if let Some(previous_owner_address) = burn_event.get_previous_owner_address() {
                        object_owners.insert(burn_event.get_token_address(), previous_owner_address);
                    }
                }
            }

            // Loop to handle all the other changes
            for (index, wsc) in transaction_info.changes.iter().enumerate() {
                if let Change::WriteResource(write_resource) = wsc.change.as_ref().unwrap() {
                    let wsc_index = index as i64;
                    if let Some(fee_store) = FeeStoreModel::from_write_resource(
                        write_resource,
                        wsc_index,
                        txn_version,
                        txn_timestamp,
                        mirage_module_address,
                    )
                    .unwrap_or_else(|e| {
                        tracing::error!(
                            transaction_version = txn_version,
                            index = index,
                                error = ?e,
                            "[Parser] error parsing FeeStore");
                        panic!("[Parser] error parsing FeeStore");
                    }) {
                        fee_stores.push(fee_store);
                    }
                    if let Some(mirage_debt_store) = MirageDebtStoreModel::from_write_resource(
                        write_resource,
                        wsc_index,
                        txn_version,
                        txn_timestamp,
                        mirage_module_address,
                    )
                    .unwrap_or_else(|e| {
                        tracing::error!(
                            transaction_version = txn_version,
                            index = index,
                                error = ?e,
                            "[Parser] error parsing FeeStore");
                        panic!("[Parser] error parsing FeeStore");
                    }) {
                        mirage_debt_stores.push(mirage_debt_store);
                    }
                    if let Some((vault_collection, vault_config)) =
                        VaultCollectionModel::from_write_resource(
                            write_resource,
                            wsc_index,
                            txn_version,
                            txn_timestamp,
                            mirage_module_address,
                        )
                        .unwrap_or_else(|e| {
                            tracing::error!(
                            transaction_version = txn_version,
                            index = index,
                                error = ?e,
                            "[Parser] error parsing VaultCollection");
                            panic!("[Parser] error parsing VaultCollection");
                        })
                    {
                        vault_collection_datas.push(vault_collection);
                        vault_configs.push(vault_config);
                    }
                    if let Some(vault_data) = VaultModel::get_from_write_resource(
                        write_resource,
                        txn_version,
                        wsc_index,
                        txn_timestamp,
                        &object_owners,
                        mirage_module_address,
                    )
                    .unwrap()
                    {
                        vault_datas.push(vault_data);
                    }
                    if let Some((market_collection, market_config)) =
                        MarketCollectionModel::from_write_resource(
                            write_resource,
                            index as i64,
                            txn_version,
                            txn_timestamp,
                            market_module_address,
                        )
                        .unwrap_or_else(|e| {
                            tracing::error!(
                            transaction_version = txn_version,
                            index = index,
                                error = ?e,
                            "[Parser] error parsing MarketCollection");
                            panic!("[Parser] error parsing MarketCollection");
                        })
                    {
                        market_datas.push(market_collection);
                        market_configs.push(market_config);
                    }
                    if let Some(position_data) = PositionModel::get_from_write_resource(
                        write_resource,
                        txn_version,
                        wsc_index,
                        txn_timestamp,
                        &object_owners,
                        market_module_address,
                    )
                    .unwrap()
                    {
                        position_datas.push(position_data);
                    }
                    if let Some(tpsl_data) = TpSlModel::get_from_write_resource(
                        write_resource,
                        txn_version,
                        wsc_index,
                        txn_timestamp,
                        &object_owners,
                        market_module_address,
                    )
                    .unwrap()
                    {
                        tpsl_datas.push(tpsl_data);
                    }
                    if let Some(limit_order) = LimitOrderModel::get_from_write_resource(
                        write_resource,
                        txn_version,
                        wsc_index,
                        txn_timestamp,
                        &object_owners,
                        &strategy_objects,
                        market_module_address,
                    )
                    .unwrap()
                    {
                        all_limit_orders.push(limit_order);
                    }
                }
            }

            // process events
            let mut vault_activities =
                VaultActivityModel::from_transaction(txn, &object_owners, mirage_module_address);
            let (
                mut trades,
                current_positions,
                current_tpsls,
                current_limit_orders,
                mut market_activities,
            ) = MarketActivityModel::from_transaction(txn, &object_owners, mirage_module_address);

            update_latest(&mut all_current_positions, current_positions, |pos| pos.position_id.clone());
            update_latest(&mut all_current_limit_orders, current_limit_orders, |pos| pos.strategy_id.clone());
            update_latest(&mut all_current_tpsls, current_tpsls, |pos| pos.strategy_id.clone());

            all_vault_activities.append(&mut vault_activities);
            all_trades.append(&mut trades);
            all_market_activities.append(&mut market_activities);
        }
    }

    let mut all_current_positions: Vec<CurrentPosition> = all_current_positions.into_values().collect();
    let mut all_current_limit_orders: Vec<CurrentLimitOrder> = all_current_limit_orders.into_values().collect();
    let mut all_current_tpsls: Vec<CurrentTpsl> = all_current_tpsls.into_values().collect();

    // Sort by PK
    mirage_debt_stores.sort_by(|a, b| a.object_address.cmp(&b.object_address));
    fee_stores.sort_by(|a, b| a.object_address.cmp(&b.object_address));
    vault_configs.sort_by(|a, b| a.collection_id.cmp(&b.collection_id));
    vault_collection_datas.sort_by(|a, b| a.collection_id.cmp(&b.collection_id));
    vault_datas
        .sort_by(|a, b| (&a.vault_id, &a.collection_id).cmp(&(&b.vault_id, &b.collection_id)));

    all_limit_orders.sort_by(|a, b| {
        a.strategy_id.cmp(&b.strategy_id)
    });

    market_configs.sort_by(|a, b| a.market_id.cmp(&b.market_id));
    market_datas.sort_by(|a, b| a.market_id.cmp(&b.market_id));
    position_datas
        .sort_by(|a, b| (&a.position_id, &a.market_id).cmp(&(&b.position_id, &b.market_id)));
    tpsl_datas.sort_by(|a, b| a.strategy_id.cmp(&b.strategy_id));

    all_trades.sort_by(|a, b| a.position_id.cmp(&b.position_id));

    all_current_positions.sort_by(|a, b| a.position_id.cmp(&b.position_id));
    all_current_tpsls.sort_by(|a, b| a.position_id.cmp(&b.position_id));
    all_current_limit_orders.sort_by(|a, b| a.strategy_id.cmp(&b.strategy_id));

    (
        mirage_debt_stores,
        fee_stores,
        vault_collection_datas,
        vault_configs,
        vault_datas,
        all_vault_activities,
        market_datas,
        market_configs,
        position_datas,
        tpsl_datas,
        all_limit_orders,
        all_trades,
        all_current_positions,
        all_current_tpsls,
        all_current_limit_orders,
        all_market_activities,
    )
}
