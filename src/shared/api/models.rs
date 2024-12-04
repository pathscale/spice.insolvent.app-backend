use ethers::types::Chain;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use web3::types::Address;

#[derive(Debug, Clone)]
pub struct TokenAddress {
    pub address: Address,
    pub chain: Chain,
}

#[derive(Debug, Clone)]
pub struct CoinMarketCapTokenInfo {
    pub cmc_id: u64,
    pub name: String,
    pub symbol: String,
    pub slug: String,
    pub addresses: Vec<TokenAddress>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapCoinPlatform {
    pub id: u64,
    pub name: String,
    pub symbol: String,
    pub slug: String,
    pub token_address: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MapCoinInfo {
    pub id: i32,
    pub rank: i32,
    pub name: String,
    pub symbol: String,
    pub slug: String,
    pub is_active: i32,
    pub first_historical_data: String,
    pub last_historical_data: String,
    pub platform: Option<MapCoinPlatform>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NativeCoin {
    pub id: String,
    pub name: String,
    pub symbol: String,
    pub slug: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Platform2 {
    pub name: String,
    pub coin: NativeCoin,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContractAddress {
    pub contract_address: String,
    pub platform: Platform2,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoinInfo {
    pub id: i64,
    pub name: String,
    pub symbol: String,
    pub category: String,
    pub slug: String,
    pub subreddit: String,
    #[serde(default, rename = "tag-names")]
    pub tag_names: Option<Vec<String>>,
    #[serde(rename = "tag-groups")]
    pub tag_groups: Option<Vec<String>>,
    pub twitter_username: String,
    pub is_hidden: i64,
    pub date_launched: Option<Value>,
    #[serde(default)]
    pub contract_address: Vec<ContractAddress>,
    pub self_reported_circulating_supply: Option<Value>,
    pub self_reported_tags: Option<Value>,
    pub self_reported_market_cap: Option<Value>,
    pub infinite_supply: bool,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuoteToken {
    pub price: f64,
    pub volume_24h: f64,
    pub volume_change_24h: f64,
    pub percent_change_1h: f64,
    pub percent_change_24h: f64,
    pub percent_change_7d: f64,
    pub market_cap: f64,
    pub market_cap_dominance: f64,
    pub fully_diluted_market_cap: f64,
    pub last_updated: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListingLatestToken {
    pub id: i64,
    pub name: String,
    pub symbol: String,
    pub slug: String,
    pub cmc_rank: i64,
    pub num_market_pairs: f64,
    pub circulating_supply: f64,
    pub total_supply: f64,
    pub max_supply: Option<f64>,
    pub infinite_supply: bool,
    pub last_updated: String,
    pub date_added: String,
    pub tags: Vec<String>,
    pub platform: Option<MapCoinPlatform>,
    pub self_reported_circulating_supply: Option<f64>,
    pub self_reported_market_cap: Option<f64>,
    pub quote: HashMap<String, QuoteToken>,
}
