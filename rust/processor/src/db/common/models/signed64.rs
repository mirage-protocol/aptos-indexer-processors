// Copyright © Mirage Protocol

/**
 * This file defines deserialized rebase module types.
 */
use crate::utils::util::deserialize_from_string;
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Signed64 {
    pub negative: bool,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub magnitude: BigDecimal,
}

impl Signed64 {
    pub fn to_bigdecimal(&self) -> BigDecimal {
        if self.negative {
            -self.magnitude.clone()
        } else {
            self.magnitude.clone()
        }
    }
}
