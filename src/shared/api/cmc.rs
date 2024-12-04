use super::models::*;
use crate::shared::{AssetInfoClient, AssetPriceByPeriod};
use async_trait::async_trait;
use chrono::{Duration, NaiveDate, Utc};
use dashmap::DashMap;
use ethers::types::Chain;
use eyre::*;
use lru::LruCache;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Client, Response, Url};
use serde::de::DeserializeOwned;
use serde_json::{from_value, Value};
use std::collections::HashMap;
use std::num::NonZeroUsize;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Mutex;
use tracing::*;

const API_KEY: &str = "ec6c4b09-03e6-4bd6-84f9-95406fc2ce81";
const BASE_URL: &str = "https://pro-api.coinmarketcap.com";
const LATEST_QUOTES_URL: &str = "/v2/cryptocurrency/quotes/latest";
const HISTORICAL_QUOTE_URL: &str = "/v2/cryptocurrency/quotes/historical";
const METADATA_URL: &str = "/v1/cryptocurrency/info";
const MAP_URL: &str = "/v1/cryptocurrency/map";
const LISTING_URL: &str = "/v1/cryptocurrency/listings/latest";

pub fn try_deserialize<T: DeserializeOwned>(data: Value) -> Result<T> {
    let data = serde_json::to_string(&data)?;
    let jd = &mut serde_json::Deserializer::from_str(&data);
    let value = serde_path_to_error::deserialize(jd)?;
    Ok(value)
}
#[derive(Debug)]
pub struct CoinMarketCap {
    client: Client,
    base_url: String,
    price_cache: Mutex<LruCache<(NaiveDate, String), f64>>,
    persistent_price_cache: DashMap<String, f64>,
    //no_reattempt_symbols: DashSet<String>,
}
impl CoinMarketCap {
    pub fn new_debug_key() -> Result<Self> {
        Self::new(API_KEY)
    }
    pub fn new(api_key: &str) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert("X-CMC_PRO_API_KEY", HeaderValue::from_str(api_key)?);
        headers.insert("Accept", HeaderValue::from_static("application/json"));
        headers.insert("Accept-Encoding", HeaderValue::from_static("deflate, gzip"));

