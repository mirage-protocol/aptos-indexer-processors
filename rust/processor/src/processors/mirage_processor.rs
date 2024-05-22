// Copyright © Mirage Protocol

use super::{ProcessingResult, ProcessorName, ProcessorTrait};
use crate::{
    models::{
       market_models::{
            market_activities::{ClosedLimitOrder, ClosedPosition, ClosedTpsl, MarketActivityModel, OpenLimitOrder, OpenPosition, OpenTpsl, Trade},
            market_datas::{
                LimitOrderModel, MarketCollectionModel, MarketConfigModel, PositionModel, TpSlModel,
            },
        },
        mirage_models::{fee_store::FeeStoreModel, mirage_debt_store::MirageDebtStoreModel},
        object_models::v2_object_utils::{
            ObjectAggregatedData, ObjectAggregatedDataMapping, ObjectWithMetadata,
        },
        vault_models::{
            vault_activities::VaultActivityModel,
            vault_datas::{VaultCollectionModel, VaultConfigModel, VaultModel},
        },
    },
    schema,
    utils::{
        counters::PROCESSOR_UNKNOWN_TYPE_COUNT,
        database::{execute_in_chunks, get_config_table_chunk_size, PgDbPool},
        util::{parse_timestamp, standardize_address},
    },
};
use ahash::AHashMap;
use anyhow::bail;
use aptos_protos::transaction::v1::{transaction::TxnData, write_set_change::Change, Transaction};
use async_trait::async_trait;
use bigdecimal::BigDecimal;
use bigdecimal::ToPrimitive;
use core::hash::Hash;
use diesel::{
    pg::Pg, query_builder::QueryFragment, sql_types::Bool, upsert::excluded, BoolExpressionMethods,
    BoxableExpression, ExpressionMethods,
};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use tracing::error;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MirageProcessorConfig {
    pub mirage_module_address: String,
}

pub struct MirageProcessor {
    connection_pool: PgDbPool,
    config: MirageProcessorConfig,
    per_table_chunk_sizes: AHashMap<String, usize>,
}

impl MirageProcessor {
    pub fn new(
        connection_pool: PgDbPool,
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
            "MirageProcessor {{ connections: {:?}  idle_connections: {:?} mirage module address: {}}}",
            state.connections, state.idle_connections, self.config.mirage_module_address
        )
    }
}

