use binance::api::*;
use binance::market::*;
use binance::model::KlineSummary;

use chrono::{TimeZone, Utc};

pub fn market_data() {
    let market: Market = Binance::new(None, None);

    let coin_pair = "BNBBUSD";

    // Order book at default depth
    match market.get_depth(coin_pair) {
        Ok(answer) => println!("BNBBUSD Depth: {:?}\n\n", answer),
        Err(e) => println!("Error: {}", e),
    }

    // Order book at depth 500
    match market.get_custom_depth(coin_pair, 500) {
        Ok(answer) => println!("BNBBUSD Top500 Depth: {:?}\n\n", answer),
        Err(e) => println!("Error: {}", e),
    }

    // Latest price for ALL symbols
    match market.get_all_prices() {
        Ok(answer) => println!("{:?}\n\n", answer),
        Err(e) => println!("Error: {:?}", e),
    }

    // Latest price for ONE symbol
    match market.get_price(coin_pair) {
        Ok(answer) => println!("BNBBUSD Price: {:?}\n\n", answer),
        Err(e) => println!("Error: {:?}", e),
    }

    // Current average price for ONE symbol
    match market.get_average_price(coin_pair) {
        Ok(answer) => println!("{:?}\n\n", answer),
        Err(e) => println!("Error: {:?}", e),
    }

    // Best price/qty on the order book for ALL symbols
    match market.get_all_book_tickers() {
        Ok(answer) => println!("{:?}\n\n", answer),
        Err(e) => println!("Error: {:?}", e),
    }

    // Best price/qty on the order book for ONE symbol
    match market.get_book_ticker(coin_pair) {
        Ok(answer) => println!(
            "Bid Price: {}, Ask Price: {}\n\n",
            answer.bid_price, answer.ask_price
        ),
        Err(e) => println!("Error: {:?}", e),
    }

    // 24hr ticker price change statistics
    match market.get_24h_price_stats(coin_pair) {
        Ok(answer) => println!(
            "Open Price: {}, Higher Price: {}, Lower Price: {:?}\n\n",
            answer.open_price, answer.high_price, answer.low_price
        ),
        Err(e) => println!("Error: {:?}", e),
    }

    // last 10 5min klines (candlesticks) for a symbol:
    match market.get_klines(coin_pair, "5m", 10, None, None) {
        Ok(klines) => {
            match klines {
                binance::model::KlineSummaries::AllKlineSummaries(klines) => {
                    let kline: KlineSummary = klines[0].clone(); // You need to iterate over the klines
                    println!(
                        "Open: {}, High: {}, Low: {}\n\n",
                        kline.open, kline.high, kline.low
                    )
                }
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}

pub fn account_data(api_key: Option<String>, secret_key: Option<String>) {
    use binance::account::*;
    use binance::api::*;

    // let api_key = Some("YOUR_API_KEY".into());
    // let secret_key = Some("YOUR_SECRET_KEY".into());

    let account: Account = Binance::new(api_key, secret_key);

    let coin_pair = "DOTBUSD";
    let coin = "DOT";

    match account.get_account() {
        Ok(answer) => {
            for balance in answer.balances {
                let free: f64 = balance.free.parse().unwrap();
                let locked: f64 = balance.locked.parse().unwrap();
                if free + locked > 0.0 {
                    println!(
                        "{:8} balance: \tfree: {:20}, \tlocked: {:20}",
                        balance.asset, free, locked
                    );
                }
            }
        }
        Err(e) => println!("Error: {:?}", e),
    }

    // match account.get_open_orders(coin_pair) {
    //     Ok(answer) => println!("open orders: {:?}\n\n", answer),
    //     Err(e) => println!("Error: {:?}", e),
    // }

    // match account.limit_buy(coin_pair, 10, 0.014000) {
    //     Ok(answer) => println!("{:?}", answer),
    //     Err(e) => println!("Error: {:?}", e),
    // }
    //
    // match account.market_buy(coin_pair, 5) {
    //     Ok(answer) => println!("{:?}", answer),
    //     Err(e) => println!("Error: {:?}", e),
    // }

    // match account.limit_sell(coin_pair, 10, 0.035000) {
    //     Ok(answer) => println!("{:?}", answer),
    //     Err(e) => println!("Error: {:?}", e),
    // }
    //
    // match account.market_sell(coin_pair, 5) {
    //     Ok(answer) => println!("{:?}", answer),
    //     Err(e) => println!("Error: {:?}", e),
    // }

    // match account.custom_order(coin_pair, 9999, 0.0123, "SELL", "LIMIT", "IOC") {
    //     Ok(answer) => println!("{:?}", answer),
    //     Err(e) => println!("Error: {:?}", e),
    // }

    let order_id = 1_957_528;
    match account.order_status(coin_pair, order_id) {
        Ok(answer) => println!("order status: {:?}\n\n", answer),
        Err(e) => println!("Error: {:?}", e),
    }

    // match account.cancel_order(coin_pair, order_id) {
    //     Ok(answer) => println!("{:?}", answer),
    //     Err(e) => println!("Error: {:?}", e),
    // }

    // match account.cancel_all_open_orders(coin_pair) {
    //     Ok(answer) => println!("{:?}", answer),
    //     Err(e) => println!("Error: {:?}", e),
    // }

    match account.get_balance(coin) {
        Ok(answer) => {
            println!("get balance: {:?}\n\n", answer);
        }
        Err(e) => println!("Error: {:?}", e),
    }

    // my_trades:
    match account.trade_history(coin_pair) {
        Ok(answer) => {
            println!("coin pair: {:?} trade history:", coin_pair);
            for trade in answer {
                let ts = Utc.timestamp((trade.time as i64) / 1000, 0); // fix
                println!(
                    "{:?},  id={:?}, {:?}, price: {:?}, qty: {:?}",
                    ts.to_string(),
                    trade.id,
                    coin_pair,
                    trade.price,
                    trade.qty,
                );
            }
        }
        Err(e) => println!("Error: {:?}", e),
    }
}
