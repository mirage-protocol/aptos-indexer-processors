// Copyright © Mirage Protocol

/**
 * This file defines deserialized rebase module types.
 */
use crate::utils::util::deserialize_from_string;
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RateLimiterConfig {
    pub name: String,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub window_duration_sec: BigDecimal,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub max_outflow: BigDecimal,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RateLimiter {
    #[serde(deserialize_with = "deserialize_from_string")]
    pub prev_qty: BigDecimal,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub window_start_sec: BigDecimal,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub cur_qty: BigDecimal,
    pub config: RateLimiterConfig
}

