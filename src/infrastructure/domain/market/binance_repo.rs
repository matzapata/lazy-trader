use crate::domain::market::{
    kline_data::{MarketKlineData, TMarketKlineDataRepo},
    market::{Interval, Market},
};
use reqwest::{Client, StatusCode};
use serde::{de, Deserialize, Deserializer, Serialize};
use serde_json::Value;

static BINANCE_URL: &str = "https://api.binance.com/api/v3";

pub struct BinanceMarketKlineDataRepo {
    client: Client,
}

impl BinanceMarketKlineDataRepo {
    pub fn new(client: Client) -> Self {
        BinanceMarketKlineDataRepo { client }
    }
}

#[async_trait::async_trait]
impl TMarketKlineDataRepo for BinanceMarketKlineDataRepo {
    async fn get_klines(&self, market: &Market) -> Option<Vec<MarketKlineData>> {
        let interval_key = match market.interval {
            Interval::D1 => "1d",
            Interval::H1 => "1h",
        };

        let req_url = format!(
            "{}/klines?symbol={}&interval={}&limit={}",
            BINANCE_URL, market.id, interval_key, market.limit
        );
        let result = self.client.get(&req_url).send().await.unwrap();

        let data: Vec<MarketKlineData> = match result.status() {
            StatusCode::OK => {
                let json_value = result.json().await.unwrap();
                let kline_dtos: Vec<KlineDataDto> = serde_json::from_value(json_value).unwrap();
                kline_dtos.into_iter().map(MarketKlineData::from).collect()
            }
            _ => {
                println!("StatusCode: {}", result.status());
                println!("Message: {:?}", result.text().await);
                return None;
            }
        };

        Some(data)
    }

    async fn get_price(&self, market: &Market) -> Result<f64, Box<dyn std::error::Error>> {
        let req_url = format!("{}/ticker/price?symbol={}", BINANCE_URL, market.id);
        let result = self.client.get(&req_url).send().await.unwrap();

        if result.status() != StatusCode::OK {
            return Err(format!("StatusCode: {}", result.status()).into());
        }

        let json_value: Value = result.json::<Value>().await.unwrap();
        let price: f64 = json_value.get("price").unwrap().as_str().unwrap().parse().unwrap();
        
        Ok(price)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct KlineDataDto {
    pub open_time: i64,
    #[serde(deserialize_with = "de_float_from_str")]
    pub open: f64,
    #[serde(deserialize_with = "de_float_from_str")]
    pub high: f64,
    #[serde(deserialize_with = "de_float_from_str")]
    pub low: f64,
    #[serde(deserialize_with = "de_float_from_str")]
    pub close: f64,
    #[serde(deserialize_with = "de_float_from_str")]
    pub volume: f64,
    pub close_time: i64,
    #[serde(deserialize_with = "de_float_from_str")]
    pub quote_asset_volume: f64,
    pub number_of_trades: usize,
    #[serde(deserialize_with = "de_float_from_str")]
    pub take_buy_base_asset_volume: f64,
    #[serde(deserialize_with = "de_float_from_str")]
    pub take_buy_quote_asset_volume: f64,
    #[serde(deserialize_with = "de_float_from_str")]
    pub ignore: f64,
}

pub fn de_float_from_str<'a, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'a>,
{
    let str_val = String::deserialize(deserializer)?;
    str_val.parse::<f64>().map_err(de::Error::custom)
}

impl From<KlineDataDto> for MarketKlineData {
    fn from(data: KlineDataDto) -> Self {
        MarketKlineData {
            open_time: data.open_time,
            open: data.open,
            high: data.high,
            low: data.low,
            close: data.close,
            volume: data.volume,
            close_time: data.close_time,
            quote_asset_volume: data.quote_asset_volume,
            number_of_trades: data.number_of_trades,
            take_buy_base_asset_volume: data.take_buy_base_asset_volume,
            take_buy_quote_asset_volume: data.take_buy_quote_asset_volume,
            ignore: data.ignore,
        }
    }
}