        Ok(Self {
            base_url: BASE_URL.to_string(),
            client: Client::builder().default_headers(headers).build()?,
            price_cache: Mutex::new(LruCache::new(NonZeroUsize::new(30000).unwrap())),
            persistent_price_cache: DashMap::new(),
            //no_reattempt_symbols: DashSet::new(),
        })
    }

    pub async fn get_token_infos_by_symbol_v2(
        &self,
        symbols: &[String],
    ) -> Result<HashMap<String, CoinInfo>> {
        let mut url = self.metadata_url();
        self.append_url_params(&mut url, "symbol", symbols);
        self.append_url_params(&mut url, "aux", &["status".to_string()]);
        let tokens: HashMap<String, CoinInfo> = self.send_and_parse_response(&url).await?;

        Ok(tokens)
    }

    pub async fn get_usd_prices_by_symbol_with_options(
        &self,
        symbols: &[String],
        tolerate_errors: bool,
    ) -> Result<HashMap<String, f64>> {
        let begin = Instant::now();

        let date = Utc::now().date_naive();
        let mut token_prices: HashMap<String, f64> = HashMap::with_capacity(symbols.len());

        for symbol in symbols {
            if let Some(price) = self
                .price_cache
                .lock()
                .await
                .get(&(date, symbol.to_string()))
            {
                token_prices.insert(symbol.clone(), *price);
            }
        }
        let new_symbols = symbols
            .iter()
            .filter(|symbol| !token_prices.contains_key(*symbol))
            .cloned()
            .collect::<Vec<_>>();

        if !new_symbols.is_empty() {
            let mut url = self.price_url();
            self.append_url_params(&mut url, "symbol", &new_symbols);
            let payload: Value = self.send_and_parse_response(&url).await?;
            for symbol in new_symbols.into_iter() {
                let token = &payload[&symbol][0];
                if !tolerate_errors && token["is_active"].as_u64().context("status not found")? != 1
                {
                    bail!("token status not found for {}", symbol)
                }
                if let Some(price) = token["quote"]["USD"]["price"].as_f64() {
                    token_prices.insert(symbol.clone(), price);
                    self.price_cache.lock().await.put((date, symbol), price);
                } else if tolerate_errors {
                    token_prices.insert(symbol.clone(), 0.0);
                    self.price_cache.lock().await.put((date, symbol), 0.0);
                } else {
                    bail!("price not found for {}", symbol)
                }
            }
        }
        let duration = Instant::now() - begin;
        trace!("get_usd_prices_by_symbol duration: {:?}", duration);
        Ok(token_prices)
    }
    pub async fn lookup_usd_price_cache_by_symbol(&self, symbol: &str) -> Option<f64> {
        if self.persistent_price_cache.contains_key(symbol) {
            self.persistent_price_cache.get(symbol).map(|v| *v)
        }
        // else if !self.no_reattempt_symbols.contains(symbol) {
        //     warn!("CMC Cache does not contain a price for symbol: {}, attempting to fetch it.", symbol);
        //     let symbol_usd = self.get_quote_price_by_symbol(symbol.to_string(),"USD".to_string()).await;

        //     if let Result::Ok(symbol_usd) = symbol_usd {
        //         info!("Successfully fetched price for {symbol} after reattempt");
        //         self.persistent_price_cache.insert(symbol.to_string(), symbol_usd);
        //     } else {
        //         warn!("Failed to get USD price for {symbol} after reattempt!");
        //         self.no_reattempt_symbols.insert(symbol.to_string());
        //     }
        //     return None;
        // }
        else {
            trace!("CMC Cache does not contain a price for symbol: {}", symbol);
            return None;
        }
    }
    pub async fn get_usd_prices_by_symbol(
        &self,
        symbols: &[String],
    ) -> Result<HashMap<String, f64>> {
        self.get_usd_prices_by_symbol_with_options(symbols, false)
            .await
    }
    /**
        Quotes Historical v2
    Returns an interval of historic market quotes for any cryptocurrency based on time and interval parameters.

    Please note: This documentation relates to our updated V2 endpoint, which may be incompatible with our V1 versions. Documentation for deprecated endpoints can be found here.

    Technical Notes

    A historic quote for every "interval" period between your "time_start" and "time_end" will be returned.
    If a "time_start" is not supplied, the "interval" will be applied in reverse from "time_end".
    If "time_end" is not supplied, it defaults to the current time.
    At each "interval" period, the historic quote that is closest in time to the requested time will be returned.
    If no historic quotes are available in a given "interval" period up until the next interval period, it will be skipped.
    Implementation Tips

    Want to get the last quote of each UTC day? Don't use "interval=daily" as that returns the first quote. Instead use "interval=24h" to repeat a specific timestamp search every 24 hours and pass ex. "time_start=2019-01-04T23:59:00.000Z" to query for the last record of each UTC day.
    This endpoint supports requesting multiple cryptocurrencies in the same call. Please note the API response will be wrapped in an additional object in this case.
    Interval Options
    There are 2 types of time interval formats that may be used for "interval".

    The first are calendar year and time constants in UTC time:
    "hourly" - Get the first quote available at the beginning of each calendar hour.
    "daily" - Get the first quote available at the beginning of each calendar day.
    "weekly" - Get the first quote available at the beginning of each calendar week.
    "monthly" - Get the first quote available at the beginning of each calendar month.
    "yearly" - Get the first quote available at the beginning of each calendar year.

    The second are relative time intervals.
    "m": Get the first quote available every "m" minutes (60 second intervals). Supported minutes are: "5m", "10m", "15m", "30m", "45m".
    "h": Get the first quote available every "h" hours (3600 second intervals). Supported hour intervals are: "1h", "2h", "3h", "4h", "6h", "12h".
    "d": Get the first quote available every "d" days (86400 second intervals). Supported day intervals are: "1d", "2d", "3d", "7d", "14d", "15d", "30d", "60d", "90d", "365d".

    This endpoint is available on the following API plans:

    Basic
    Hobbyist (1 month)
    Startup (1 month)
    Standard (3 month)
    Professional (12 months)
    Enterprise (Up to 6 years)
    Cache / Update frequency: Every 5 minutes.
    Plan credit use: 1 call credit per 100 historical data points returned (rounded up) and 1 call credit per convert option beyond the first.
    CMC equivalent pages: Our historical cryptocurrency charts like coinmarketcap.com/currencies/bitcoin/#charts.


    PARAMETERS
    Query Parameters ?
     id
    string
    One or more comma-separated CoinMarketCap cryptocurrency IDs. Example: "1,2"

     symbol
    string
    Alternatively pass one or more comma-separated cryptocurrency symbols. Example: "BTC,ETH". At least one "id" or "symbol" is required for this request.

     time_start
    string
    Timestamp (Unix or ISO 8601) to start returning quotes for. Optional, if not passed, we'll return quotes calculated in reverse from "time_end".

     time_end
    string
    Timestamp (Unix or ISO 8601) to stop returning quotes for (inclusive). Optional, if not passed, we'll default to the current time. If no "time_start" is passed, we return quotes in reverse order starting from this time.

     count
    number [ 1 .. 10000 ]
    10
    The number of interval periods to return results for. Optional, required if both "time_start" and "time_end" aren't supplied. The default is 10 items. The current query limit is 10000.

     interval
    string
    "5m"
    "yearly""monthly""weekly""daily""hourly""5m""10m""15m""30m""45m""1h""2h""3h""4h""6h""12h""24h""1d""2d""3d""7d""14d""15d""30d""60d""90d""365d"
    Interval of time to return data points for. See details in endpoint description.

     convert
    string
    By default market quotes are returned in USD. Optionally calculate market quotes in up to 3 other fiat currencies or cryptocurrencies.

     convert_id
    string
    Optionally calculate market quotes by CoinMarketCap ID instead of symbol. This option is identical to convert outside of ID format. Ex: convert_id=1,2781 would replace convert=BTC,USD in your query. This parameter cannot be used when convert is used.

     aux
    string
    "price,volume,market_cap,circulating_supply,total_supply,quote_timestamp,is_active,is_fiat"
    Optionally specify a comma-separated list of supplemental data fields to return. Pass price,volume,market_cap,circulating_supply,total_supply,quote_timestamp,is_active,is_fiat,search_interval to include all auxiliary fields.

     skip_invalid
    boolean
    true
    Pass true to relax request validation rules. When requesting records on multiple cryptocurrencies an error is returned if no match is found for 1 or more requested cryptocurrencies. If set to true, invalid lookups will be skipped allowing valid cryptocurrencies to still be returned.


        */
    pub async fn get_usd_price_days_ago(
        &self,
        symbols: &[String],
        days: u32,
        tolerate_errors: bool,
    ) -> Result<HashMap<String, f64>> {
        let begin = Instant::now();
        let date = Utc::now().date_naive() - Duration::days(days as i64);
        let mut token_prices: HashMap<String, f64> = HashMap::with_capacity(symbols.len());

        for symbol in symbols {
            if let Some(price) = self
                .price_cache
                .lock()
                .await
                .get(&(date, symbol.to_string()))
            {
                token_prices.insert(symbol.clone(), *price);
            }
        }
        let new_symbols = symbols
            .iter()
            .filter(|symbol| !token_prices.contains_key(*symbol))
            .cloned()
            .collect::<Vec<_>>();

        if !new_symbols.is_empty() {
            let mut url = self.quotes_historical_url();
            let today = Utc::now();
            let ago = today - Duration::days(days as i64);

            self.append_url_params(&mut url, "symbol", symbols);
            self.append_url_params(&mut url, "time_start", &[ago.to_rfc3339()]);
            self.append_url_params(&mut url, "interval", &["daily".to_string()]);
            self.append_url_params(&mut url, "count", &["1".to_string()]);

            let payload: Value = self.send_and_parse_response(&url).await?;
            for symbol in new_symbols.into_iter() {
                let base = &payload[&symbol][0];
                let quote = &base["quotes"][0]["quote"]["USD"];
                let price = quote["price"].as_f64();
                if let Some(price) = price {
                    token_prices.insert(symbol.clone(), price);
                    self.price_cache.lock().await.put((date, symbol), price);
                } else if tolerate_errors {
                    token_prices.insert(symbol.clone(), 0.0);
                    self.price_cache.lock().await.put((date, symbol), 0.0);
                } else {
                    bail!("price not found for {}", symbol)
                }
            }
        }
        let duration = Instant::now() - begin;
        trace!("get_usd_price_days_ago duration: {:?}", duration);
        Ok(token_prices)
    }
    pub async fn get_quote_price_by_symbol(
        &self,
        base_symbol: String,
        quote_symbol: String,
    ) -> Result<f64> {
        let begin = Instant::now();
        let mut url = self.price_url();
        self.append_url_params(&mut url, "symbol", &[base_symbol.clone()]);
        self.append_url_params(&mut url, "convert", &[quote_symbol.clone()]);
        let payload: Value = self.send_and_parse_response(&url).await?;
        let base = &payload[base_symbol][0];
        let quote = &base["quote"][quote_symbol];
        let price = quote["price"].as_f64().context("price not found")?;
        let duration = Instant::now() - begin;
        trace!("get_quote_price_by_symbol duration: {:?}", duration);
        Ok(price)
    }

    pub async fn get_top_25_coins(&self) -> Result<Vec<MapCoinInfo>> {
        let mut url = self.map_url();
        self.append_url_params(&mut url, "limit", &vec!["25".to_string()]);
        self.append_url_params(&mut url, "sort", &vec!["cmc_rank".to_string()]);

        let data: Vec<MapCoinInfo> = self.send_and_parse_response(&url).await?;
        Ok(data)
    }
    fn price_url(&self) -> Url {
        Url::parse(&format!("{}{}", self.base_url, LATEST_QUOTES_URL)).unwrap()
    }
    fn quotes_historical_url(&self) -> Url {
        Url::parse(&format!("{}{}", self.base_url, HISTORICAL_QUOTE_URL)).unwrap()
    }
    fn metadata_url(&self) -> Url {
        Url::parse(&format!("{}{}", self.base_url, METADATA_URL)).unwrap()
    }
    fn map_url(&self) -> Url {
        Url::parse(&format!("{}{}", self.base_url, MAP_URL)).unwrap()
    }

    fn append_url_params(&self, url: &mut Url, param_key: &str, param_values: &[String]) -> () {
        let mut params = url.query_pairs_mut();
        params.append_pair(param_key, &param_values.join(","));
    }
    pub async fn send_and_parse_response<T: DeserializeOwned>(&self, url: &Url) -> Result<T> {
        trace!("Request: {}", url);
        let response = self.client.get(url.clone()).send().await?;
        self.parse_response(response).await
    }

    pub async fn parse_response<T: DeserializeOwned>(&self, response: Response) -> Result<T> {
        let text = response.text().await?;
        trace!("Response: {}", text);

        let json = Value::from_str(&text)?;
        if let Some(err) = json["status"].get("error_message") {
            if !err.is_null() {
                bail!("error_message: {}", err);
            }
        }
        // Ok(try_deserialize(json["data"].clone())?)
        Ok(from_value(json["data"].clone())?)
    }
    pub fn coin_symbol_to_chain(&self, coin_symbol: &str) -> Result<Chain> {
        match coin_symbol {
            "ETH" => Ok(Chain::Mainnet),
            "BNB" => Ok(Chain::BinanceSmartChain),
            _ => bail!("chain not supported {}", coin_symbol),
        }
    }
    pub async fn fetch_latest_listing(&self) -> Result<()> {
        let mut offset = 1;
        let limit = 5000;
        loop {
            info!("Sleeping 10 seconds for CMC latest listing");
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

            let mut url: Url = format!("{}{}", self.base_url, LISTING_URL).parse()?;
            self.append_url_params(&mut url, "start", &vec![offset.to_string()]);
            self.append_url_params(&mut url, "limit", &vec![limit.to_string()]);
            self.append_url_params(&mut url, "sort", &vec!["market_cap".to_string()]);

            let resp: Vec<ListingLatestToken> = self.send_and_parse_response(&url).await?;
            let len = resp.len();

            for r in resp {
                let Some(price) = r.quote.get("USD") else {
                    warn!("price not found for {}", r.symbol);
                    continue;
                };
                self.persistent_price_cache
                    .insert(r.symbol.clone(), price.price);
            }
            if len < limit {
                break;
            }
            offset += limit;
        }

        // Get price for ETH specifically
        let eth = String::from("ETH");
        let eth_usd = self
            .get_quote_price_by_symbol(eth.clone(), "USD".to_string())
            .await;

        if let Result::Ok(eth_usd) = eth_usd {
            info!("Successfully fetched ETH price: {eth_usd} USD");
            self.persistent_price_cache.insert(eth, eth_usd);
        } else {
            warn!("Failed to get USD price for ETH")
        }

        Ok(())
    }
}

