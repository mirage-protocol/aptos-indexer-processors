// @generated automatically by Diesel CLI.

diesel::table! {
    account_transactions (account_address, transaction_version) {
        transaction_version -> Int8,
        account_address -> Varchar,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    ans_lookup (transaction_version, write_set_change_index) {
        transaction_version -> Int8,
        write_set_change_index -> Int8,
        domain -> Varchar,
        subdomain -> Varchar,
        registered_address -> Nullable<Varchar>,
        expiration_timestamp -> Nullable<Timestamp>,
        token_name -> Varchar,
        is_deleted -> Bool,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    ans_lookup_v2 (transaction_version, write_set_change_index) {
        transaction_version -> Int8,
        write_set_change_index -> Int8,
        domain -> Varchar,
        subdomain -> Varchar,
        token_standard -> Varchar,
        registered_address -> Nullable<Varchar>,
        expiration_timestamp -> Nullable<Timestamp>,
        token_name -> Varchar,
        is_deleted -> Bool,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    ans_primary_name (transaction_version, write_set_change_index) {
        transaction_version -> Int8,
        write_set_change_index -> Int8,
        registered_address -> Varchar,
        domain -> Nullable<Varchar>,
        subdomain -> Nullable<Varchar>,
        token_name -> Nullable<Varchar>,
        is_deleted -> Bool,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    ans_primary_name_v2 (transaction_version, write_set_change_index) {
        transaction_version -> Int8,
        write_set_change_index -> Int8,
        registered_address -> Varchar,
        domain -> Nullable<Varchar>,
        subdomain -> Nullable<Varchar>,
        token_standard -> Varchar,
        token_name -> Nullable<Varchar>,
        is_deleted -> Bool,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    block_metadata_transactions (version) {
        version -> Int8,
        block_height -> Int8,
        id -> Varchar,
        round -> Int8,
        epoch -> Int8,
        previous_block_votes_bitvec -> Jsonb,
        proposer -> Varchar,
        failed_proposer_indices -> Jsonb,
        timestamp -> Timestamp,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    closed_limit_orders (id) {
        user_addr -> Varchar,
        type_hash -> Varchar,
        margin_type -> Varchar,
        perp_type -> Varchar,
        id -> Numeric,
        transaction_timestamp -> Timestamp,
        transaction_version -> Int8,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    coin_activities (transaction_version, event_account_address, event_creation_number, event_sequence_number) {
        transaction_version -> Int8,
        event_account_address -> Varchar,
        event_creation_number -> Int8,
        event_sequence_number -> Int8,
        owner_address -> Varchar,
        coin_type -> Varchar,
        amount -> Numeric,
        activity_type -> Varchar,
        is_gas_fee -> Bool,
        is_transaction_success -> Bool,
        entry_function_id_str -> Nullable<Varchar>,
        block_height -> Int8,
        transaction_timestamp -> Timestamp,
        inserted_at -> Timestamp,
        event_index -> Nullable<Int8>,
        gas_fee_payer_address -> Nullable<Varchar>,
        storage_refund_amount -> Numeric,
    }
}

diesel::table! {
    coin_balances (transaction_version, owner_address, coin_type_hash) {
        transaction_version -> Int8,
        owner_address -> Varchar,
        coin_type_hash -> Varchar,
        coin_type -> Varchar,
        amount -> Numeric,
        transaction_timestamp -> Timestamp,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    coin_infos (coin_type_hash) {
        coin_type_hash -> Varchar,
        coin_type -> Varchar,
        transaction_version_created -> Int8,
        creator_address -> Varchar,
        name -> Varchar,
        symbol -> Varchar,
        decimals -> Int4,
        transaction_created_timestamp -> Timestamp,
        inserted_at -> Timestamp,
        supply_aggregator_table_handle -> Nullable<Varchar>,
        supply_aggregator_table_key -> Nullable<Text>,
    }
}

diesel::table! {
    coin_supply (transaction_version, coin_type_hash) {
        transaction_version -> Int8,
        coin_type_hash -> Varchar,
        coin_type -> Varchar,
        supply -> Numeric,
        transaction_timestamp -> Timestamp,
        transaction_epoch -> Int8,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    collection_datas (collection_data_id_hash, transaction_version) {
        collection_data_id_hash -> Varchar,
        transaction_version -> Int8,
        creator_address -> Varchar,
        collection_name -> Varchar,
        description -> Text,
        metadata_uri -> Varchar,
        supply -> Numeric,
        maximum -> Numeric,
        maximum_mutable -> Bool,
        uri_mutable -> Bool,
        description_mutable -> Bool,
        inserted_at -> Timestamp,
        table_handle -> Varchar,
        transaction_timestamp -> Timestamp,
    }
}

diesel::table! {
    collections_v2 (transaction_version, write_set_change_index) {
        transaction_version -> Int8,
        write_set_change_index -> Int8,
        collection_id -> Varchar,
        creator_address -> Varchar,
        collection_name -> Varchar,
        description -> Text,
        uri -> Varchar,
        current_supply -> Numeric,
        max_supply -> Nullable<Numeric>,
        total_minted_v2 -> Nullable<Numeric>,
        mutable_description -> Nullable<Bool>,
        mutable_uri -> Nullable<Bool>,
        table_handle_v1 -> Nullable<Varchar>,
        token_standard -> Varchar,
        transaction_timestamp -> Timestamp,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    current_ans_lookup (domain, subdomain) {
        domain -> Varchar,
        subdomain -> Varchar,
        registered_address -> Nullable<Varchar>,
        expiration_timestamp -> Timestamp,
        last_transaction_version -> Int8,
        inserted_at -> Timestamp,
        token_name -> Varchar,
        is_deleted -> Bool,
    }
}

diesel::table! {
    current_ans_lookup_v2 (domain, subdomain, token_standard) {
        domain -> Varchar,
        subdomain -> Varchar,
        token_standard -> Varchar,
        token_name -> Nullable<Varchar>,
        registered_address -> Nullable<Varchar>,
        expiration_timestamp -> Timestamp,
        last_transaction_version -> Int8,
        is_deleted -> Bool,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    current_ans_primary_name (registered_address) {
        registered_address -> Varchar,
        domain -> Nullable<Varchar>,
        subdomain -> Nullable<Varchar>,
        token_name -> Nullable<Varchar>,
        is_deleted -> Bool,
        last_transaction_version -> Int8,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    current_ans_primary_name_v2 (registered_address, token_standard) {
        registered_address -> Varchar,
        token_standard -> Varchar,
        domain -> Nullable<Varchar>,
        subdomain -> Nullable<Varchar>,
        token_name -> Nullable<Varchar>,
        is_deleted -> Bool,
        last_transaction_version -> Int8,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    current_coin_balances (owner_address, coin_type_hash) {
        owner_address -> Varchar,
        coin_type_hash -> Varchar,
        coin_type -> Varchar,
        amount -> Numeric,
        last_transaction_version -> Int8,
        last_transaction_timestamp -> Timestamp,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    current_collection_datas (collection_data_id_hash) {
        collection_data_id_hash -> Varchar,
        creator_address -> Varchar,
        collection_name -> Varchar,
        description -> Text,
        metadata_uri -> Varchar,
        supply -> Numeric,
        maximum -> Numeric,
        maximum_mutable -> Bool,
        uri_mutable -> Bool,
        description_mutable -> Bool,
        last_transaction_version -> Int8,
        inserted_at -> Timestamp,
        table_handle -> Varchar,
        last_transaction_timestamp -> Timestamp,
    }
}

diesel::table! {
    current_collections_v2 (collection_id) {
        collection_id -> Varchar,
        creator_address -> Varchar,
        collection_name -> Varchar,
        description -> Text,
        uri -> Varchar,
        current_supply -> Numeric,
        max_supply -> Nullable<Numeric>,
        total_minted_v2 -> Nullable<Numeric>,
        mutable_description -> Nullable<Bool>,
        mutable_uri -> Nullable<Bool>,
        table_handle_v1 -> Nullable<Varchar>,
        token_standard -> Varchar,
        last_transaction_version -> Int8,
        last_transaction_timestamp -> Timestamp,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    current_delegated_staking_pool_balances (staking_pool_address) {
        staking_pool_address -> Varchar,
        total_coins -> Numeric,
        total_shares -> Numeric,
        last_transaction_version -> Int8,
        inserted_at -> Timestamp,
        operator_commission_percentage -> Numeric,
        inactive_table_handle -> Varchar,
        active_table_handle -> Varchar,
    }
}

diesel::table! {
    current_delegated_voter (delegation_pool_address, delegator_address) {
        delegation_pool_address -> Varchar,
        delegator_address -> Varchar,
        table_handle -> Nullable<Varchar>,
        voter -> Nullable<Varchar>,
        pending_voter -> Nullable<Varchar>,
        last_transaction_version -> Int8,
        last_transaction_timestamp -> Timestamp,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    current_delegator_balances (delegator_address, pool_address, pool_type, table_handle) {
        delegator_address -> Varchar,
        pool_address -> Varchar,
        pool_type -> Varchar,
        table_handle -> Varchar,
        last_transaction_version -> Int8,
        inserted_at -> Timestamp,
        shares -> Numeric,
        parent_table_handle -> Varchar,
    }
}

diesel::table! {
    current_fungible_asset_balances (storage_id) {
        storage_id -> Varchar,
        owner_address -> Varchar,
        asset_type -> Varchar,
        is_primary -> Bool,
        is_frozen -> Bool,
        amount -> Numeric,
        last_transaction_timestamp -> Timestamp,
        last_transaction_version -> Int8,
        token_standard -> Varchar,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    current_objects (object_address) {
        object_address -> Varchar,
        owner_address -> Varchar,
        state_key_hash -> Varchar,
        allow_ungated_transfer -> Bool,
        last_guid_creation_num -> Numeric,
        last_transaction_version -> Int8,
        is_deleted -> Bool,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    current_staking_pool_voter (staking_pool_address) {
        staking_pool_address -> Varchar,
        voter_address -> Varchar,
        last_transaction_version -> Int8,
        inserted_at -> Timestamp,
        operator_address -> Varchar,
    }
}

diesel::table! {
    current_table_items (table_handle, key_hash) {
        table_handle -> Varchar,
        key_hash -> Varchar,
        key -> Text,
        decoded_key -> Jsonb,
        decoded_value -> Nullable<Jsonb>,
        is_deleted -> Bool,
        last_transaction_version -> Int8,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    current_token_datas (token_data_id_hash) {
        token_data_id_hash -> Varchar,
        creator_address -> Varchar,
        collection_name -> Varchar,
        name -> Varchar,
        maximum -> Numeric,
        supply -> Numeric,
        largest_property_version -> Numeric,
        metadata_uri -> Varchar,
        payee_address -> Varchar,
        royalty_points_numerator -> Numeric,
        royalty_points_denominator -> Numeric,
        maximum_mutable -> Bool,
        uri_mutable -> Bool,
        description_mutable -> Bool,
        properties_mutable -> Bool,
        royalty_mutable -> Bool,
        default_properties -> Jsonb,
        last_transaction_version -> Int8,
        inserted_at -> Timestamp,
        collection_data_id_hash -> Varchar,
        last_transaction_timestamp -> Timestamp,
        description -> Text,
    }
}

diesel::table! {
    current_token_datas_v2 (token_data_id) {
        token_data_id -> Varchar,
        collection_id -> Varchar,
        token_name -> Varchar,
        maximum -> Nullable<Numeric>,
        supply -> Numeric,
        largest_property_version_v1 -> Nullable<Numeric>,
        token_uri -> Varchar,
        description -> Text,
        token_properties -> Jsonb,
        token_standard -> Varchar,
        is_fungible_v2 -> Nullable<Bool>,
        last_transaction_version -> Int8,
        last_transaction_timestamp -> Timestamp,
        inserted_at -> Timestamp,
        decimals -> Int8,
    }
}

diesel::table! {
    current_token_ownerships (token_data_id_hash, property_version, owner_address) {
        token_data_id_hash -> Varchar,
        property_version -> Numeric,
        owner_address -> Varchar,
        creator_address -> Varchar,
        collection_name -> Varchar,
        name -> Varchar,
        amount -> Numeric,
        token_properties -> Jsonb,
        last_transaction_version -> Int8,
        inserted_at -> Timestamp,
        collection_data_id_hash -> Varchar,
        table_type -> Text,
        last_transaction_timestamp -> Timestamp,
    }
}

diesel::table! {
    current_token_ownerships_v2 (token_data_id, property_version_v1, owner_address, storage_id) {
        token_data_id -> Varchar,
        property_version_v1 -> Numeric,
        owner_address -> Varchar,
        storage_id -> Varchar,
        amount -> Numeric,
        table_type_v1 -> Nullable<Varchar>,
        token_properties_mutated_v1 -> Nullable<Jsonb>,
        is_soulbound_v2 -> Nullable<Bool>,
        token_standard -> Varchar,
        is_fungible_v2 -> Nullable<Bool>,
        last_transaction_version -> Int8,
        last_transaction_timestamp -> Timestamp,
        inserted_at -> Timestamp,
        non_transferrable_by_owner -> Nullable<Bool>,
    }
}

diesel::table! {
    current_token_pending_claims (token_data_id_hash, property_version, from_address, to_address) {
        token_data_id_hash -> Varchar,
        property_version -> Numeric,
        from_address -> Varchar,
        to_address -> Varchar,
        collection_data_id_hash -> Varchar,
        creator_address -> Varchar,
        collection_name -> Varchar,
        name -> Varchar,
        amount -> Numeric,
        table_handle -> Varchar,
        last_transaction_version -> Int8,
        inserted_at -> Timestamp,
        last_transaction_timestamp -> Timestamp,
        token_data_id -> Varchar,
        collection_id -> Varchar,
    }
}

diesel::table! {
    current_token_v2_metadata (object_address, resource_type) {
        object_address -> Varchar,
        resource_type -> Varchar,
        data -> Jsonb,
        state_key_hash -> Varchar,
        last_transaction_version -> Int8,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    delegated_staking_activities (transaction_version, event_index) {
        transaction_version -> Int8,
        event_index -> Int8,
        delegator_address -> Varchar,
        pool_address -> Varchar,
        event_type -> Text,
        amount -> Numeric,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    delegated_staking_pool_balances (transaction_version, staking_pool_address) {
        transaction_version -> Int8,
        staking_pool_address -> Varchar,
        total_coins -> Numeric,
        total_shares -> Numeric,
        inserted_at -> Timestamp,
        operator_commission_percentage -> Numeric,
        inactive_table_handle -> Varchar,
        active_table_handle -> Varchar,
    }
}

diesel::table! {
    delegated_staking_pools (staking_pool_address) {
        staking_pool_address -> Varchar,
        first_transaction_version -> Int8,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    events (transaction_version, event_index) {
        sequence_number -> Int8,
        creation_number -> Int8,
        account_address -> Varchar,
        transaction_version -> Int8,
        transaction_block_height -> Int8,
        #[sql_name = "type"]
        type_ -> Text,
        data -> Jsonb,
        inserted_at -> Timestamp,
        event_index -> Int8,
    }
}

diesel::table! {
    fungible_asset_activities (transaction_version, event_index) {
        transaction_version -> Int8,
        event_index -> Int8,
        owner_address -> Varchar,
        storage_id -> Varchar,
        asset_type -> Varchar,
        is_frozen -> Nullable<Bool>,
        amount -> Nullable<Numeric>,
        #[sql_name = "type"]
        type_ -> Varchar,
        is_gas_fee -> Bool,
        gas_fee_payer_address -> Nullable<Varchar>,
        is_transaction_success -> Bool,
        entry_function_id_str -> Nullable<Varchar>,
        block_height -> Int8,
        token_standard -> Varchar,
        transaction_timestamp -> Timestamp,
        inserted_at -> Timestamp,
        storage_refund_amount -> Numeric,
    }
}

diesel::table! {
    fungible_asset_balances (transaction_version, write_set_change_index) {
        transaction_version -> Int8,
        write_set_change_index -> Int8,
        storage_id -> Varchar,
        owner_address -> Varchar,
        asset_type -> Varchar,
        is_primary -> Bool,
        is_frozen -> Bool,
        amount -> Numeric,
        transaction_timestamp -> Timestamp,
        token_standard -> Varchar,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    fungible_asset_metadata (asset_type) {
        asset_type -> Varchar,
        creator_address -> Varchar,
        name -> Varchar,
        symbol -> Varchar,
        decimals -> Int4,
        icon_uri -> Nullable<Varchar>,
        project_uri -> Nullable<Varchar>,
        last_transaction_version -> Int8,
        last_transaction_timestamp -> Timestamp,
        supply_aggregator_table_handle_v1 -> Nullable<Varchar>,
        supply_aggregator_table_key_v1 -> Nullable<Text>,
        token_standard -> Varchar,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    indexer_status (db) {
        db -> Varchar,
        is_indexer_up -> Bool,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    ledger_infos (chain_id) {
        chain_id -> Int8,
    }
}

diesel::table! {
    limit_orders (transaction_version, id) {
        user_addr -> Varchar,
        type_hash -> Varchar,
        margin_type -> Varchar,
        perp_type -> Varchar,
        id -> Numeric,
        is_long -> Bool,
        is_increase -> Bool,
        position_size -> Numeric,
        margin -> Numeric,
        trigger_price -> Numeric,
        triggers_above -> Bool,
        trigger_payment -> Numeric,
        max_price_slippage -> Numeric,
        expiration -> Numeric,
        transaction_timestamp -> Timestamp,
        transaction_version -> Int8,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    market_activities (transaction_version, event_creation_number, event_sequence_number) {
        transaction_version -> Int8,
        event_creation_number -> Int8,
        event_sequence_number -> Int8,
        event_index -> Int8,
        type_hash -> Varchar,
        event_type -> Varchar,
        margin_type -> Varchar,
        perp_type -> Varchar,
        user_addr -> Nullable<Varchar>,
        position_limit -> Nullable<Numeric>,
        id -> Nullable<Numeric>,
        perp_price -> Nullable<Numeric>,
        is_long -> Nullable<Bool>,
        margin_amount -> Nullable<Numeric>,
        position_size -> Nullable<Numeric>,
        maintenance_margin -> Nullable<Numeric>,
        fee -> Nullable<Numeric>,
        pnl -> Nullable<Numeric>,
        caller_addr -> Nullable<Varchar>,
        take_profit_price -> Nullable<Numeric>,
        stop_loss_price -> Nullable<Numeric>,
        trigger_price -> Nullable<Numeric>,
        max_price_slippage -> Nullable<Numeric>,
        is_increase -> Nullable<Bool>,
        triggers_above -> Nullable<Bool>,
        expiration -> Nullable<Numeric>,
        trigger_payment_amount -> Nullable<Numeric>,
        next_funding_pos -> Nullable<Bool>,
        next_funding_rate -> Nullable<Numeric>,
        transaction_timestamp -> Timestamp,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    market_configs (transaction_version, type_hash) {
        type_hash -> Varchar,
        margin_type -> Varchar,
        perp_type -> Varchar,
        max_taker_fee -> Numeric,
        min_taker_fee -> Numeric,
        max_maker_fee -> Numeric,
        min_maker_fee -> Numeric,
        liquidation_fee -> Numeric,
        min_funding_rate -> Numeric,
        max_funding_rate -> Numeric,
        pool_funding_discount -> Numeric,
        funding_interval -> Numeric,
        max_oi -> Numeric,
        max_oi_imbalance -> Numeric,
        max_leverage -> Numeric,
        base_maintenance_margin -> Nullable<Numeric>,
        base_position_limit -> Numeric,
        max_position_limit -> Numeric,
        min_order_size -> Numeric,
        transaction_timestamp -> Timestamp,
        transaction_version -> Int8,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    markets (transaction_version, type_hash) {
        type_hash -> Varchar,
        margin_type -> Varchar,
        perp_type -> Varchar,
        long_margin_base -> Numeric,
        long_margin_elastic -> Numeric,
        short_margin_base -> Numeric,
        short_margin_elastic -> Numeric,
        long_oi -> Numeric,
        short_oi -> Numeric,
        next_funding_rate -> Numeric,
        next_funding_pos -> Bool,
        last_funding_round -> Numeric,
        is_long_close_only -> Bool,
        is_short_close_only -> Bool,
        transaction_timestamp -> Timestamp,
        transaction_version -> Int8,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    move_modules (transaction_version, write_set_change_index) {
        transaction_version -> Int8,
        write_set_change_index -> Int8,
        transaction_block_height -> Int8,
        name -> Text,
        address -> Varchar,
        bytecode -> Nullable<Bytea>,
        friends -> Nullable<Jsonb>,
        exposed_functions -> Nullable<Jsonb>,
        structs -> Nullable<Jsonb>,
        is_deleted -> Bool,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    move_resources (transaction_version, write_set_change_index) {
        transaction_version -> Int8,
        write_set_change_index -> Int8,
        transaction_block_height -> Int8,
        name -> Text,
        address -> Varchar,
        #[sql_name = "type"]
        type_ -> Text,
        module -> Text,
        generic_type_params -> Nullable<Jsonb>,
        data -> Nullable<Jsonb>,
        is_deleted -> Bool,
        inserted_at -> Timestamp,
        state_key_hash -> Varchar,
    }
}

diesel::table! {
    nft_points (transaction_version) {
        transaction_version -> Int8,
        owner_address -> Varchar,
        token_name -> Text,
        point_type -> Text,
        amount -> Numeric,
        transaction_timestamp -> Timestamp,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    objects (transaction_version, write_set_change_index) {
        transaction_version -> Int8,
        write_set_change_index -> Int8,
        object_address -> Varchar,
        owner_address -> Varchar,
        state_key_hash -> Varchar,
        guid_creation_num -> Numeric,
        allow_ungated_transfer -> Bool,
        is_deleted -> Bool,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    open_limit_orders (id) {
        user_addr -> Varchar,
        type_hash -> Varchar,
        margin_type -> Varchar,
        perp_type -> Varchar,
        id -> Numeric,
        transaction_timestamp -> Timestamp,
        transaction_version -> Int8,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    position_limits (transaction_version, type_hash, user_addr) {
        user_addr -> Varchar,
        type_hash -> Varchar,
        margin_type -> Varchar,
        perp_type -> Varchar,
        position_limit -> Numeric,
        transaction_timestamp -> Timestamp,
        transaction_version -> Int8,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    positions (transaction_version, id) {
        user_addr -> Varchar,
        type_hash -> Varchar,
        margin_type -> Varchar,
        perp_type -> Varchar,
        id -> Numeric,
        opening_price -> Numeric,
        is_long -> Bool,
        margin_part -> Numeric,
        position_size -> Numeric,
        maintenance_margin -> Numeric,
        take_profit_price -> Numeric,
        stop_loss_price -> Numeric,
        trigger_payment -> Numeric,
        transaction_timestamp -> Timestamp,
        transaction_version -> Int8,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    processor_status (processor) {
        processor -> Varchar,
        last_success_version -> Int8,
        last_updated -> Timestamp,
    }
}

diesel::table! {
    proposal_votes (transaction_version, proposal_id, voter_address) {
        transaction_version -> Int8,
        proposal_id -> Int8,
        voter_address -> Varchar,
        staking_pool_address -> Varchar,
        num_votes -> Numeric,
        should_pass -> Bool,
        transaction_timestamp -> Timestamp,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    signatures (transaction_version, multi_agent_index, multi_sig_index, is_sender_primary) {
        transaction_version -> Int8,
        multi_agent_index -> Int8,
        multi_sig_index -> Int8,
        transaction_block_height -> Int8,
        signer -> Varchar,
        is_sender_primary -> Bool,
        #[sql_name = "type"]
        type_ -> Varchar,
        public_key -> Varchar,
        signature -> Varchar,
        threshold -> Int8,
        public_key_indices -> Jsonb,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    table_items (transaction_version, write_set_change_index) {
        key -> Text,
        transaction_version -> Int8,
        write_set_change_index -> Int8,
        transaction_block_height -> Int8,
        table_handle -> Varchar,
        decoded_key -> Jsonb,
        decoded_value -> Nullable<Jsonb>,
        is_deleted -> Bool,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    table_metadatas (handle) {
        handle -> Varchar,
        key_type -> Text,
        value_type -> Text,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    token_activities (transaction_version, event_account_address, event_creation_number, event_sequence_number) {
        transaction_version -> Int8,
        event_account_address -> Varchar,
        event_creation_number -> Int8,
        event_sequence_number -> Int8,
        collection_data_id_hash -> Varchar,
        token_data_id_hash -> Varchar,
        property_version -> Numeric,
        creator_address -> Varchar,
        collection_name -> Varchar,
        name -> Varchar,
        transfer_type -> Varchar,
        from_address -> Nullable<Varchar>,
        to_address -> Nullable<Varchar>,
        token_amount -> Numeric,
        coin_type -> Nullable<Text>,
        coin_amount -> Nullable<Numeric>,
        inserted_at -> Timestamp,
        transaction_timestamp -> Timestamp,
        event_index -> Nullable<Int8>,
    }
}

diesel::table! {
    token_activities_v2 (transaction_version, event_index) {
        transaction_version -> Int8,
        event_index -> Int8,
        event_account_address -> Varchar,
        token_data_id -> Varchar,
        property_version_v1 -> Numeric,
        #[sql_name = "type"]
        type_ -> Varchar,
        from_address -> Nullable<Varchar>,
        to_address -> Nullable<Varchar>,
        token_amount -> Numeric,
        before_value -> Nullable<Text>,
        after_value -> Nullable<Text>,
        entry_function_id_str -> Nullable<Varchar>,
        token_standard -> Varchar,
        is_fungible_v2 -> Nullable<Bool>,
        transaction_timestamp -> Timestamp,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    token_datas (token_data_id_hash, transaction_version) {
        token_data_id_hash -> Varchar,
        transaction_version -> Int8,
        creator_address -> Varchar,
        collection_name -> Varchar,
        name -> Varchar,
        maximum -> Numeric,
        supply -> Numeric,
        largest_property_version -> Numeric,
        metadata_uri -> Varchar,
        payee_address -> Varchar,
        royalty_points_numerator -> Numeric,
        royalty_points_denominator -> Numeric,
        maximum_mutable -> Bool,
        uri_mutable -> Bool,
        description_mutable -> Bool,
        properties_mutable -> Bool,
        royalty_mutable -> Bool,
        default_properties -> Jsonb,
        inserted_at -> Timestamp,
        collection_data_id_hash -> Varchar,
        transaction_timestamp -> Timestamp,
        description -> Text,
    }
}

diesel::table! {
    token_datas_v2 (transaction_version, write_set_change_index) {
        transaction_version -> Int8,
        write_set_change_index -> Int8,
        token_data_id -> Varchar,
        collection_id -> Varchar,
        token_name -> Varchar,
        maximum -> Nullable<Numeric>,
        supply -> Numeric,
        largest_property_version_v1 -> Nullable<Numeric>,
        token_uri -> Varchar,
        token_properties -> Jsonb,
        description -> Text,
        token_standard -> Varchar,
        is_fungible_v2 -> Nullable<Bool>,
        transaction_timestamp -> Timestamp,
        inserted_at -> Timestamp,
        decimals -> Int8,
    }
}

diesel::table! {
    token_ownerships (token_data_id_hash, property_version, transaction_version, table_handle) {
        token_data_id_hash -> Varchar,
        property_version -> Numeric,
        transaction_version -> Int8,
        table_handle -> Varchar,
        creator_address -> Varchar,
        collection_name -> Varchar,
        name -> Varchar,
        owner_address -> Nullable<Varchar>,
        amount -> Numeric,
        table_type -> Nullable<Text>,
        inserted_at -> Timestamp,
        collection_data_id_hash -> Varchar,
        transaction_timestamp -> Timestamp,
    }
}

diesel::table! {
    token_ownerships_v2 (transaction_version, write_set_change_index) {
        transaction_version -> Int8,
        write_set_change_index -> Int8,
        token_data_id -> Varchar,
        property_version_v1 -> Numeric,
        owner_address -> Nullable<Varchar>,
        storage_id -> Varchar,
        amount -> Numeric,
        table_type_v1 -> Nullable<Varchar>,
        token_properties_mutated_v1 -> Nullable<Jsonb>,
        is_soulbound_v2 -> Nullable<Bool>,
        token_standard -> Varchar,
        is_fungible_v2 -> Nullable<Bool>,
        transaction_timestamp -> Timestamp,
        inserted_at -> Timestamp,
        non_transferrable_by_owner -> Nullable<Bool>,
    }
}

diesel::table! {
    tokens (token_data_id_hash, property_version, transaction_version) {
        token_data_id_hash -> Varchar,
        property_version -> Numeric,
        transaction_version -> Int8,
        creator_address -> Varchar,
        collection_name -> Varchar,
        name -> Varchar,
        token_properties -> Jsonb,
        inserted_at -> Timestamp,
        collection_data_id_hash -> Varchar,
        transaction_timestamp -> Timestamp,
    }
}

diesel::table! {
    trades (id, transaction_version) {
        user_addr -> Varchar,
        type_hash -> Varchar,
        margin_type -> Varchar,
        perp_type -> Varchar,
        id -> Numeric,
        is_long -> Bool,
        size -> Numeric,
        price -> Numeric,
        fee -> Numeric,
        pnl -> Numeric,
        transaction_timestamp -> Timestamp,
        transaction_version -> Int8,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    transactions (version) {
        version -> Int8,
        block_height -> Int8,
        hash -> Varchar,
        #[sql_name = "type"]
        type_ -> Varchar,
        payload -> Nullable<Jsonb>,
        state_change_hash -> Varchar,
        event_root_hash -> Varchar,
        state_checkpoint_hash -> Nullable<Varchar>,
        gas_used -> Numeric,
        success -> Bool,
        vm_status -> Text,
        accumulator_root_hash -> Varchar,
        num_events -> Int8,
        num_write_set_changes -> Int8,
        inserted_at -> Timestamp,
        epoch -> Int8,
    }
}

diesel::table! {
    user_transactions (version) {
        version -> Int8,
        block_height -> Int8,
        parent_signature_type -> Varchar,
        sender -> Varchar,
        sequence_number -> Int8,
        max_gas_amount -> Numeric,
        expiration_timestamp_secs -> Timestamp,
        gas_unit_price -> Numeric,
        timestamp -> Timestamp,
        entry_function_id_str -> Varchar,
        inserted_at -> Timestamp,
        epoch -> Int8,
    }
}

diesel::table! {
    vault_activities (transaction_version, event_creation_number, event_sequence_number) {
        transaction_version -> Int8,
        event_creation_number -> Int8,
        event_sequence_number -> Int8,
        event_index -> Int8,
        type_hash -> Varchar,
        event_type -> Varchar,
        collateral_type -> Varchar,
        borrow_type -> Varchar,
        collateral_amount -> Nullable<Numeric>,
        borrow_amount -> Nullable<Numeric>,
        user_addr -> Nullable<Varchar>,
        withdraw_addr -> Nullable<Varchar>,
        liquidator_addr -> Nullable<Varchar>,
        accrued_amount -> Nullable<Numeric>,
        rate -> Nullable<Numeric>,
        fees_earned -> Nullable<Numeric>,
        old_interest_per_second -> Nullable<Numeric>,
        new_interest_per_second -> Nullable<Numeric>,
        transaction_timestamp -> Timestamp,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    vault_users (transaction_version, user_addr, type_hash) {
        user_addr -> Varchar,
        type_hash -> Varchar,
        collateral_type -> Varchar,
        borrow_type -> Varchar,
        collateral -> Numeric,
        borrow_part -> Numeric,
        transaction_timestamp -> Timestamp,
        transaction_version -> Int8,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    vaults (transaction_version, type_hash) {
        type_hash -> Varchar,
        collateral_type -> Varchar,
        borrow_type -> Varchar,
        total_collateral -> Numeric,
        borrow_elastic -> Numeric,
        borrow_base -> Numeric,
        global_debt_part -> Numeric,
        interest_per_second -> Numeric,
        last_interest_payment -> Numeric,
        collateralization_rate -> Numeric,
        liquidation_multiplier -> Numeric,
        borrow_fee -> Numeric,
        distribution_part -> Numeric,
        cached_exchange_rate -> Numeric,
        last_interest_update -> Numeric,
        is_emergency -> Bool,
        transaction_timestamp -> Timestamp,
        transaction_version -> Int8,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    write_set_changes (transaction_version, index) {
        transaction_version -> Int8,
        index -> Int8,
        hash -> Varchar,
        transaction_block_height -> Int8,
        #[sql_name = "type"]
        type_ -> Text,
        address -> Varchar,
        inserted_at -> Timestamp,
    }
}

diesel::joinable!(block_metadata_transactions -> transactions (version));

diesel::allow_tables_to_appear_in_same_query!(
    account_transactions,
    ans_lookup,
    ans_lookup_v2,
    ans_primary_name,
    ans_primary_name_v2,
    block_metadata_transactions,
    closed_limit_orders,
    coin_activities,
    coin_balances,
    coin_infos,
    coin_supply,
    collection_datas,
    collections_v2,
    current_ans_lookup,
    current_ans_lookup_v2,
    current_ans_primary_name,
    current_ans_primary_name_v2,
    current_coin_balances,
    current_collection_datas,
    current_collections_v2,
    current_delegated_staking_pool_balances,
    current_delegated_voter,
    current_delegator_balances,
    current_fungible_asset_balances,
    current_objects,
    current_staking_pool_voter,
    current_table_items,
    current_token_datas,
    current_token_datas_v2,
    current_token_ownerships,
    current_token_ownerships_v2,
    current_token_pending_claims,
    current_token_v2_metadata,
    delegated_staking_activities,
    delegated_staking_pool_balances,
    delegated_staking_pools,
    events,
    fungible_asset_activities,
    fungible_asset_balances,
    fungible_asset_metadata,
    indexer_status,
    ledger_infos,
    limit_orders,
    market_activities,
    market_configs,
    markets,
    move_modules,
    move_resources,
    nft_points,
    objects,
    open_limit_orders,
    position_limits,
    positions,
    processor_status,
    proposal_votes,
    signatures,
    table_items,
    table_metadatas,
    token_activities,
    token_activities_v2,
    token_datas,
    token_datas_v2,
    token_ownerships,
    token_ownerships_v2,
    tokens,
    trades,
    transactions,
    user_transactions,
    vault_activities,
    vault_users,
    vaults,
    write_set_changes,
);
