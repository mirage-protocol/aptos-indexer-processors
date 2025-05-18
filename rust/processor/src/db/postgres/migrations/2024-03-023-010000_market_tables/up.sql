CREATE EXTENSION IF NOT EXISTS timescaledb CASCADE;
-- Your SQL goes here

-- market configs
CREATE TABLE market_configs (
  transaction_version BIGINT NOT NULL,
  write_set_change_index BIGINT NOT NULL,

  market_id VARCHAR(66) NOT NULL,

  margin_token_id VARCHAR(66) NOT NULL,
  perp_symbol VARCHAR(66) NOT NULL,

  min_taker_fee NUMERIC NOT NULL,
  max_taker_fee NUMERIC NOT NULL,
  min_maker_fee NUMERIC NOT NULL,
  max_maker_fee NUMERIC NOT NULL,

  min_funding_rate NUMERIC NOT NULL,
  max_funding_rate NUMERIC NOT NULL,
  base_funding_rate NUMERIC NOT NULL,
  funding_interval NUMERIC NOT NULL,

  max_oi NUMERIC NOT NULL,
  max_oi_imbalance NUMERIC NOT NULL,

  maintenance_margin NUMERIC NULL,
  max_leverage NUMERIC NOT NULL,
  min_order_size NUMERIC NOT NULL,
  max_order_size NUMERIC NOT NULL,
  min_margin_amount NUMERIC NOT NULL,

  transaction_timestamp TIMESTAMP NOT NULL,
  inserted_at TIMESTAMP NOT NULL DEFAULT NOW(),
  -- Constraints
  PRIMARY KEY (transaction_version, write_set_change_index)
);
CREATE INDEX market_configs_mid on market_configs (market_id);

-- market infos
CREATE TABLE market_datas (
  transaction_version BIGINT NOT NULL,
  write_set_change_index BIGINT NOT NULL,

  market_id VARCHAR(66) NOT NULL,

  margin_token_id VARCHAR(66) NOT NULL,
  perp_symbol VARCHAR(66) NOT NULL,

  total_long_margin NUMERIC NOT NULL,
  total_short_margin NUMERIC NOT NULL,

  long_oi NUMERIC NOT NULL,
  short_oi NUMERIC NOT NULL,

  long_funding_accumulated_per_unit NUMERIC NOT NULL,
  short_funding_accumulated_per_unit NUMERIC NOT NULL,
  total_long_funding_accumulated NUMERIC NOT NULL,
  total_short_funding_accumulated NUMERIC NOT NULL,

  next_funding_rate NUMERIC NOT NULL,
  last_funding_round TIMESTAMP NOT NULL,

  is_long_close_only BOOLEAN NOT NULL,
  is_short_close_only BOOLEAN NOT NULL,

  transaction_timestamp TIMESTAMP NOT NULL,
  inserted_at TIMESTAMP NOT NULL DEFAULT NOW(),
  -- Constraints
  PRIMARY KEY (transaction_version, write_set_change_index)
);
CREATE INDEX market_datas_mid on market_datas (market_id);
CREATE INDEX market_datas_ts on market_datas (transaction_timestamp);

-- limit order infos
CREATE TABLE limit_order_datas (
  transaction_version BIGINT NOT NULL,
  write_set_change_index BIGINT NOT NULL,

  market_id VARCHAR(66) NOT NULL,
  position_id VARCHAR(66) NOT NULL,
  strategy_id VARCHAR(66) NOT NULL,
  owner_addr VARCHAR(66) NOT NULL,

  is_decrease_only BOOLEAN NOT NULL,
  position_size NUMERIC NOT NULL,
  is_long BOOLEAN NOT NULL,
  margin NUMERIC NOT NULL,
  trigger_price NUMERIC NOT NULL,
  triggers_above BOOLEAN NOT NULL,
  max_price_slippage NUMERIC NOT NULL,
  expiration NUMERIC NOT NULL,

  transaction_timestamp TIMESTAMP NOT NULL,
  inserted_at TIMESTAMP NOT NULL DEFAULT NOW(),
  -- Constraints
  PRIMARY KEY (transaction_version, write_set_change_index)
);
CREATE INDEX limit_order_datas_sid on limit_order_datas (strategy_id);
CREATE INDEX limit_order_datas_pid on limit_order_datas (position_id);
CREATE INDEX limit_order_datas_psid on limit_order_datas (position_id, strategy_id);
CREATE INDEX limit_order_datas_ts on limit_order_datas (transaction_timestamp);

-- current positions
CREATE TABLE current_positions (
  last_transaction_version BIGINT NOT NULL,

  market_id VARCHAR(66) NOT NULL,
  position_id VARCHAR(66) NOT NULL,
  owner_addr VARCHAR(66) NOT NULL,

  is_closed BOOLEAN NOT NULL,

  event_index BIGINT NOT NULL,
  transaction_timestamp TIMESTAMP NOT NULL,
  inserted_at TIMESTAMP NOT NULL DEFAULT NOW(),
  -- Constraints
  PRIMARY KEY (position_id)
);
CREATE INDEX current_positions_mid on current_positions (market_id);