#[async_trait]
impl AssetInfoClient for CoinMarketCap {
    async fn get_usd_price_latest(&self, symbols: &[String]) -> Result<HashMap<String, f64>> {
        self.get_usd_prices_by_symbol(symbols).await
    }

    async fn get_usd_price_period(
        &self,
        symbols: &[String],
    ) -> Result<HashMap<String, AssetPriceByPeriod>> {
        let mut result = HashMap::new();
        let prices = self.get_usd_prices_by_symbol(symbols).await?;
        let prices_1d = self.get_usd_price_days_ago(symbols, 1, true).await?;
        let prices_7d = self.get_usd_price_days_ago(symbols, 7, true).await?;
        let prices_30d = self.get_usd_price_days_ago(symbols, 30, true).await?;
        for symbol in symbols {
            let price_latest = prices
                .get(symbol)
                .cloned()
                .context("could not get price of token that was just fetched")?;
            let price_1d = prices_1d.get(symbol).cloned();
            let price_7d = prices_7d.get(symbol).cloned();
            let price_30d = prices_30d.get(symbol).cloned();
            result.insert(
                symbol.clone(),
                AssetPriceByPeriod {
                    symbol: symbol.clone(),
                    price_latest,
                    price_1d,
                    price_7d,
                    price_30d,
                },
            );
        }
        Ok(result)
    }
}

