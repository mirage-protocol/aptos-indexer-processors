// Copyright © Mirage Protocol

use super::{ProcessingResult, ProcessorName, ProcessorTrait};
use crate::{
    models::{
        vault_models::{
            vaults::Vault,
            vault_activities::VaultActivity,
            vault_users::VaultUser,
        },
        market_models::{
            limit_orders::LimitOrder,
            markets::{Market, MarketConfig},
            market_activities::{
                MarketActivity,
                Trade,
                OpenLimitOrder,
                ClosedLimitOrder
            },
            traders::{Position, PositionLimit},
        },
        mirage::MIRAGE_ADDRESS,
    },
    schema,
    utils::database::{
        clean_data_for_db, execute_with_better_error, get_chunks, MyDbConnection, PgDbPool,
    },
};

use std::fmt::Debug;
use async_trait::async_trait;
use bigdecimal::BigDecimal;

use anyhow::bail;
use aptos_protos::transaction::v1::Transaction;

use diesel::{result::Error, ExpressionMethods, pg::upsert::excluded};
use field_count::FieldCount;
use tracing::error;

pub struct MirageProcessor {
    connection_pool: PgDbPool,
}

impl MirageProcessor {
    pub fn new(connection_pool: PgDbPool) -> Self {
        Self { connection_pool }
    }
}

impl Debug for MirageProcessor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state = &self.connection_pool.state();
        write!(
            f,
            "MirageProcessor {{ connections: {:?}  idle_connections: {:?} mirage contract: {}}}",
            state.connections, state.idle_connections, MIRAGE_ADDRESS
        )
    }
}

async fn insert_to_db_impl(
    conn: &mut MyDbConnection,
    all_vaults: &[Vault],
    all_vault_users: &[VaultUser],
    all_vault_activities: &[VaultActivity],
    all_markets: &[Market],
    all_market_configs: &[MarketConfig],
    all_positions: &[Position],
    all_trades: &[Trade],
    all_position_limits: &[PositionLimit],
    all_limit_orders: &[LimitOrder],
    all_open_limit_orders: &[OpenLimitOrder],
    all_closed_limit_orders: &[ClosedLimitOrder],
    all_market_activities: &[MarketActivity],
) -> Result<(), diesel::result::Error> {
    insert_vaults(conn, all_vaults).await?;
    insert_vault_user(conn, all_vault_users).await?;
    insert_vault_activities(conn, all_vault_activities).await?;

    insert_markets(conn, all_markets).await?;
    insert_market_configs(conn, all_market_configs).await?;
    insert_positions(conn, all_positions).await?;
    insert_trades(conn, all_trades).await?;
    insert_position_limits(conn, all_position_limits).await?;
    insert_limit_orders(conn, all_limit_orders).await?;
    insert_open_limit_orders(conn, all_open_limit_orders).await?;
    insert_closed_limit_orders(conn, all_closed_limit_orders).await?;
    insert_market_activities(conn, all_market_activities).await?;

    Ok(())
}

async fn insert_to_db(
    conn: &mut MyDbConnection,
    name: &'static str,
    start_version: u64,
    end_version: u64,
    all_vaults: Vec<Vault>,
    all_vault_users: Vec<VaultUser>,
    all_vault_activities: Vec<VaultActivity>,
    all_markets: Vec<Market>,
    all_market_configs: Vec<MarketConfig>,
    all_positions: Vec<Position>,
    all_trades: Vec<Trade>,
    all_position_limits: Vec<PositionLimit>,
    all_limit_orders: Vec<LimitOrder>,
    all_open_limit_orders: Vec<OpenLimitOrder>,
    all_closed_limit_orders: Vec<ClosedLimitOrder>,
    all_market_activities: Vec<MarketActivity>,
) -> Result<(), diesel::result::Error> {
    tracing::trace!(
        name = name,
        start_version = start_version,
        end_version = end_version,
        "Inserting to db",
    );
    match conn
        .build_transaction()
        .read_write()
        .run::<_, Error, _>(|pg_conn| {
            Box::pin(insert_to_db_impl(
                pg_conn,
                &all_vaults,
                &all_vault_users,
                &all_vault_activities,
                &all_markets,
                &all_market_configs,
                &all_positions,
                &all_trades,
                &all_position_limits,
                &all_limit_orders,
                &all_open_limit_orders,
                &all_closed_limit_orders,
                &all_market_activities,
            ))
        })
        .await
    {
        Ok(_) => Ok(()),
        Err(_) => {
            conn.build_transaction()
                .read_write()
                .run::<_, Error, _>(|pg_conn| {
                    Box::pin(async {
                        let all_vaults: Vec<Vault> = clean_data_for_db(all_vaults, true);
                        let all_vault_users = clean_data_for_db(all_vault_users, true);
                        let all_vault_activities = clean_data_for_db(all_vault_activities, true);
                        let all_markets = clean_data_for_db(all_markets, true);
                        let all_market_configs = clean_data_for_db(all_market_configs, true);
                        let all_positions = clean_data_for_db(all_positions, true);
                        let all_trades = clean_data_for_db(all_trades, true);
                        let all_position_limits = clean_data_for_db(all_position_limits, true);
                        let all_limit_orders = clean_data_for_db(all_limit_orders, true);
                        let all_open_limit_orders = clean_data_for_db(all_open_limit_orders, true);
                        let all_closed_limit_orders = clean_data_for_db(all_closed_limit_orders, true);
                        let all_market_activities = clean_data_for_db(all_market_activities, true);

                        insert_to_db_impl(
                            pg_conn,
                            &all_vaults,
                            &all_vault_users,
                            &all_vault_activities,
                            &all_markets,
                            &all_market_configs,
                            &all_positions,
                            &all_trades,
                            &all_position_limits,
                            &all_limit_orders,
                            &all_open_limit_orders,
                            &all_closed_limit_orders,
                            &all_market_activities,
                        ).await
                    })
                })
                .await
        },
    }
}

