use binance_async::{
    account::*, api::*, config::Config, errors::Error as BinanceLibError, general::*, market::*,
    rest_model::*, wallet::*,
};

use binance_async::wallet::*;

use chrono::{Duration, TimeZone, Utc};
use log::{debug, error, info, warn};
#[allow(unused_imports)]
use pretty_env_logger;

// account:
pub struct AccountService {
    client: Account,
}

impl AccountService {
    pub fn new(api_key: Option<String>, secret_key: Option<String>) -> Self {
        Self { client: Binance::new(api_key, secret_key) }
    }

    // 打印用户余额:
    pub async fn get_all_balance(&self) {
        match self.client.get_account().await {
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
    pub async fn get_balance_by_coins(&self, coins: &Vec<&str>) {
        // todo x: fix vec.iter(), not vec itself
        for coin in coins.iter() {
            match self.client.get_balance(coin.to_string()).await {
                Ok(answer) => {
                    info!("🍄 {}: {:?}", coin, answer);
                },
                Err(e) => error!("Error: {:?}", e),
            }
        }
    }

    pub async fn calc_avg_cost_by_coins(&self, coins: &Vec<&str>) {
        for coin in coins.iter() {
            self.calc_avg_cost_by_coin(coin).await;
        }
    }

    // 单个币种, 多个交易对计算:
    pub async fn calc_avg_cost_by_coin(&self, coin: &str) {
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
            match self.client.trade_history(&coin_pair).await {
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
}

// auth:
pub async fn do_account_cmd(api_key: &str, secret_key: &str) {
    let cli = AccountService::new(Some(api_key.into()), Some(secret_key.into()));

    // 多个币种计算:
    let coins = vec!["BTC", "ETH", "BNB", "SOL", "FIL", "DOT"];

    // 打印账号余额:
    cli.get_all_balance().await;

    // 批量计算多个币种:
    cli.calc_avg_cost_by_coins(&coins).await; // todo x: fix 参数类型, 改为 引用类型, 否则会出现 move 错误, rust 的生命周期

    // 查询币种余额:
    cli.get_balance_by_coins(&coins).await;

    warn!("do account cmd done.")

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
}
