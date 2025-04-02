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
pub struct SwapDetails {
    /// All intent hashes that took part in this swap
    #[serde(rename = "intentHashes")]
    pub intent_hashes: Vec<String>,
    /// All Near transactions executed for this swap
    #[serde(rename = "nearTxHashes")]
    pub near_tx_hashes: Vec<String>,
    /// Exact amount of `originToken` after trade was settled
    #[serde(rename = "amountIn", skip_serializing_if = "Option::is_none")]
    pub amount_in: Option<String>,
    /// Exact amount of `originToken` after trade was settled in readable format
    #[serde(rename = "amountInFormatted", skip_serializing_if = "Option::is_none")]
    pub amount_in_formatted: Option<String>,
    /// Exact amount of `originToken` equivalent in USD
    #[serde(rename = "amountInUsd", skip_serializing_if = "Option::is_none")]
    pub amount_in_usd: Option<String>,
    /// Exact amount of `destinationToken` after trade was settled
    #[serde(rename = "amountOut", skip_serializing_if = "Option::is_none")]
    pub amount_out: Option<String>,
    /// Exact amount of `destinationToken` in readable format
    #[serde(rename = "amountOutFormatted", skip_serializing_if = "Option::is_none")]
    pub amount_out_formatted: Option<String>,
    /// Exact amount of `destinationToken` equivalent in USD
    #[serde(rename = "amountOutUsd", skip_serializing_if = "Option::is_none")]
    pub amount_out_usd: Option<String>,
    /// Actual slippage
    #[serde(rename = "slippage", skip_serializing_if = "Option::is_none")]
    pub slippage: Option<f64>,
    /// Hashes and explorer URLs for all transactions on the origin chain
    #[serde(rename = "originChainTxHashes")]
    pub origin_chain_tx_hashes: Vec<models::TransactionDetails>,
    /// Hashes and explorer URLs for all transactions on the destination chain
    #[serde(rename = "destinationChainTxHashes")]
    pub destination_chain_tx_hashes: Vec<models::TransactionDetails>,
    /// Amount of `originAsset` that got transferred to `refundTo`
    #[serde(rename = "refundedAmount", skip_serializing_if = "Option::is_none")]
    pub refunded_amount: Option<String>,
    /// Refunded amount in readable format
    #[serde(
        rename = "refundedAmountFormatted",
        skip_serializing_if = "Option::is_none"
    )]
    pub refunded_amount_formatted: Option<String>,
    /// Refunded amount equivalent in USD
    #[serde(rename = "refundedAmountUsd", skip_serializing_if = "Option::is_none")]
    pub refunded_amount_usd: Option<String>,
}

impl SwapDetails {
    pub fn new(
        intent_hashes: Vec<String>,
        near_tx_hashes: Vec<String>,
        origin_chain_tx_hashes: Vec<models::TransactionDetails>,
        destination_chain_tx_hashes: Vec<models::TransactionDetails>,
    ) -> SwapDetails {
        SwapDetails {
            intent_hashes,
            near_tx_hashes,
            amount_in: None,
            amount_in_formatted: None,
            amount_in_usd: None,
            amount_out: None,
            amount_out_formatted: None,
            amount_out_usd: None,
            slippage: None,
            origin_chain_tx_hashes,
            destination_chain_tx_hashes,
            refunded_amount: None,
            refunded_amount_formatted: None,
            refunded_amount_usd: None,
        }
    }
}
