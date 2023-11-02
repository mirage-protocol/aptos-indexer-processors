// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::{
    gap_detectors::DEFAULT_GAP_DETECTION_BATCH_SIZE, processors::ProcessorConfig,
    transaction_filter::TransactionFilter, worker::Worker,
};
use ahash::AHashMap;
use anyhow::{Context, Result};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use server_framework::RunnableConfig;
use std::{collections::HashSet, env, time::Duration};
use tracing::error;
use url::Url;

pub const QUERY_DEFAULT_RETRIES: u32 = 5;
pub const QUERY_DEFAULT_RETRY_DELAY_MS: u64 = 500;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IndexerGrpcProcessorConfig {
    pub processor_config: ProcessorConfig,
    #[serde(default)]
    pub postgres_connection_string: Option<String>,
    // TODO: Add TLS support.
    #[serde(default)]
    pub indexer_grpc_data_service_address: Option<Url>,
    #[serde(flatten)]
    pub grpc_http2_config: IndexerGrpcHttp2Config,
    pub auth_token: String,
    // Version to start indexing from
    #[serde(default)]
    pub starting_version: Option<u64>,
    // Version to end indexing at
    pub ending_version: Option<u64>,
    // Number of tasks waiting to pull transaction batches from the channel and process them
    pub number_concurrent_processing_tasks: Option<usize>,
    // Size of the pool for writes/reads to the DB. Limits maximum number of queries in flight
    pub db_pool_size: Option<u32>,
    // Maximum number of batches "missing" before we assume we have an issue with gaps and abort
    #[serde(default = "IndexerGrpcProcessorConfig::default_gap_detection_batch_size")]
    pub gap_detection_batch_size: u64,
    // Maximum number of batches "missing" before we assume we have an issue with gaps and abort
    #[serde(default = "IndexerGrpcProcessorConfig::default_gap_detection_batch_size")]
    pub parquet_gap_detection_batch_size: u64,
    // Number of protobuff transactions to send per chunk to the processor tasks
    #[serde(default = "IndexerGrpcProcessorConfig::default_pb_channel_txn_chunk_size")]
    pub pb_channel_txn_chunk_size: usize,
    // Number of rows to insert, per chunk, for each DB table. Default per table is ~32,768 (2**16/2)
    #[serde(default = "AHashMap::new")]
    pub per_table_chunk_sizes: AHashMap<String, usize>,
    pub enable_verbose_logging: Option<bool>,

    #[serde(default = "IndexerGrpcProcessorConfig::default_grpc_response_item_timeout_in_secs")]
    pub grpc_response_item_timeout_in_secs: u64,

    #[serde(default)]
    pub transaction_filter: TransactionFilter,
    // String vector for deprecated tables to skip db writes
    #[serde(default)]
    pub deprecated_tables: HashSet<String>,
}

impl IndexerGrpcProcessorConfig {
    pub fn new(
        processor_config: ProcessorConfig,
        postgres_connection_string: Option<String>,
        indexer_grpc_data_service_address: Option<Url>,
        grpc_http2_config: IndexerGrpcHttp2Config,
        auth_token: String,
        ending_version: Option<u64>,
        number_concurrent_processing_tasks: Option<usize>,
        db_pool_size: Option<u32>,
        gap_detection_batch_size: u64,
        parquet_gap_detection_batch_size: u64,
        pb_channel_txn_chunk_size: usize,
        per_table_chunk_sizes: AHashMap<String, usize>,
        enable_verbose_logging: Option<bool>,
        transaction_filter: TransactionFilter,
        grpc_response_item_timeout_in_secs: u64,
        deprecated_tables: HashSet<String>,
        starting_version: Option<u64>,
    ) -> Result<Self> {
        // Load .env file if it exists
        dotenv().ok();

        // Get starting version from environment variable if available
        let env_starting_version = env::var("PROCESSOR_STARTING_VERSION")
            .ok()
            .and_then(|v| v.parse::<u64>().ok());

        // Get Postgres connection string from environment variable if available
        let postgres_connection_string = env::var("PROCESSOR_POSTGRES_CONNECTION_STRING")
            .ok()
            .or(postgres_connection_string)
            .ok_or_else(|| {
                error!("Postgres connection string not configured. Please set PROCESSOR_POSTGRES_CONNECTION_STRING environment variable or provide it in the config file.");
                anyhow::anyhow!("Postgres connection string not configured")
            })?;

        // Get GRPC address from environment variable if available
        let indexer_grpc_data_service_address = env::var("PROCESSOR_INDEXER_GRPC_DATA_SERVICE_ADDRESS")
            .ok()
            .and_then(|addr| Url::parse(&addr).ok())
            .or(indexer_grpc_data_service_address)
            .ok_or_else(|| {
                error!("Indexer GRPC data service address not configured. Please set PROCESSOR_INDEXER_GRPC_DATA_SERVICE_ADDRESS environment variable or provide it in the config file.");
                anyhow::anyhow!("Indexer GRPC data service address not configured")
            })?;

        // Get auth token from environment variable if available
        let auth_token = env::var("PROCESSOR_AUTH_TOKEN")
            .ok()
            .unwrap_or(auth_token);

        // Get ending version from environment variable if available
        let env_ending_version = env::var("PROCESSOR_ENDING_VERSION")
            .ok()
            .and_then(|v| v.parse::<u64>().ok());

        // Get number of concurrent processing tasks from environment variable if available
        let number_concurrent_processing_tasks = env::var("PROCESSOR_NUMBER_CONCURRENT_TASKS")
            .ok()
            .and_then(|v| v.parse::<usize>().ok())
            .or(number_concurrent_processing_tasks);

        // Get DB pool size from environment variable if available
        let db_pool_size = env::var("PROCESSOR_DB_POOL_SIZE")
            .ok()
            .and_then(|v| v.parse::<u32>().ok())
            .or(db_pool_size);

        // Get gap detection batch size from environment variable if available
        let gap_detection_batch_size = env::var("PROCESSOR_GAP_DETECTION_BATCH_SIZE")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(gap_detection_batch_size);

        // Get parquet gap detection batch size from environment variable if available
        let parquet_gap_detection_batch_size = env::var("PROCESSOR_PARQUET_GAP_DETECTION_BATCH_SIZE")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(parquet_gap_detection_batch_size);

        // Get PB channel transaction chunk size from environment variable if available
        let pb_channel_txn_chunk_size = env::var("PROCESSOR_PB_CHANNEL_TXN_CHUNK_SIZE")
            .ok()
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(pb_channel_txn_chunk_size);

        // Get enable verbose logging from environment variable if available
        let enable_verbose_logging = env::var("PROCESSOR_ENABLE_VERBOSE_LOGGING")
            .ok()
            .and_then(|v| v.parse::<bool>().ok())
            .or(enable_verbose_logging);

        // Get GRPC response item timeout from environment variable if available
        let grpc_response_item_timeout_in_secs = env::var("PROCESSOR_GRPC_RESPONSE_ITEM_TIMEOUT_SECS")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(grpc_response_item_timeout_in_secs);

        Ok(Self {
            processor_config,
            postgres_connection_string: Some(postgres_connection_string),
            indexer_grpc_data_service_address: Some(indexer_grpc_data_service_address),
            grpc_http2_config,
            auth_token,
            starting_version: env_starting_version.or(starting_version),
            ending_version: env_ending_version.or(ending_version),
            number_concurrent_processing_tasks,
            db_pool_size,
            gap_detection_batch_size,
            parquet_gap_detection_batch_size,
            pb_channel_txn_chunk_size,
            per_table_chunk_sizes,
            enable_verbose_logging,
            transaction_filter,
            grpc_response_item_timeout_in_secs,
            deprecated_tables,
        })
    }

