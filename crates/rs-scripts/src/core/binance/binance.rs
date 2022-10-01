use binance::{account::Account, api::*, market::*, model::KlineSummary};

use chrono::{TimeZone, Utc};
use log::{debug, error, info, warn};
#[allow(unused_imports)]
use pretty_env_logger;

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
        Ok(answer) => {
            println!("Bid Price: {}, Ask Price: {}\n\n", answer.bid_price, answer.ask_price)
        },
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
                    println!("Open: {}, High: {}, Low: {}\n\n", kline.open, kline.high, kline.low)
                },
            }
        },
        Err(e) => println!("Error: {}", e),
    }
}

pub fn account_data(api_key: Option<String>, secret_key: Option<String>) {
    use binance::{account::*, api::*};

    // let api_key = Some("YOUR_API_KEY".into());
    // let secret_key = Some("YOUR_SECRET_KEY".into());

    let account: Account = Binance::new(api_key, secret_key);

    // 多个币种计算:
    let coins =
        vec!["BTC", "ETH", "BNB", "SOL", "FIL", "KSM", "MATIC", "SFP", "GLMR", "PHA", "AR", "DOT"];

    // 打印账号余额:
    get_all_balance(&account);

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

    // let order_id = 1_957_528;
    // match account.order_status(coin_pair, order_id) {
    //     Ok(answer) => println!("order status: {:?}\n\n", answer),
    //     Err(e) => println!("Error: {:?}", e),
    // }

    // match account.cancel_order(coin_pair, order_id) {
    //     Ok(answer) => println!("{:?}", answer),
    //     Err(e) => println!("Error: {:?}", e),
    // }

    // match account.cancel_all_open_orders(coin_pair) {
    //     Ok(answer) => println!("{:?}", answer),
    //     Err(e) => println!("Error: {:?}", e),
    // }

    // 批量计算多个币种:
    calc_avg_cost_by_coins(&account, &coins); // todo x: fix 参数类型, 改为 引用类型, 否则会出现 move 错误, rust 的生命周期

    // 查询币种余额:
    get_balance_by_coins(&account, &coins);
}

// 打印用户余额:
fn get_all_balance(account: &Account) {
    match account.get_account() {
        Ok(answer) => {
            for balance in answer.balances {
                let free: f64 = balance.free.parse().unwrap();
                let locked: f64 = balance.locked.parse().unwrap();

                // 显式资金>0的币种余额:
                if free + locked > 0.0 {
                    info!("🍄 {:?}: {:?}", balance.asset, balance);
                }
            }
        },
        Err(e) => error!("Error: {:?}", e),
    }
}

// 批量获取币种余额:
fn get_balance_by_coins(account: &Account, coins: &Vec<&str>) {
    // todo x: fix vec.iter(), not vec itself
    for coin in coins.iter() {
        match account.get_balance(coin.to_string()) {
            Ok(answer) => {
                info!("🍄 {}: {:?}", coin, answer);
            },
            Err(e) => error!("Error: {:?}", e),
        }
    }
}

fn calc_avg_cost_by_coins(account: &Account, coins: &Vec<&str>) {
    for coin in coins.iter() {
        let cli = account.clone();
        calc_avg_cost_by_coin(cli, coin);
    }
}

// 单个币种, 多个交易对计算:
fn calc_avg_cost_by_coin(account: Account, coin: &str) {
    // 使用 BUSD/USDT 购买过 DOT: // 多个交易对复合计算平均成本
    let coin_pairs = vec![
        (coin.to_owned() + "BUSD").to_string(), // 交易对
        (coin.to_owned() + "USDT").to_string(), // 交易对
    ];

    // 买单:
    let mut buy_total_qty: f64 = 0.0;
    let mut buy_total_cost: f64 = 0.0;
    let mut buy_avg_price: f64 = 0.0;

    // 卖单:
    let mut sell_total_qty: f64 = 0.0;
    let mut sell_total_cost: f64 = 0.0;
    let mut sell_avg_price: f64 = 0.0;

    // 当前平均持有成本:
    let mut current_avg_price: f64 = 0.0;
    let mut current_total_qty: f64 = 0.0;

    info!("💰 {:?} analyze:", coin);

    // 多个交易对合并计算:
    for coin_pair in coin_pairs {
        // my_trades:
        match account.trade_history(&coin_pair) {
            Ok(answer) => {
                debug!("coin pair: {:?} trade history:", coin_pair);
                // 计算:
                for trade in answer {
                    let ts = Utc.timestamp((trade.time as i64) / 1000, 0); // fix
                    // buy:
                    if trade.is_buyer {
                        // 买单: 总数量
                        buy_total_qty += trade.qty;
                        // 买单: 总成本
                        buy_total_cost += trade.qty * trade.price;

                        debug!(
                            "🍄 {:?}, buyer={:?},  id={:?}, {:?}, price: {:?}, qty: {:?}",
                            ts.to_string(),
                            trade.is_buyer,
                            trade.id,
                            coin_pair,
                            trade.price,
                            trade.qty,
                        );
                    } else {
                        // 卖单: 总数量
                        sell_total_qty += trade.qty;
                        // 卖单: 总成本
                        sell_total_cost += trade.qty * trade.price;

                        debug!(
                            "💰 {:?}, buyer={:?}, id={:?}, {:?}, price: {:?}, qty: {:?}",
                            ts.to_string(),
                            trade.is_buyer,
                            trade.id,
                            coin_pair,
                            trade.price,
                            trade.qty,
                        );
                    }
                }

                // 统计:
                buy_avg_price = buy_total_cost / buy_total_qty;
                sell_avg_price = sell_total_cost / sell_total_qty;

                current_total_qty = buy_total_qty - sell_total_qty;
                current_avg_price = (buy_total_cost - sell_total_cost) / current_total_qty;
            },
            Err(e) => error!("Error: {:?}", e),
        }

        info!(
            "🐛 {:?}, buy: total_qty: {:20?}, \t🐛 total_cost: {:20?}, \t🐛 avg_price: {:20?}",
            coin_pair, buy_total_qty, buy_total_cost, buy_avg_price
        );
        info!(
            "🐛 {:?}, sell: total_qty: {:20?}, \t🐛 total_cost: {:20?}, \t🐛 avg_price: {:20?}",
            coin_pair, sell_total_qty, sell_total_cost, sell_avg_price
        );
    }

    // 买单:
    warn!(
        "💎 buy: total_qty: {:10.20?}, \t💎 total_cost: {:20?}, \t💎 avg_price: {:20?}",
        buy_total_qty, buy_total_cost, buy_avg_price
    );
    warn!(
        "💎 sell: total_qty: {:10.20?}, \t💎 total_cost: {:20?}, \t💎 avg_price: {:20?}",
        sell_total_qty, sell_total_cost, sell_avg_price
    );
    warn!(
        "💎 current: total_qty: {:10.20?}, \t💎 avg_price: {:20?}\n",
        current_total_qty, current_avg_price
    );
}
