use binance_async::{
    account::*, api::*, config::Config, errors::Error as BinanceLibError, general::*, market::*,
    rest_model::*, wallet::*,
};

use binance_async::wallet::*;

use chrono::{Duration, TimeZone, Utc};
use log::{debug, error, info, warn};
#[allow(unused_imports)]
use pretty_env_logger;

pub struct MarketService {
    client: Market,
}

impl MarketService {
    pub fn new() -> Self {
        Self { client: Binance::new(None, None) }
    }

    pub async fn get_depth(&self, symbol: &str) {
        match self.client.get_depth(symbol).await {
            Ok(answer) => info!("get_depth: {:?}", answer),
            Err(e) => error!("Error: {:?}", e),
        }
    }

    // 币价查询:
    pub async fn get_all_prices(&self) {
        match self.client.get_all_prices().await {
            Ok(answer) => {
                info!("💰 get_all_prices:");
                match answer {
                    Prices::AllPrices(prices) =>
                        for item in prices.iter() {
                            info!("\t💎 price: {:?}", item);
                        },
                }
            },
            Err(e) => error!("Error: {:?}", e),
        }
    }

    // 单个交易对价格查询:
    pub async fn get_price(&self, symbol: &str) {
        match self.client.get_price(symbol).await {
            Ok(answer) => {
                info!("get_price: {:?}", answer);
            },
            Err(e) => error!("Error: {:?}", e),
        }
    }
}

pub async fn do_market_cmd() {
    let cli = MarketService::new();

    // cli.get_depth("BNBETH").await;
    // cli.get_all_prices().await;
    cli.get_price("BNBUSDT").await;

    warn!("do market cmd done.")
}
