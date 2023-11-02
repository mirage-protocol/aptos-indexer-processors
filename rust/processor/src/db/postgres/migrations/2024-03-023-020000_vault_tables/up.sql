-- Your SQL goes here

-- vaults
CREATE TABLE vault_collection_configs (
  transaction_version BIGINT NOT NULL,
  write_set_change_index BIGINT NOT NULL,

  collection_id VARCHAR(66) NOT NULL,
  collateral_token_id VARCHAR(66) NOT NULL,
  borrow_token_id VARCHAR(66) NOT NULL,

  interest_per_second NUMERIC NOT NULL,
  initial_collateralization_rate NUMERIC NOT NULL,
  maintenance_collateralization_rate NUMERIC NOT NULL,
  liquidation_multiplier NUMERIC NOT NULL,
  borrow_fee NUMERIC NOT NULL,
  protocol_liquidation_fee NUMERIC NOT NULL,
  min_collateral_amount NUMERIC NOT NULL,
  max_collection_debt_amount NUMERIC NOT NULL,
  liquidation_rate_limiter_max_outflow NUMERIC NOT NULL,

  transaction_timestamp TIMESTAMP NOT NULL,
  inserted_at TIMESTAMP NOT NULL DEFAULT NOW(),
  -- Constraints
  PRIMARY KEY (transaction_version, write_set_change_index)
);
CREATE INDEX vault_collection_configs_c_id on vault_collection_configs (collection_id);
CREATE INDEX vault_collection_configs_cb_id on vault_collection_configs (collateral_token_id, borrow_token_id);

-- vaults
CREATE TABLE vault_collection_datas (
  transaction_version BIGINT NOT NULL,
  write_set_change_index BIGINT NOT NULL,

  collection_id VARCHAR(66) NOT NULL,
  collateral_token_id VARCHAR(66) NOT NULL,
  borrow_token_id VARCHAR(66) NOT NULL,

  total_collateral NUMERIC NOT NULL,
  borrow_elastic NUMERIC NOT NULL,
  borrow_base NUMERIC NOT NULL,
  global_debt_part NUMERIC NOT NULL,
  last_interest_payment TIMESTAMP NOT NULL,
  cached_exchange_rate NUMERIC NOT NULL,
  last_interest_update TIMESTAMP NOT NULL,
  is_emergency BOOLEAN NOT NULL,

  liquidation_rate_limiter_prev_qty NUMERIC NOT NULL,
  liquidation_rate_limiter_cur_qty NUMERIC NOT NULL,
  liquidation_rate_limiter_window_start TIMESTAMP NOT NULL,

  transaction_timestamp TIMESTAMP NOT NULL,
  inserted_at TIMESTAMP NOT NULL DEFAULT NOW(),
  -- Constraints
  PRIMARY KEY (transaction_version, write_set_change_index)
);
CREATE INDEX vault_collection_datas_cid on vault_collection_datas (collection_id);
CREATE INDEX vault_collection_datas_cb_id on vault_collection_datas (collateral_token_id, borrow_token_id);

-- user infos
CREATE TABLE vault_datas (
  transaction_version BIGINT NOT NULL,
  write_set_change_index BIGINT NOT NULL,

  owner_addr VARCHAR(66) NOT NULL,
  collection_id VARCHAR(66) NOT NULL,
  vault_id VARCHAR(66) NOT NULL,

  collateral_amount NUMERIC NOT NULL,
  borrow_part NUMERIC NOT NULL,

  transaction_timestamp TIMESTAMP NOT NULL,
  inserted_at TIMESTAMP NOT NULL DEFAULT NOW(),
  -- Constraints
  PRIMARY KEY (transaction_version, write_set_change_index)
);
CREATE INDEX vault_datas_owner on vault_datas (owner_addr);

-- vault activities
CREATE TABLE vault_activities (
  transaction_version BIGINT NOT NULL,
  event_creation_number BIGINT NOT NULL,
  event_sequence_number BIGINT NOT NULL,
  event_index BIGINT NOT NULL,

  collection_id VARCHAR(66) NOT NULL,
  vault_id VARCHAR(66),
  src_vault_id VARCHAR(66),
  event_type VARCHAR(5000) NOT NULL,
  owner_addr VARCHAR(66),

  collateral_amount NUMERIC,
  borrow_amount NUMERIC,
  fee_amount NUMERIC,
  socialized_amount NUMERIC,
  collateralization_rate_before NUMERIC,
  collateralization_rate_after NUMERIC,
  new_interest_per_second NUMERIC,

  transaction_timestamp TIMESTAMP NOT NULL,
  inserted_at TIMESTAMP NOT NULL DEFAULT NOW(),
  -- Constraints
  PRIMARY KEY (
    transaction_version,
    event_creation_number,
    event_sequence_number,
    event_index
  )
);
CREATE INDEX vault_activities_c_id_et_sn on vault_activities (collection_id, event_type, event_sequence_number);