async fn insert_vaults(
    conn: &mut MyDbConnection,
    item_to_insert: &[Vault],
) -> Result<(), diesel::result::Error> {
    use schema::vaults::dsl::*;

    let chunks = get_chunks(item_to_insert.len(), Vault::field_count());
    for (start_ind, end_ind) in chunks {
        execute_with_better_error(
            conn,
            diesel::insert_into(schema::vaults::table)
                .values(&item_to_insert[start_ind..end_ind])
                .on_conflict((transaction_version, type_hash))
                .do_nothing(),
                None,
            ).await?;
    }
    Ok(())
}

async fn insert_vault_user(
    conn: &mut MyDbConnection,
    item_to_insert: &[VaultUser],
) -> Result<(), diesel::result::Error> {
    use schema::vault_users::dsl::*;

    let chunks = get_chunks(item_to_insert.len(), VaultUser::field_count());
    for (start_ind, end_ind) in chunks {
        execute_with_better_error(
            conn,
            diesel::insert_into(schema::vault_users::table)
                .values(&item_to_insert[start_ind..end_ind])
                .on_conflict((transaction_version, user_addr, type_hash))
                .do_nothing(),
                None,
            ).await?;
    }
    Ok(())
}

async fn insert_vault_activities(
    conn: &mut MyDbConnection,
    item_to_insert: &[VaultActivity],
) -> Result<(), diesel::result::Error> {
    use schema::vault_activities::dsl::*;

    let chunks = get_chunks(item_to_insert.len(), VaultActivity::field_count());
    for (start_ind, end_ind) in chunks {
        execute_with_better_error(
            conn,
            diesel::insert_into(schema::vault_activities::table)
                .values(&item_to_insert[start_ind..end_ind])
                .on_conflict((
                    transaction_version,
                    event_creation_number,
                    event_sequence_number,
                ))
                .do_nothing(),
            None,
        ).await?;
    }
    Ok(())
}

async fn insert_markets(
    conn: &mut MyDbConnection,
    item_to_insert: &[Market],
) -> Result<(), diesel::result::Error> {
    use schema::markets::dsl::*;

    let chunks = get_chunks(item_to_insert.len(), Market::field_count());
    for (start_ind, end_ind) in chunks {
        execute_with_better_error(
            conn,
            diesel::insert_into(schema::markets::table)
                .values(&item_to_insert[start_ind..end_ind])
                .on_conflict((transaction_version, type_hash))
                .do_nothing(),
                None,
            ).await?;
    }
    Ok(())
}

async fn insert_market_configs(
    conn: &mut MyDbConnection,
    item_to_insert: &[MarketConfig],
) -> Result<(), diesel::result::Error> {
    use schema::market_configs::dsl::*;

    let chunks = get_chunks(item_to_insert.len(), MarketConfig::field_count());
    for (start_ind, end_ind) in chunks {
        execute_with_better_error(
            conn,
            diesel::insert_into(schema::market_configs::table)
                .values(&item_to_insert[start_ind..end_ind])
                .on_conflict((transaction_version, type_hash))
                .do_nothing(),
                None,
            ).await?;
    }
    Ok(())
}

async fn insert_positions(
    conn: &mut MyDbConnection,
    item_to_insert: &[Position],
) -> Result<(), diesel::result::Error> {
    use schema::positions::dsl::*;

    let chunks = get_chunks(item_to_insert.len(), Position::field_count());
    for (start_ind, end_ind) in chunks {
        execute_with_better_error(
            conn,
            diesel::insert_into(schema::positions::table)
                .values(&item_to_insert[start_ind..end_ind])
                .on_conflict((transaction_version, id))
                .do_nothing(),
                None,
            ).await?;
    }
    Ok(())
}

