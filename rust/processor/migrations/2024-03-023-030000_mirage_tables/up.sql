-- Your SQL goes here

-- market configs
CREATE TABLE mirage_debt_store_datas (
  transaction_version BIGINT NOT NULL,
  write_set_change_index BIGINT NOT NULL,

  asset_type VARCHAR(66) NOT NULL,
  debt_elastic NUMERIC NOT NULL,
  debt_base NUMERIC NOT NULL,

  transaction_timestamp TIMESTAMP NOT NULL,
  inserted_at TIMESTAMP NOT NULL DEFAULT NOW(),
  -- Constraints
  PRIMARY KEY (transaction_version, write_set_change_index)
);

CREATE TABLE fee_store_datas (
  transaction_version BIGINT NOT NULL,
  write_set_change_index BIGINT NOT NULL,

  asset_type VARCHAR(66) NOT NULL,
  net_accumulated_fees NUMERIC NOT NULL,

  transaction_timestamp TIMESTAMP NOT NULL,
  inserted_at TIMESTAMP NOT NULL DEFAULT NOW(),
  -- Constraints
  PRIMARY KEY (transaction_version, write_set_change_index)
);