-- current tpsl
CREATE TABLE current_tpsls (
  last_transaction_version BIGINT NOT NULL,

  market_id VARCHAR(66) NOT NULL,
  position_id VARCHAR(66) NOT NULL,
  strategy_id VARCHAR(66) NOT NULL,
  owner_addr VARCHAR(66) NOT NULL,

  is_closed BOOLEAN NOT NULL,

  event_index BIGINT NOT NULL,
  transaction_timestamp TIMESTAMP NOT NULL,
  inserted_at TIMESTAMP NOT NULL DEFAULT NOW(),
  -- Constraints
  PRIMARY KEY (strategy_id)
);
CREATE INDEX current_tpsl_pid on current_tpsls (position_id);
CREATE INDEX current_tpsl_mid on current_tpsls (market_id);

-- open limit orders
CREATE TABLE current_limit_orders (
  last_transaction_version BIGINT NOT NULL,

  market_id VARCHAR(66) NOT NULL,
  position_id VARCHAR(66) NOT NULL,
  strategy_id VARCHAR(66) NOT NULL,
  owner_addr VARCHAR(66) NOT NULL,

  is_closed BOOLEAN NOT NULL,

  event_index BIGINT NOT NULL,
  transaction_timestamp TIMESTAMP NOT NULL,
  inserted_at TIMESTAMP NOT NULL DEFAULT NOW(),
  -- Constraints
  PRIMARY KEY (strategy_id)
);
CREATE INDEX current_limit_orders_pid on current_limit_orders (position_id);
CREATE INDEX current_limit_orders_mid on current_limit_orders (market_id);

-- positions
CREATE TABLE position_datas (
  transaction_version BIGINT NOT NULL,
  write_set_change_index BIGINT NOT NULL,

  market_id VARCHAR(66) NOT NULL,
  position_id VARCHAR(66) NOT NULL,
  owner_addr VARCHAR(66) NOT NULL,

  last_settled_price NUMERIC NOT NULL,
  last_open_timestamp NUMERIC NOT NULL,
  side VARCHAR(8) NOT NULL,
  margin_amount NUMERIC NOT NULL,
  total_strategy_margin NUMERIC NOT NULL,
  position_size NUMERIC NOT NULL,
  last_funding_accumulated NUMERIC NOT NULL,

  transaction_timestamp TIMESTAMP NOT NULL,
  inserted_at TIMESTAMP NOT NULL DEFAULT NOW(),
  -- Constraints
  PRIMARY KEY (transaction_version, write_set_change_index)
);
CREATE INDEX position_datas_oa on position_datas (owner_addr);
CREATE INDEX position_datas_mid on position_datas (market_id);
CREATE INDEX position_datas_oa_mid on position_datas (owner_addr, market_id);
CREATE INDEX position_datas_ts on position_datas (transaction_timestamp);
-- tpsl
CREATE TABLE tpsl_datas (
  transaction_version BIGINT NOT NULL,
  write_set_change_index BIGINT NOT NULL,

  market_id VARCHAR(66) NOT NULL,
  position_id VARCHAR(66) NOT NULL,
  strategy_id VARCHAR(66) NOT NULL,
  owner_addr VARCHAR(66) NOT NULL,

  take_profit_price NUMERIC NOT NULL,
  stop_loss_price NUMERIC NOT NULL,

  transaction_timestamp TIMESTAMP NOT NULL,
  inserted_at TIMESTAMP NOT NULL DEFAULT NOW(),
  -- Constraints
  PRIMARY KEY (transaction_version, write_set_change_index)
);
CREATE INDEX tpsl_datas_pid on tpsl_datas (position_id);
CREATE INDEX tpsl_datas_sid on tpsl_datas (strategy_id);
CREATE INDEX tpsl_datas_psid on tpsl_datas (strategy_id, position_id);
CREATE INDEX tpsl_datas_ts on tpsl_datas (transaction_timestamp);

-- trades
CREATE TABLE trade_datas (
  transaction_version BIGINT NOT NULL,

  market_id VARCHAR(66) NOT NULL,
  position_id VARCHAR(66) NOT NULL,
  owner_addr VARCHAR(66) NOT NULL,

  is_long BOOLEAN NOT NULL,
  position_size NUMERIC NOT NULL,
  price NUMERIC NOT NULL,
  fee NUMERIC NOT NULL,
  pnl NUMERIC NOT NULL,
  event_type VARCHAR NOT NULL,

  event_index BIGINT NOT NULL,
  transaction_timestamp TIMESTAMP NOT NULL,
  inserted_at TIMESTAMP NOT NULL DEFAULT NOW(),
  -- Constraints
  PRIMARY KEY (transaction_version, event_index, transaction_timestamp)
);
CREATE INDEX trade_datas_oa on trade_datas (owner_addr);
CREATE INDEX trade_mid on trade_datas (market_id);
CREATE INDEX trades_oa_mid on trade_datas (owner_addr, market_id);
CREATE INDEX trade_datas_ts on trade_datas (transaction_timestamp);

