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

  transaction_timestamp TIMESTAMP NOT NULL,
  inserted_at TIMESTAMP NOT NULL DEFAULT NOW(),
  -- Constraints
  PRIMARY KEY (transaction_version, write_set_change_index)
);