async fn insert_trades(
    conn: &mut MyDbConnection,
    item_to_insert: &[Trade],
) -> Result<(), diesel::result::Error> {
    use schema::trades::dsl::*;

    let chunks = get_chunks(item_to_insert.len(), Trade::field_count());
    for (start_ind, end_ind) in chunks {
        execute_with_better_error(
            conn,
            diesel::insert_into(schema::trades::table)
                .values(&item_to_insert[start_ind..end_ind])
                .on_conflict((id, transaction_version))
                .do_nothing(),
                None,
            ).await?;
    }
    Ok(())
}

async fn insert_position_limits(
    conn: &mut MyDbConnection,
    item_to_insert: &[PositionLimit],
) -> Result<(), diesel::result::Error> {
    use schema::position_limits::dsl::*;

    let chunks = get_chunks(item_to_insert.len(), PositionLimit::field_count());
    for (start_ind, end_ind) in chunks {
        execute_with_better_error(
            conn,
            diesel::insert_into(schema::position_limits::table)
                .values(&item_to_insert[start_ind..end_ind])
                .on_conflict((transaction_version, user_addr, type_hash))
                .do_nothing(),
                None,
            ).await?;
    }
    Ok(())
}

async fn insert_limit_orders(
    conn: &mut MyDbConnection,
    item_to_insert: &[LimitOrder],
) -> Result<(), diesel::result::Error> {
    use schema::limit_orders::dsl::*;

    let chunks = get_chunks(item_to_insert.len(), LimitOrder::field_count());
    for (start_ind, end_ind) in chunks {
        execute_with_better_error(
            conn,
            diesel::insert_into(schema::limit_orders::table)
                .values(&item_to_insert[start_ind..end_ind])
                .on_conflict((transaction_version, id))
                .do_nothing(),
                None,
            ).await?;
    }
    Ok(())
}

async fn insert_open_limit_orders(
    conn: &mut MyDbConnection,
    item_to_insert: &[OpenLimitOrder],
) -> Result<(), diesel::result::Error> {
    use schema::open_limit_orders::dsl::*;

    let chunks = get_chunks(item_to_insert.len(), OpenLimitOrder::field_count());
    for (start_ind, end_ind) in chunks {
        execute_with_better_error(
            conn,
            diesel::insert_into(schema::open_limit_orders::table)
                .values(&item_to_insert[start_ind..end_ind])
                .on_conflict(id)
                .do_update()
                .set((
                    transaction_timestamp.eq(excluded(transaction_timestamp)),
                    transaction_version.eq(excluded(transaction_version)),
                    inserted_at.eq(excluded(inserted_at)),
                )),
                Some(" WHERE open_limit_orders.last_transaction_version <= excluded.last_transaction_version "),
            ).await?;
    }
    Ok(())
}

async fn insert_closed_limit_orders(
    conn: &mut MyDbConnection,
    item_to_insert: &[ClosedLimitOrder],
) -> Result<(), diesel::result::Error> {
    use schema::closed_limit_orders;
    use schema::open_limit_orders;

    let chunks = get_chunks(item_to_insert.len(), ClosedLimitOrder::field_count());
    for (start_ind, end_ind) in chunks {
        execute_with_better_error(
            conn,
            diesel::insert_into(closed_limit_orders::table)
                .values(&item_to_insert[start_ind..end_ind])
                .on_conflict(closed_limit_orders::id)
                .do_nothing(),
                None,
            ).await?;

            // Delete closed limit orders.
            let closed_ids: Vec<BigDecimal> = item_to_insert[start_ind..end_ind].iter().map(|x| x.id.clone()).collect();
            execute_with_better_error(
                conn,
                diesel::delete(open_limit_orders::table)
                    .filter(open_limit_orders::id.eq_any(&closed_ids)),
                    None
                ).await?;
    }
    Ok(())
}

async fn insert_market_activities(
    conn: &mut MyDbConnection,
    item_to_insert: &[MarketActivity],
) -> Result<(), diesel::result::Error> {
    use schema::market_activities::dsl::*;

    let chunks = get_chunks(item_to_insert.len(), MarketActivity::field_count());
    for (start_ind, end_ind) in chunks {
        execute_with_better_error(
            conn,
            diesel::insert_into(schema::market_activities::table)
                .values(&item_to_insert[start_ind..end_ind])
                .on_conflict((
                    transaction_version,
                    event_creation_number,
                    event_sequence_number,
                ))
                .do_nothing(),
            None,
        ).await?;
    }
    Ok(())
}