-- Convert trade_datas to a hypertable
SELECT create_hypertable('trade_datas', 'transaction_timestamp');

-- market activities
CREATE TABLE market_activities (
  transaction_version BIGINT NOT NULL,
  event_creation_number BIGINT NOT NULL,
  event_sequence_number BIGINT NOT NULL,
  event_index BIGINT NOT NULL,

  market_id VARCHAR(66) NOT NULL,
  position_id VARCHAR(66),
  strategy_id VARCHAR(66),
  event_type VARCHAR(5000) NOT NULL,
  owner_addr VARCHAR(66),

  perp_price NUMERIC,
  is_long BOOLEAN,
  margin_amount NUMERIC,
  position_size NUMERIC,
  fee NUMERIC,
  protocol_fee NUMERIC,
  pnl NUMERIC,
  take_profit_price NUMERIC,
  stop_loss_price NUMERIC,
  trigger_price NUMERIC,
  max_price_slippage NUMERIC,
  is_decrease_only BOOLEAN,
  triggers_above BOOLEAN,
  expiration NUMERIC,
  next_funding_rate NUMERIC,
  transaction_timestamp TIMESTAMP NOT NULL,
  inserted_at TIMESTAMP NOT NULL DEFAULT NOW(),
  -- Constraints
  PRIMARY KEY (
    transaction_version,
    event_index
  )
);
CREATE INDEX market_activities_mid on market_activities (market_id, event_type, event_sequence_number);
CREATE INDEX market_activities_ts on market_activities (transaction_timestamp);

-- sql for the materialised view:
-- Drop existing materialized view
DROP MATERIALIZED VIEW IF EXISTS owner_trades_materialized;

-- Create new materialized view with ROW_NUMBER
CREATE MATERIALIZED VIEW owner_trades_materialized AS
SELECT
  subquery.owner_addr,
  subquery.total_pnl,
  subquery.total_fee,
  subquery.trade_count,
  subquery.profit,
  subquery.volume,
  row_number() OVER (
    ORDER BY
      subquery.profit DESC,
      subquery.owner_addr ASC -- Tiebreaker for uniqueness
  ) AS rank
FROM
  (
    SELECT
      trade_datas.owner_addr,
      sum(trade_datas.pnl) AS total_pnl,
      sum(trade_datas.fee) AS total_fee,
      count(*) AS trade_count,
      sum((trade_datas.pnl - trade_datas.fee)) AS profit,
      sum(
        (
          (trade_datas.position_size * trade_datas.price) / (100000000) :: numeric
        )
      ) AS volume
    FROM
      public.trade_datas
    GROUP BY
      trade_datas.owner_addr
  ) subquery;

-- Recreate indexes
CREATE UNIQUE INDEX IF NOT EXISTS owner_trades_mat_owner_addr_idx
ON owner_trades_materialized (owner_addr);

CREATE INDEX IF NOT EXISTS owner_trades_mat_profit_owner_addr_idx
ON owner_trades_materialized (profit DESC, owner_addr ASC);

CREATE INDEX IF NOT EXISTS owner_trades_mat_rank_idx
ON owner_trades_materialized (rank);

CREATE OR REPLACE PROCEDURE refresh_owner_trades_mv(job_id int, config jsonb)
LANGUAGE PLPGSQL AS
$$
BEGIN
  RAISE NOTICE 'Refreshing owner_trades_materialized view';
  REFRESH MATERIALIZED VIEW CONCURRENTLY owner_trades_materialized;
END
$$;

-- Schedule the refresh job to run every 15m using TimescaleDB's job scheduler
SELECT add_job(
  'refresh_owner_trades_mv',  -- procedure name
  '15m',                       -- run every 15m
  config => '{}'::jsonb,      -- no special config needed
  fixed_schedule => true      -- run on a fixed schedule
);

-- Drop existing view if needed
DROP MATERIALIZED VIEW IF EXISTS owner_trades_1hour;

-- Create continuous aggregate
CREATE MATERIALIZED VIEW owner_trades_1hour
WITH (timescaledb.continuous) AS
SELECT
  owner_addr,
  time_bucket('1 hours', transaction_timestamp) AS bucket,
  COALESCE(SUM(pnl - fee), 0) AS profit,
  COALESCE(SUM(fee), 0) AS fee,
  COALESCE(SUM(pnl), 0) AS pnl,
  COALESCE(SUM((position_size * price) / (100000000)::numeric), 0) AS volume
FROM trade_datas
GROUP BY owner_addr, time_bucket('1 hours', transaction_timestamp)
WITH NO DATA;

-- Index for faster queries
CREATE INDEX IF NOT EXISTS owner_trades_1hour_idx ON owner_trades_1hour (bucket, owner_addr);

SELECT add_continuous_aggregate_policy('owner_trades_1hour'::regclass,
  start_offset=>NULL,
  end_offset=>'1 hours'::interval,
  schedule_interval=>'15 mins'::interval);