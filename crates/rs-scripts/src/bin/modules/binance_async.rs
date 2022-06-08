use binance_async::{
    account::*,
    api::*,
    config::Config,
    errors::Error as BinanceLibError,
    general::*,
    market::*,
    rest_model::{
        AccountSnapshotQuery, AccountSnapshotType, DepositHistoryQuery, OrderSide, OrderType,
        SymbolPrice, TimeInForce, WithdrawalHistoryQuery,
    },
    wallet::*,
};

use binance_async::wallet::*;

use chrono::{TimeZone, Utc};
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

// auth:
pub async fn account_data(api_key: Option<String>, secret_key: Option<String>) {
    let account = Binance::new(api_key, secret_key);

    // 多个币种计算:
    let coins = vec!["BTC", "ETH", "BNB", "SOL", "FIL", "DOT"];

    // 打印账号余额:
    get_all_balance(&account).await;

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
    calc_avg_cost_by_coins(&account, &coins).await; // todo x: fix 参数类型, 改为 引用类型, 否则会出现 move 错误, rust 的生命周期

    // 查询币种余额:
    get_balance_by_coins(&account, &coins).await;
}

// 打印用户余额:
async fn get_all_balance(account: &Account) {
    match account.get_account().await {
        Ok(answer) => {
            for balance in answer.balances {
                // let free: f64 = balance.free.parse().unwrap();
                // let locked: f64 = balance.locked.parse().unwrap();
                let free = balance.free;
                let locked = balance.locked;

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
async fn get_balance_by_coins(account: &Account, coins: &Vec<&str>) {
    // todo x: fix vec.iter(), not vec itself
    for coin in coins.iter() {
        match account.get_balance(coin.to_string()).await {
            Ok(answer) => {
                info!("🍄 {}: {:?}", coin, answer);
            },
            Err(e) => error!("Error: {:?}", e),
        }
    }
}

async fn calc_avg_cost_by_coins(account: &Account, coins: &Vec<&str>) {
    for coin in coins.iter() {
        let cli = account.clone();
        calc_avg_cost_by_coin(cli, coin).await;
    }
}

// 单个币种, 多个交易对计算:
async fn calc_avg_cost_by_coin(account: Account, coin: &str) {
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
        match account.trade_history(&coin_pair).await {
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

// auth:
pub async fn wallet_data(api_key: Option<String>, secret_key: Option<String>) {
    let wallet: Wallet = Binance::new(api_key, secret_key);

    let now_at = Utc::now().timestamp_millis();
    let ts_90days_ago: i64 = Utc::now().timestamp_millis() - (60 * 60 * 24 * 90);
    info!("💰 start time: {:?}", now_at);
    info!("💰 ts_90days_ago: {:?}", ts_90days_ago);

    let deposit_req = DepositHistoryQuery {
        coin: Some("USDT".to_string()),
        status: None,
        start_time: Some(ts_90days_ago as u64),
        end_time: Some(now_at as u64),
        limit: None,
        offset: None,
    };

    info!("💰 wallet deposit: req={:?}", deposit_req);
    match wallet.deposit_history(deposit_req).await {
        Ok(answer) => {
            info!("💰 deposit history: {:?}", answer);

            for deposit in answer {
                info!("💰 user deposit records: {:?}", deposit);
            }
        },
        Err(e) => error!("Error: {:?}", e),
    }

    let withdraw_req = WithdrawalHistoryQuery {
        coin: Some("USDT".to_string()),
        withdraw_order_id: None,
        status: None,
        start_time: Some(ts_90days_ago as u64),
        end_time: Some(now_at as u64),
        limit: None,
        offset: None,
    };

    info!("💰 withdrawal history: req={:?}", withdraw_req);

    match wallet.withdraw_history(withdraw_req).await {
        Ok(answer) => {
            info!("💰 withdraw history: {:?}", answer);

            for withdraw in answer {
                info!("💰 user withdraw records: {:?}", withdraw);
            }
        },
        Err(e) => error!("Error: {:?}", e),
    }

    let snapshot_req: AccountSnapshotQuery = AccountSnapshotQuery {
        start_time: None,
        end_time: None,
        limit: None,
        account_type: AccountSnapshotType::Spot,
    };

    match wallet.daily_account_snapshot(snapshot_req).await {
        Ok(answer) => {
            info!("💰 daily account snapshot: {:?}", answer);
        },

        Err(e) => error!("Error: {:?}", e),
    }

    match wallet.all_coin_info().await {
        Ok(answer) => {
            info!("💰 all coin info: {:?}", answer);
        },
        Err(e) => error!("Error: {:?}", e),
    }

    match wallet.funding_wallet(Some("USDT".into()), None).await {
        Ok(answer) => {
            info!("💰 funding rate: {:?}", answer);
        },
        Err(e) => error!("Error: {:?}", e),
    }

    match wallet.account_status().await {
        Ok(answer) => {
            info!("💰 account status: {:?}", answer);
        },
        Err(e) => error!("Error: {:?}", e),
    }
}