    pub const fn default_gap_detection_batch_size() -> u64 {
        DEFAULT_GAP_DETECTION_BATCH_SIZE
    }

    pub const fn default_query_retries() -> u32 {
        QUERY_DEFAULT_RETRIES
    }

    pub const fn default_query_retry_delay_ms() -> u64 {
        QUERY_DEFAULT_RETRY_DELAY_MS
    }

    /// Make the default very large on purpose so that by default it's not chunked
    /// This prevents any unexpected changes in behavior
    pub const fn default_pb_channel_txn_chunk_size() -> usize {
        100_000
    }

    /// Default timeout for grpc response item in seconds. Defaults to 60 seconds.
    pub const fn default_grpc_response_item_timeout_in_secs() -> u64 {
        60
    }
}

#[async_trait::async_trait]
impl RunnableConfig for IndexerGrpcProcessorConfig {
    async fn run(&self) -> Result<()> {
        let mut worker = Worker::new(
            self.processor_config.clone(),
            self.postgres_connection_string.clone().unwrap(),
            self.indexer_grpc_data_service_address.clone().unwrap(),
            self.grpc_http2_config.clone(),
            self.auth_token.clone(),
            self.starting_version,
            self.ending_version,
            self.number_concurrent_processing_tasks,
            self.db_pool_size,
            self.gap_detection_batch_size,
            self.parquet_gap_detection_batch_size,
            self.pb_channel_txn_chunk_size,
            self.per_table_chunk_sizes.clone(),
            self.enable_verbose_logging,
            self.transaction_filter.clone(),
            self.grpc_response_item_timeout_in_secs,
            self.deprecated_tables.clone(),
        )
        .await
        .context("Failed to build worker")?;
        worker.run().await;
        Ok(())
    }

    fn get_server_name(&self) -> String {
        // Get the part before the first _ and trim to 12 characters.
        let before_underscore = self
            .processor_config
            .name()
            .split('_')
            .next()
            .unwrap_or("unknown");
        before_underscore[..before_underscore.len().min(12)].to_string()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct IndexerGrpcHttp2Config {
    /// Indexer GRPC http2 ping interval in seconds. Defaults to 30.
    /// Tonic ref: https://docs.rs/tonic/latest/tonic/transport/channel/struct.Endpoint.html#method.http2_keep_alive_interval
    indexer_grpc_http2_ping_interval_in_secs: u64,

    /// Indexer GRPC http2 ping timeout in seconds. Defaults to 10.
    indexer_grpc_http2_ping_timeout_in_secs: u64,

    /// Seconds before timeout for grpc connection.
    indexer_grpc_connection_timeout_secs: u64,
}

impl IndexerGrpcHttp2Config {
    pub fn grpc_http2_ping_interval_in_secs(&self) -> Duration {
        Duration::from_secs(self.indexer_grpc_http2_ping_interval_in_secs)
    }

    pub fn grpc_http2_ping_timeout_in_secs(&self) -> Duration {
        Duration::from_secs(self.indexer_grpc_http2_ping_timeout_in_secs)
    }

    pub fn grpc_connection_timeout_secs(&self) -> Duration {
        Duration::from_secs(self.indexer_grpc_connection_timeout_secs)
    }
}

impl Default for IndexerGrpcHttp2Config {
    fn default() -> Self {
        Self {
            indexer_grpc_http2_ping_interval_in_secs: 30,
            indexer_grpc_http2_ping_timeout_in_secs: 10,
            indexer_grpc_connection_timeout_secs: 5,
        }
    }
}