pub async fn prefetch_prices(cmc_client: Arc<CoinMarketCap>, token_names: Vec<String>) {
    if let Err::<(), Error>(e) = async {
        for chunk in token_names.chunks_exact(30) {
            cmc_client
                .get_usd_prices_by_symbol_with_options(chunk, true)
                .await?;
        }
        for i in [7, 30] {
            for chunk in token_names.chunks_exact(30) {
                cmc_client.get_usd_price_days_ago(chunk, i, true).await?;
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
        }
        // wait until 00:00 UTC
        let now = Utc::now().timestamp();
        let next_day = (now / 86400 + 1) * 86400;
        tokio::time::sleep(tokio::time::Duration::from_secs(next_day as _)).await;
        // load prices every day
        loop {
            for chunk in token_names.chunks_exact(30) {
                cmc_client
                    .get_usd_prices_by_symbol_with_options(chunk, true)
                    .await?;
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(60 * 60 * 24)).await;
        }
    }
    .await
    {
        error!("load_token_prices error: {:?}", e);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use endpoint_libs::libs::log::{setup_logs, LogLevel};
    use std::{assert_eq, println, vec};
    use tracing::info;

    #[tokio::test]
    async fn test_get_usd_price_by_symbol() -> Result<()> {
        let cmc = CoinMarketCap::new_debug_key().unwrap();
        let prices = cmc
            .get_usd_prices_by_symbol(&vec!["ETH".to_string()])
            .await?;
        assert_eq!(prices.len(), 1);
        assert!(*prices.get("ETH").unwrap() > 0.0);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_cmc_top_25_tokens() -> Result<()> {
        setup_logs(LogLevel::Debug, None)?;
        let cmc = CoinMarketCap::new_debug_key().unwrap();
        let infos = cmc.get_top_25_coins().await?;
        info!("{:?}", infos);
        Ok(())
    }
    #[tokio::test]
    async fn test_get_quote_price_by_symbol() -> Result<()> {
        let cmc = CoinMarketCap::new_debug_key().unwrap();
        let price = cmc
            .get_quote_price_by_symbol("ETH".to_string(), "USDC".to_string())
            .await?;
        println!("PRICE: {:?}", price);
        assert!(price > 0.0);
        Ok(())
    }
    #[tokio::test]
    async fn test_get_price_in_usd_30_days_ago() -> Result<()> {
        setup_logs(LogLevel::Info, None)?;
        let cmc = CoinMarketCap::new_debug_key().unwrap();
        let price = cmc
            .get_usd_price_days_ago(&["ETH".to_string()], 30, false)
            .await?
            .get("ETH")
            .unwrap()
            .clone();
        println!("PRICE: {:?}", price);
        assert!(price > 0.0);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_token_info_v2() -> Result<()> {
        setup_logs(LogLevel::Debug, None)?;
        let cmc = CoinMarketCap::new_debug_key()?;
        let info = cmc
            .get_token_infos_by_symbol_v2(&["USDC".to_string()])
            .await?;
        info!("{:?}", info);
        Ok(())
    }
    #[tokio::test]
    async fn test_get_token_info() -> Result<()> {
        setup_logs(LogLevel::Debug, None)?;
        let cmc = CoinMarketCap::new_debug_key()?;
        cmc.fetch_latest_listing().await?;
        info!("fetched symbols: {}", cmc.persistent_price_cache.len());
        Ok(())
    }
}