#[async_trait]
impl ProcessorTrait for MirageProcessor {
    fn name(&self) -> &'static str {
        ProcessorName::CoinProcessor.into()
    }

    async fn process_transactions(
        &self,
        transactions: Vec<Transaction>,
        start_version: u64,
        end_version: u64,
        _: Option<u64>,
    ) -> anyhow::Result<ProcessingResult> {
        let mut conn = self.get_conn().await;

        let mut all_vaults: Vec<Vault> = vec![];
        let mut all_vault_users: Vec<VaultUser> = vec![];
        let mut all_vault_activities: Vec<VaultActivity> = vec![];

        let mut all_markets: Vec<Market> = vec![];
        let mut all_market_configs: Vec<MarketConfig> = vec![];

        let mut all_positions: Vec<Position> = vec![];
        let mut all_trades: Vec<Trade> = vec![];
        let mut all_position_limits: Vec<PositionLimit> = vec![];

        let mut all_limit_orders: Vec<LimitOrder> = vec![];
        let mut all_open_limit_orders: Vec<OpenLimitOrder> = vec![];
        let mut all_closed_limit_orders: Vec<ClosedLimitOrder> = vec![];

        let mut all_market_activities: Vec<MarketActivity> = vec![];

        for txn in &transactions {
            let (mut vaults, mut vault_users, mut vault_activities) = VaultActivity::from_transaction(
                txn,
            );
            all_vaults.append(&mut vaults);
            all_vault_users.append(&mut vault_users);
            all_vault_activities.append(&mut vault_activities);

            let (
                    mut markets,
                    mut market_configs,
                    mut positions,
                    mut trades,
                    mut position_limits,
                    mut limit_orders,
                    mut open_limit_orders,
                    mut closed_limit_orders,
                    mut market_activities,
                ) = MarketActivity::from_transaction(
                txn,
            );

            all_markets.append(&mut markets);
            all_market_configs.append(&mut market_configs);
            all_positions.append(&mut positions);
            all_trades.append(&mut trades);
            all_position_limits.append(&mut position_limits);
            all_limit_orders.append(&mut limit_orders);
            all_open_limit_orders.append(&mut open_limit_orders);
            all_closed_limit_orders.append(&mut closed_limit_orders);
            all_market_activities.append(&mut market_activities);
        }

        // Sort by vault type
        all_vaults.sort_by(|a, b| (&a.collateral_type, &a.borrow_type)
            .cmp(&(&b.collateral_type, &b.borrow_type)));

        // Sort by user address, vault type
        all_vault_users.sort_by(|a, b| (&a.user_addr, &a.collateral_type, &a.borrow_type)
            .cmp(&(&b.user_addr, &b.collateral_type, &b.borrow_type)));

        // Sort by market type
        all_markets.sort_by(|a, b| (&a.margin_type, &a.perp_type)
            .cmp(&(&b.margin_type, &b.perp_type)));
        all_market_configs.sort_by(|a, b| (&a.margin_type, &a.perp_type)
            .cmp(&(&b.margin_type, &b.perp_type)));

        // Sort by user address, market type
        all_position_limits.sort_by(|a, b| (&a.user_addr, &a.margin_type, &a.perp_type)
            .cmp(&(&b.user_addr, &b.margin_type, &b.perp_type)));

        // Sort by id
        all_positions.sort_by(|a, b| a.id.cmp(&b.id));
        all_trades.sort_by(|a, b| a.id.cmp(&b.id));
        all_limit_orders.sort_by(|a, b| a.id.cmp(&b.id));
        all_open_limit_orders.sort_by(|a, b| a.id.cmp(&b.id));
        all_closed_limit_orders.sort_by(|a, b| a.id.cmp(&b.id));

        tracing::trace!(
            name = self.name(),
            start_version = start_version,
            end_version = end_version,
            r#"{{ processed: {:?}
                user info: {:?} vaults: {:?} vault activities: {:?}
                markets {:?} market config {:?} market activities {:?}
                all_position {:?} all_trades {:?}
                all_position_limits {:?} all_limit_orders {:?} all_open_limit_orders {:?} all_closed_limit_orders {:?}"#,
            transactions.len(),
            all_vault_users.len(),
            all_vaults.len(),
            all_vault_activities.len(),
            all_markets.len(),
            all_market_activities.len(),
            all_market_configs.len(),
            all_positions.len(),
            all_trades.len(),
            all_position_limits.len(),
            all_limit_orders.len(),
            all_open_limit_orders.len(),
            all_closed_limit_orders.len(),
        );

        let tx_result = insert_to_db(
            &mut conn,
            self.name(),
            start_version,
            end_version,
            all_vaults,
            all_vault_users,
            all_vault_activities,
            all_markets,
            all_market_configs,
            all_positions,
            all_trades,
            all_position_limits,
            all_limit_orders,
            all_open_limit_orders,
            all_closed_limit_orders,
            all_market_activities,
        ).await;
        match tx_result {
            Ok(_) => Ok((start_version, end_version)),
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
