/*
 * 1Click Swap API
 *
 * API for One-Click Swaps
 *
 * The version of the OpenAPI document: 0.1
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubmitDepositTxRequest {
    /// Transaction hash of your deposit
    #[serde(rename = "txHash")]
    pub tx_hash: String,
    /// Deposit address for the quote
    #[serde(rename = "depositAddress")]
    pub deposit_address: String,
}

impl SubmitDepositTxRequest {
    pub fn new(tx_hash: String, deposit_address: String) -> SubmitDepositTxRequest {
        SubmitDepositTxRequest {
            tx_hash,
            deposit_address,
        }
    }
}
