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
                    Prices::AllPrices(prices) => {
                        for item in prices.iter() {
                            info!("\t💎 price: {:?}", item);
                        }
                    },
                }
            },
            Err(e) => error!("Error: {:?}", e),
        }
    }

    // 单个交易对价格查询: // Latest price for ONE symbol
    pub async fn get_price(&self, symbol: &str) {
        match self.client.get_price(symbol).await {
            Ok(answer) => {
                info!("get_price: {:?}", answer);
            },
            Err(e) => error!("Error: {:?}", e),
        }
    }

    // 平均价格: 5min/avg
    pub async fn get_average_price(&self, symbol: &str) {
        // Current average price for ONE symbol
        match self.client.get_average_price(symbol).await {
            Ok(answer) => info!("get_average_price: symbol: {}, {:?}", symbol, answer),
            Err(e) => error!("Error: {:?}", e),
        }
    }

    pub async fn get_all_book_tickers(&self) {
        // Best price/qty on the order book for ALL symbols
        match self.client.get_all_book_tickers().await {
            Ok(answer) => info!("get_all_book_tickers: {:?}", answer),
            Err(e) => error!("Error: {:?}", e),
        }
    }

    pub async fn get_book_ticker(&self, symbol: &str) {
        // Best price/qty on the order book for ONE symbol
        match self.client.get_book_ticker(symbol).await {
            Ok(answer) => info!(
                "get_book_ticker: symbol: {}, Bid Price: {}, Ask Price: {}",
                symbol, answer.bid_price, answer.ask_price
            ),
            Err(e) => error!("Error: {:?}", e),
        }
    }

    pub async fn get_24h_price_stats(&self, symbol: &str) {
        // 24hr ticker price change statistics
        match self.client.get_24h_price_stats(symbol).await {
            Ok(answer) => info!(
                "get_24h_price_stats: Open Price: {}, Higher Price: {}, Lower Price: {:?}",
                answer.open_price, answer.high_price, answer.low_price
            ),
            Err(e) => error!("Error: {:?}", e),
        }
    }

    pub async fn get_klines(&self, symbol: &str) {
        // last 10 5min klines (candlesticks) for a symbol:
        match self.client.get_klines(symbol, "5m", 10, None, None).await {
            Ok(answer) => info!("get_klines: {:?}", answer),
            Err(e) => error!("Error: {:?}", e),
        }
    }

    pub async fn get_agg_trades(&self, symbol: &str) {
        // 10 latest (aggregated) trades
        match self.client.get_agg_trades(symbol, None, None, None, Some(10)).await {
            Ok(trades) => {
                let trade = &trades[0]; // You need to iterate over them
                info!(
                    "get_agg_trades: {} BNB Qty: {}, Price: {}",
                    if trade.maker { "SELL" } else { "BUY" },
                    trade.qty,
                    trade.price
                )
            },
            Err(e) => println!("Error: {:?}", e),
        }
    }
}

pub async fn do_market_cmd() {
    let cli = MarketService::new();

    let coin_pair = "BNBUSDT";

    // cli.get_depth("BNBETH").await;

    // cli.get_all_prices().await;
    cli.get_price(coin_pair).await;
    cli.get_average_price(coin_pair).await;

    // cli.get_all_book_tickers().await;
    cli.get_book_ticker(coin_pair).await;
    cli.get_24h_price_stats(coin_pair).await;

    // cli.get_klines(coin_pair).await;

    cli.get_agg_trades(coin_pair).await;

    warn!("do market cmd done.")
}