pub async fn insert_to_db(
    conn: PgDbPool,
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
    open_positions: &[OpenPosition],
    closed_positions: &[ClosedPosition],
    open_tpsls: &[OpenTpsl],
    closed_tpsls: &[ClosedTpsl],
    open_limit_orders: &[OpenLimitOrder],
    closed_limit_orders: &[ClosedLimitOrder],
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
    let op = execute_in_chunks(
        conn.clone(),
        insert_open_positions_query,
        open_positions,
        get_config_table_chunk_size::<OpenPosition>("open_position", per_table_chunk_sizes),
    );
    let cp = execute_in_chunks(
        conn.clone(),
        insert_closed_positions_query,
        closed_positions,
        get_config_table_chunk_size::<ClosedPosition>(
            "closed_positions",
            per_table_chunk_sizes,
        ),
    );
    let otp = execute_in_chunks(
        conn.clone(),
        insert_open_tpsls_query,
        open_tpsls,
        get_config_table_chunk_size::<OpenTpsl>("open_tpsls", per_table_chunk_sizes),
    );
    let ctp = execute_in_chunks(
        conn.clone(),
        insert_closed_tpsls_query,
        closed_tpsls,
        get_config_table_chunk_size::<ClosedTpsl>(
            "closed_tpsls",
            per_table_chunk_sizes,
        ),
    );
    let ol = execute_in_chunks(
        conn.clone(),
        insert_open_limit_orders_query,
        open_limit_orders,
        get_config_table_chunk_size::<OpenLimitOrder>("open_limit_orders", per_table_chunk_sizes),
    );
    let cl = execute_in_chunks(
        conn.clone(),
        insert_closed_limit_orders_query,
        closed_limit_orders,
        get_config_table_chunk_size::<ClosedLimitOrder>(
            "closed_limit_orders",
            per_table_chunk_sizes,
        ),
    );
    let dl = execute_in_chunks(
        conn.clone(),
        delete_open_limit_orders_query,
        closed_limit_orders,
        get_config_table_chunk_size::<OpenLimitOrder>("open_limit_orders", per_table_chunk_sizes),
    );
    let dtp = execute_in_chunks(
        conn.clone(),
        delete_open_tpsls_query,
        closed_tpsls,
        get_config_table_chunk_size::<OpenTpsl>("open_tpsls", per_table_chunk_sizes),
    );
    let dop = execute_in_chunks(
        conn.clone(),
        delete_open_positions_query,
        closed_positions,
        get_config_table_chunk_size::<ClosedPosition>("closed_positions", per_table_chunk_sizes),
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
        op_res,
        cp_res,
        otp_res,
        ctp_res,
        ol_res,
        cl_res,
        dop_res,
        dtp_res,
        dl_res,
        ma_res,
    ) = tokio::join!(cfd, cfs, vcd, vc, vd, va, mcd, mc, pd, tpd, lod, td, op, cp, otp, ctp, ol, cl, dop, dtp, dl, ma);

    for res in [
        cfd_res, cfs_res, vcd_res, vc_res, vd_res, va_res, mcd_res, mc_res, pd_res, tpd_res,
        lod_res, td_res, op_res, cp_res, otp_res, ctp_res, ol_res, cl_res, dop_res, dtp_res, dl_res, ma_res,
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

fn insert_open_positions_query(
    items_to_insert: Vec<OpenPosition>,
) -> (
    impl QueryFragment<Pg> + diesel::query_builder::QueryId + Send,
    Option<&'static str>,
) {
    use schema::open_positions::dsl::*;
    (
        diesel::insert_into(schema::open_positions::table)
            .values(items_to_insert)
            .on_conflict(position_id)
            .do_update()
            .set((
                transaction_timestamp.eq(excluded(transaction_timestamp)),
                last_transaction_version.eq(excluded(last_transaction_version)),
                inserted_at.eq(excluded(inserted_at)),
            )),
        Some("WHERE open_positions.last_transaction_version <= excluded.last_transaction_version "),
    )
}

fn insert_closed_positions_query(
    items_to_insert: Vec<ClosedPosition>,
) -> (
    impl QueryFragment<Pg> + diesel::query_builder::QueryId + Send,
    Option<&'static str>,
) {
    use schema::closed_positions::dsl::*;
    (
        diesel::insert_into(schema::closed_positions::table)
            .values(items_to_insert)
            .on_conflict(position_id)
            .do_nothing(),
        None,
    )
}

fn insert_open_tpsls_query(
    items_to_insert: Vec<OpenTpsl>,
) -> (
    impl QueryFragment<Pg> + diesel::query_builder::QueryId + Send,
    Option<&'static str>,
) {
    use schema::open_tpsls::dsl::*;
    (
        diesel::insert_into(schema::open_tpsls::table)
            .values(items_to_insert)
            .on_conflict(position_id)
            .do_update()
            .set((
                transaction_timestamp.eq(excluded(transaction_timestamp)),
                last_transaction_version.eq(excluded(last_transaction_version)),
                inserted_at.eq(excluded(inserted_at)),
            )),
        Some("WHERE open_tpsls.last_transaction_version <= excluded.last_transaction_version "),
    )
}

fn insert_closed_tpsls_query(
    items_to_insert: Vec<ClosedTpsl>,
) -> (
    impl QueryFragment<Pg> + diesel::query_builder::QueryId + Send,
    Option<&'static str>,
) {
    use schema::closed_tpsls::*;
    (
        diesel::insert_into(schema::closed_tpsls::table)
            .values(items_to_insert)
            .on_conflict(position_id)
            .do_nothing(),
        None,
    )
}

fn insert_open_limit_orders_query(
    items_to_insert: Vec<OpenLimitOrder>,
) -> (
    impl QueryFragment<Pg> + diesel::query_builder::QueryId + Send,
    Option<&'static str>,
) {
    use schema::open_limit_orders::dsl::*;
    (
        diesel::insert_into(schema::open_limit_orders::table)
            .values(items_to_insert)
            .on_conflict((position_id, limit_order_id))
            .do_update()
            .set((
                transaction_timestamp.eq(excluded(transaction_timestamp)),
                last_transaction_version.eq(excluded(last_transaction_version)),
                inserted_at.eq(excluded(inserted_at)),
            )),
        Some("WHERE open_limit_orders.last_transaction_version <= excluded.last_transaction_version "),
    )
}

fn insert_closed_limit_orders_query(
    items_to_insert: Vec<ClosedLimitOrder>,
) -> (
    impl QueryFragment<Pg> + diesel::query_builder::QueryId + Send,
    Option<&'static str>,
) {
    use schema::closed_limit_orders::dsl::*;
    (
        diesel::insert_into(schema::closed_limit_orders::table)
            .values(items_to_insert)
            .on_conflict((position_id, limit_order_id))
            .do_nothing(),
        None,
    )
}

fn delete_open_limit_orders_query(
    items_to_insert: Vec<ClosedLimitOrder>,
) -> (
    impl QueryFragment<Pg> + diesel::query_builder::QueryId + Send,
    Option<&'static str>,
) {
    use schema::open_limit_orders;
    let position_ids: Vec<String> = items_to_insert
        .iter()
        .map(|x| x.position_id.clone())
        .collect();
    let limit_order_ids: Vec<BigDecimal> = items_to_insert
        .iter()
        .map(|x| x.limit_order_id.clone())
        .collect();

    if items_to_insert.is_empty() {
        // empty query
        let conditions =
            Box::new(open_limit_orders::position_id.eq(String::from("INVALID ADDRESS")))
                as Box<dyn BoxableExpression<open_limit_orders::table, Pg, SqlType = Bool>>;
        (
            diesel::delete(open_limit_orders::table).filter(conditions),
            None,
        )
    } else {
        // delete closed, open limit orders
        let initial_conditional = open_limit_orders::position_id
            .eq(position_ids[0].to_string())
            .and(open_limit_orders::limit_order_id.eq(limit_order_ids[0].clone()));
        let mut conditions = Box::new(initial_conditional)
            as Box<dyn BoxableExpression<open_limit_orders::table, Pg, SqlType = Bool>>;
        for index in 1..position_ids.len() {
            let position_id = position_ids[index].to_string();
            let limit_order_id = limit_order_ids[index].clone();
            let new_condition = open_limit_orders::position_id
                .eq(position_id)
                .and(open_limit_orders::limit_order_id.eq(limit_order_id));
            conditions = Box::new(conditions.or(new_condition));
        }
        (
            diesel::delete(open_limit_orders::table).filter(conditions),
            None,
        )
    }
}

fn delete_open_positions_query(
    items_to_insert: Vec<ClosedPosition>,
) -> (
    impl QueryFragment<Pg> + diesel::query_builder::QueryId + Send,
    Option<&'static str>,
) {
    use schema::open_positions;
    let position_ids: Vec<String> = items_to_insert
        .iter()
        .map(|x| x.position_id.clone())
        .collect();

    if items_to_insert.is_empty() {
        // empty query
        let conditions =
            Box::new(open_positions::position_id.eq(String::from("INVALID ADDRESS")))
                as Box<dyn BoxableExpression<open_positions::table, Pg, SqlType = Bool>>;
        (
            diesel::delete(open_positions::table).filter(conditions),
            None,
        )
    } else {
        // delete closed, open limit orders
        let initial_conditional = open_positions::position_id
            .eq(position_ids[0].to_string());
        let mut conditions = Box::new(initial_conditional)
            as Box<dyn BoxableExpression<open_positions::table, Pg, SqlType = Bool>>;
        for index in 1..position_ids.len() {
            let position_id = position_ids[index].to_string();
            let new_condition = open_positions::position_id
                .eq(position_id);
            conditions = Box::new(conditions.or(new_condition));
        }
        (
            diesel::delete(open_positions::table).filter(conditions),
            None,
        )
    }
}

fn delete_open_tpsls_query(
    items_to_insert: Vec<ClosedTpsl>,
) -> (
    impl QueryFragment<Pg> + diesel::query_builder::QueryId + Send,
    Option<&'static str>,
) {
    use schema::open_tpsls;
    let position_ids: Vec<String> = items_to_insert
        .iter()
        .map(|x| x.position_id.clone())
        .collect();

    if items_to_insert.is_empty() {
        // empty query
        let conditions =
            Box::new(open_tpsls::position_id.eq(String::from("INVALID ADDRESS")))
                as Box<dyn BoxableExpression<open_tpsls::table, Pg, SqlType = Bool>>;
        (
            diesel::delete(open_tpsls::table).filter(conditions),
            None,
        )
    } else {
        // delete closed, open limit orders
        let initial_conditional = open_tpsls::position_id
            .eq(position_ids[0].to_string());
        let mut conditions = Box::new(initial_conditional)
            as Box<dyn BoxableExpression<open_tpsls::table, Pg, SqlType = Bool>>;
        for index in 1..position_ids.len() {
            let position_id = position_ids[index].to_string();
            let new_condition = open_tpsls::position_id
                .eq(position_id);
            conditions = Box::new(conditions.or(new_condition));
        }
        (
            diesel::delete(open_tpsls::table).filter(conditions),
            None,
        )
    }
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
            open_positions,
            closed_positions,
            open_tpsls,
            closed_tpsls,
            open_limit_orders,
            closed_limit_orders,
            market_activities,
        ) = parse_mirage_protocol(&transactions, &self.config.mirage_module_address).await;

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
            &open_positions,
            &closed_positions,
            &open_tpsls,
            &closed_tpsls,
            &open_limit_orders,
            &closed_limit_orders,
            &market_activities,
            &self.per_table_chunk_sizes,
        )
        .await;

        let db_insertion_duration_in_secs = db_insertion_start.elapsed().as_secs_f64();
        match tx_result {
            Ok(_) => Ok(ProcessingResult {
                start_version,
                end_version,
                processing_duration_in_secs,
                db_insertion_duration_in_secs,
                last_transaction_timestamp,
            }),
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

    fn connection_pool(&self) -> &PgDbPool {
        &self.connection_pool
    }
}

pub async fn parse_mirage_protocol(
    transactions: &[Transaction],
    mirage_module_address: &str,
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
    Vec<OpenPosition>,
    Vec<ClosedPosition>,
    Vec<OpenTpsl>,
    Vec<ClosedTpsl>,
    Vec<OpenLimitOrder>,
    Vec<ClosedLimitOrder>,
    Vec<MarketActivityModel>,
) {
    // Get Metadata for token v2 by object
    // We want to persist this through the entire batch so that even if a token is burned,
    // we can still get the object core metadata for it
    let mut token_v2_metadata_helper: ObjectAggregatedDataMapping = AHashMap::new();

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
    let mut all_open_positions: AHashMap<String, OpenPosition> = AHashMap::new();
    let mut all_closed_positions: Vec<ClosedPosition> = vec![];
    let mut all_open_tpsls: AHashMap<String, OpenTpsl> = AHashMap::new();
    let mut all_closed_tpsls: Vec<ClosedTpsl> = vec![];
    let mut all_open_limit_orders: AHashMap<(String, u64), OpenLimitOrder> = AHashMap::new();
    let mut all_closed_limit_orders: Vec<ClosedLimitOrder> = vec![];
    let mut all_market_activities: Vec<MarketActivityModel> = vec![];

    // Helper function to update the latest transaction in the HashMap
    fn update_latest<T, K>(map: &mut AHashMap<K, T>, items: Vec<T>, get_id: impl Fn(&T) -> K) where K: Hash, K: std::cmp::Eq {
        for item in items {
            let id = get_id(&item);
            map.insert(id, item);
        }
    }

    for txn in transactions {
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

        if let TxnData::User(_) = txn_data {
            let txn_version = txn.version as i64;
            let txn_timestamp = parse_timestamp(txn.timestamp.as_ref().unwrap(), txn_version);
            let transaction_info = txn.info.as_ref().expect("Transaction info doesn't exist!");

            // Need to do a first pass to get all the objects
            for wsc in transaction_info.changes.iter() {
                if let Change::WriteResource(wr) = wsc.change.as_ref().unwrap() {
                    if let Some(object) =
                        ObjectWithMetadata::from_write_resource(wr, txn_version).unwrap()
                    {
                        token_v2_metadata_helper.insert(
                            standardize_address(&wr.address.to_string()),
                            ObjectAggregatedData {
                                aptos_collection: None,
                                fixed_supply: None,
                                object,
                                unlimited_supply: None,
                                concurrent_supply: None,
                                property_map: None,
                                transfer_events: vec![],
                                token: None,
                                fungible_asset_metadata: None,
                                fungible_asset_supply: None,
                                fungible_asset_store: None,
                                token_identifier: None,
                            },
                        );
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
                        &token_v2_metadata_helper,
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
                            mirage_module_address,
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
                        &token_v2_metadata_helper,
                        mirage_module_address,
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
                        &token_v2_metadata_helper,
                        mirage_module_address,
                    )
                    .unwrap()
                    {
                        tpsl_datas.push(tpsl_data);
                    }
                    if let Some(mut limit_orders) = LimitOrderModel::get_from_write_resource(
                        write_resource,
                        txn_version,
                        wsc_index,
                        txn_timestamp,
                        &token_v2_metadata_helper,
                        mirage_module_address,
                    )
                    .unwrap()
                    {
                        all_limit_orders.append(&mut limit_orders);
                    }
                }
            }

            // process events
            let mut vault_activities =
                VaultActivityModel::from_transaction(txn, &token_v2_metadata_helper, mirage_module_address);
            let (
                mut trades,
                open_positions,
                mut closed_positions,
                open_tpsls,
                mut closed_tpsls,
                open_limit_orders,
                mut closed_limit_orders,
                mut market_activities,
            ) = MarketActivityModel::from_transaction(txn, &token_v2_metadata_helper, mirage_module_address);

            update_latest(&mut all_open_positions, open_positions, |pos| pos.position_id.clone());
            update_latest(&mut all_open_limit_orders, open_limit_orders, |pos| (pos.position_id.clone(), pos.limit_order_id.to_u64().expect("invalid limit order id")));
            update_latest(&mut all_open_tpsls, open_tpsls, |pos| pos.position_id.clone());

            all_vault_activities.append(&mut vault_activities);
            all_trades.append(&mut trades);
            all_closed_positions.append(&mut closed_positions);
            all_closed_tpsls.append(&mut closed_tpsls);
            all_closed_limit_orders.append(&mut closed_limit_orders);
            all_market_activities.append(&mut market_activities);
        }
    }

    let mut all_open_positions: Vec<OpenPosition> = all_open_positions.into_values().collect();
    let mut all_open_limit_orders: Vec<OpenLimitOrder> = all_open_limit_orders.into_values().collect();
    let mut all_open_tpsls: Vec<OpenTpsl> = all_open_tpsls.into_values().collect();

    // Sort by PK
    mirage_debt_stores.sort_by(|a, b| a.asset_type.cmp(&b.asset_type));
    fee_stores.sort_by(|a, b| a.asset_type.cmp(&b.asset_type));
    vault_configs.sort_by(|a, b| a.collection_id.cmp(&b.collection_id));
    vault_collection_datas.sort_by(|a, b| a.collection_id.cmp(&b.collection_id));
    vault_datas
        .sort_by(|a, b| (&a.vault_id, &a.collection_id).cmp(&(&b.vault_id, &b.collection_id)));

    all_limit_orders.sort_by(|a, b| {
        (&a.position_id, &a.limit_order_id).cmp(&(&b.position_id, &b.limit_order_id))
    });

    market_configs.sort_by(|a, b| a.market_id.cmp(&b.market_id));
    market_datas.sort_by(|a, b| a.market_id.cmp(&b.market_id));
    position_datas
        .sort_by(|a, b| (&a.position_id, &a.market_id).cmp(&(&b.position_id, &b.market_id)));
    tpsl_datas.sort_by(|a, b| (&a.position_id, &a.market_id).cmp(&(&b.position_id, &b.market_id)));

    all_trades.sort_by(|a, b| a.position_id.cmp(&b.position_id));
    all_closed_limit_orders.sort_by(|a, b| a.limit_order_id.cmp(&b.limit_order_id));

    all_open_positions.sort_by(|a, b| a.position_id.cmp(&b.position_id));
    all_open_tpsls.sort_by(|a, b| a.position_id.cmp(&b.position_id));
    all_open_limit_orders.sort_by(|a, b| (&a.position_id, &a.limit_order_id).cmp(&(&b.position_id, &b.limit_order_id)));

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
        all_open_positions,
        all_closed_positions,
        all_open_tpsls,
        all_closed_tpsls,
        all_open_limit_orders,
        all_closed_limit_orders,
        all_market_activities,
    )
}
