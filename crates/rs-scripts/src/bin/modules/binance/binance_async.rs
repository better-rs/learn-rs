use binance_async::{
    account::*, api::*, config::Config, errors::Error as BinanceLibError, general::*, market::*,
    rest_model::*, wallet::*,
};

use binance_async::wallet::*;

use chrono::{Duration, TimeZone, Utc};
use log::{debug, error, info, warn};
#[allow(unused_imports)]
use pretty_env_logger;

// no auth:
pub async fn general() {
    let general: General = Binance::new(None, None);

    let ping = general.ping().await;
    match ping {
        Ok(answer) => info!("{:?}", answer),
        Err(err) => {
            match err {
                BinanceLibError::BinanceError { response } => match response.code {
                    -1000_i16 => error!("An unknown error occured while processing the request"),
                    _ => error!("Non-catched code {}: {}", response.code, response.msg),
                },
                _ => error!("Other errors: {:?}.", err),
            };
        },
    }

    let result = general.get_server_time().await;
    match result {
        Ok(answer) => info!("Server Time: {}", answer.server_time),
        Err(e) => error!("Error: {:?}", e),
    }

    let result = general.exchange_info().await;
    match result {
        Ok(answer) => {
            // info!("Exchange information: {:?}", answer);
            info!(
                "Exchange info: server_time={:?}, timezone={:?},",
                answer.server_time, answer.timezone,
            );
        },
        Err(e) => error!("Error: {:?}", e),
    }
}

// no auth:
pub async fn market_data() {
    let market: Market = Binance::new(None, None);

    // Order book
    match market.get_depth("BNBETH").await {
        Ok(answer) => info!("{:?}", answer),
        Err(e) => error!("Error: {:?}", e),
    }

    // Latest price for ALL symbols
    match market.get_all_prices().await {
        Ok(answer) => info!("{:?}", answer),
        Err(e) => error!("Error: {:?}", e),
    }

    // Latest price for ONE symbol
    match market.get_price("KNCETH").await {
        Ok(answer) => info!("{:?}", answer),
        Err(e) => error!("Error: {:?}", e),
    }

    // Current average price for ONE symbol
    match market.get_average_price("KNCETH").await {
        Ok(answer) => info!("{:?}", answer),
        Err(e) => error!("Error: {:?}", e),
    }

    // Best price/qty on the order book for ALL symbols
    match market.get_all_book_tickers().await {
        Ok(answer) => info!("{:?}", answer),
        Err(e) => error!("Error: {:?}", e),
    }

    // Best price/qty on the order book for ONE symbol
    match market.get_book_ticker("BNBETH").await {
        Ok(answer) => info!("Bid Price: {}, Ask Price: {}", answer.bid_price, answer.ask_price),
        Err(e) => error!("Error: {:?}", e),
    }

    // 24hr ticker price change statistics
    match market.get_24h_price_stats("BNBETH").await {
        Ok(answer) => info!(
            "Open Price: {}, Higher Price: {}, Lower Price: {:?}",
            answer.open_price, answer.high_price, answer.low_price
        ),
        Err(e) => error!("Error: {:?}", e),
    }

    // last 10 5min klines (candlesticks) for a symbol:
    match market.get_klines("BNBETH", "5m", 10, None, None).await {
        Ok(answer) => info!("{:?}", answer),
        Err(e) => error!("Error: {:?}", e),
    }

    // 10 latest (aggregated) trades
    match market.get_agg_trades("BNBETH", None, None, None, Some(10)).await {
        Ok(trades) => {
            let trade = &trades[0]; // You need to iterate over them
            println!(
                "{} BNB Qty: {}, Price: {}",
                if trade.maker { "SELL" } else { "BUY" },
                trade.qty,
                trade.price
            )
        },
        Err(e) => println!("Error: {:?}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_ts() {
        let now_at = Utc::now().timestamp_millis();
        let now = Utc.timestamp_millis(now_at).to_rfc3339();

        let duration_90days = Duration::days(90).num_milliseconds();

        let ts_90days_ago: i64 = Utc::now().timestamp_millis() - duration_90days;
        let day_90_ago = Utc.timestamp_millis(ts_90days_ago).to_rfc3339();

        eprintln!("💰 start time: {:?}", now_at); // 1655065755968 // 1655066033984
        eprintln!("💰 ts_90days_ago: {:?}", ts_90days_ago);
        eprintln!("💰 day range: [{:?}, {:?}]", now, day_90_ago);

        let day_at = Utc::now().timestamp_millis() - (60 * 60 * 24);
        let date_day = Utc::now().date();
        let day_2year_ago = Utc::now().timestamp_millis() - (60 * 60 * 24 * 365 * 2);

        let at_from_ts = Utc.timestamp_millis(1655065755968).to_rfc3339();

        eprintln!("💰 day_at: {:?}", day_at); // 1655065755968
        eprintln!("💰 date_day: {:?}", date_day); // 2020-04-01 // 2020-04-01T00:00:00+00:00
        eprintln!("💰 day_2year_ago: {:?}", day_2year_ago); // 1655065755968

        eprintln!("💰 at_from_ts: {:?}", at_from_ts); // 2020-04-01
    }
}
