-- Your SQL goes here

-- market configs
CREATE TABLE mirage_debt_store_datas (
  transaction_version BIGINT NOT NULL,
  write_set_change_index BIGINT NOT NULL,

  object_address VARCHAR(66) NOT NULL,
  debt_elastic NUMERIC NOT NULL,
  debt_base NUMERIC NOT NULL,
  burn_prev_qty NUMERIC NOT NULL,
  burn_cur_qty NUMERIC NOT NULL,
  burn_window_start TIMESTAMP NOT NULL,
  burn_window_duration_sec NUMERIC NOT NULL,
  burn_max_outflow NUMERIC NOT NULL,
  mint_prev_qty NUMERIC NOT NULL,
  mint_cur_qty NUMERIC NOT NULL,
  mint_window_start TIMESTAMP NOT NULL,
  mint_window_duration_sec NUMERIC NOT NULL,
  mint_max_outflow NUMERIC NOT NULL,

  net_fees NUMERIC NOT NULL,
  net_burn NUMERIC NOT NULL,

  transaction_timestamp TIMESTAMP NOT NULL,
  inserted_at TIMESTAMP NOT NULL DEFAULT NOW(),
  -- Constraints
  PRIMARY KEY (transaction_version, write_set_change_index, transaction_timestamp)
);

CREATE INDEX mirage_debt_store_datas_oa on mirage_debt_store_datas (object_address);
CREATE INDEX mirage_debt_store_datas_ts on mirage_debt_store_datas (transaction_timestamp);

-- Convert mirage_debt_store_datas to a hypertable
SELECT create_hypertable('mirage_debt_store_datas', 'transaction_timestamp', migrate_data => true);

-- Drop existing view if needed
DROP MATERIALIZED VIEW IF EXISTS mirage_debt_1hour;

-- Create continuous aggregate for 1-hour fees and burn tracking
CREATE MATERIALIZED VIEW mirage_debt_1hour
WITH (timescaledb.continuous) AS
SELECT
  object_address,
  time_bucket('1 hours', transaction_timestamp) AS bucket,
  -- Basic aggregations (no window functions allowed in continuous aggregates)
  COALESCE(FIRST(net_fees, transaction_timestamp), 0) AS start_net_fees,
  COALESCE(LAST(net_fees, transaction_timestamp), 0) AS end_net_fees,
  COALESCE(FIRST(net_burn, transaction_timestamp), 0) AS start_net_burn,
  COALESCE(LAST(net_burn, transaction_timestamp), 0) AS end_net_burn,
  COALESCE(FIRST(CASE WHEN debt_base = 0 THEN 1 ELSE debt_elastic / CASE WHEN debt_base = 0 THEN 1 ELSE debt_base END END, transaction_timestamp), 1) AS start_debt_ratio,
  COALESCE(LAST(CASE WHEN debt_base = 0 THEN 1 ELSE debt_elastic / CASE WHEN debt_base = 0 THEN 1 ELSE debt_base END END, transaction_timestamp), 1) AS end_debt_ratio
FROM mirage_debt_store_datas
GROUP BY object_address, time_bucket('1 hours', transaction_timestamp)
WITH NO DATA;

-- Index for faster queries
CREATE INDEX IF NOT EXISTS mirage_debt_1hour_idx ON mirage_debt_1hour (bucket, object_address);

-- Drop existing policy if it exists
DO $$
BEGIN
  PERFORM remove_continuous_aggregate_policy('mirage_debt_1hour');
EXCEPTION
  WHEN others THEN
    NULL; -- Ignore errors if policy doesn't exist
END
$$;

-- Add continuous aggregate policy to refresh every 15 minutes
SELECT add_continuous_aggregate_policy('mirage_debt_1hour'::regclass,
  start_offset=>NULL,
  end_offset=>'1 hours'::interval,
  schedule_interval=>'15 mins'::interval);
