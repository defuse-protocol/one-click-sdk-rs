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
pub struct TokenResponse {
    /// Unique asset identifier
    #[serde(rename = "assetId")]
    pub asset_id: String,
    /// Number of decimals for the token
    #[serde(rename = "decimals")]
    pub decimals: f64,
    /// Blockchain associated with the token
    #[serde(rename = "blockchain")]
    pub blockchain: Blockchain,
    /// Token symbol (e.g. BTC, ETH)
    #[serde(rename = "symbol")]
    pub symbol: String,
    /// Current price of the token
    #[serde(rename = "price")]
    pub price: f64,
    /// Date when the price was last updated
    #[serde(rename = "priceUpdatedAt")]
    pub price_updated_at: String,
    /// Contract address of the token
    #[serde(rename = "contractAddress")]
    pub contract_address: String,
}

impl TokenResponse {
    pub fn new(asset_id: String, decimals: f64, blockchain: Blockchain, symbol: String, price: f64, price_updated_at: String, contract_address: String) -> TokenResponse {
        TokenResponse {
            asset_id,
            decimals,
            blockchain,
            symbol,
            price,
            price_updated_at,
            contract_address,
        }
    }
}
/// Blockchain associated with the token
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Blockchain {
    #[serde(rename = "near")]
    Near,
    #[serde(rename = "eth")]
    Eth,
    #[serde(rename = "base")]
    Base,
    #[serde(rename = "arb")]
    Arb,
    #[serde(rename = "btc")]
    Btc,
    #[serde(rename = "sol")]
    Sol,
    #[serde(rename = "ton")]
    Ton,
    #[serde(rename = "doge")]
    Doge,
    #[serde(rename = "xrp")]
    Xrp,
    #[serde(rename = "zec")]
    Zec,
    #[serde(rename = "gnosis")]
    Gnosis,
    #[serde(rename = "bera")]
    Bera,
    #[serde(rename = "bsc")]
    Bsc,
    #[serde(rename = "pol")]
    Pol,
}

impl Default for Blockchain {
    fn default() -> Blockchain {
        Self::Near
    }
}

