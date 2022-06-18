use binance_async::{
    account::*, api::*, config::Config, errors::Error as BinanceLibError, general::*, market::*,
    rest_model::*, wallet::*,
};

use binance_async::wallet::*;

use chrono::{Duration, TimeZone, Utc};
use log::{debug, error, info, warn};
#[allow(unused_imports)]
use pretty_env_logger;

// wallet:
// api docs:
//      - https://binance-docs.github.io/apidocs/spot/cn/#system
//
pub struct WalletService {
    client: Wallet,
}

impl WalletService {
    pub fn new(api_key: Option<String>, secret_key: Option<String>) -> Self {
        Self { client: Binance::new(api_key, secret_key) }
    }

    pub async fn deposit_history(&self, coin: Option<&str>) {
        let req = DepositHistoryQuery {
            coin: coin.map(|c| c.to_string()),
            status: None,
            start_time: None,
            end_time: None,
            limit: None,
            offset: None,
        };

        match self.client.deposit_history_quick(req, None, None).await {
            Ok(answer) => {
                for r in answer {
                    info!(
                        "💰 deposit history: [{:?}, {:?}], length={}",
                        r.start_at,
                        r.end_at,
                        r.records.len()
                    );
                    for item in r.records {
                        info!("💎 deposit: {:?}", item);
                    }
                }
            },
            Err(e) => error!("Error: {:?}", e),
        }
    }

    // 查询充值历史:
    pub async fn deposit_history_quick(
        &self,
        coin: Option<&str>,
        years: Option<i64>,
        months: Option<i64>,
        begin_at: Option<i64>,
    ) {
        let now_at = Utc::now().timestamp_millis();

        // min range = 2years
        let query_range =
            Duration::days(365 * years.unwrap_or(2)) + Duration::days(12 * months.unwrap_or(0));

        let stop_at = now_at - query_range.num_milliseconds();

        // one query step:
        let query_90days = Duration::days(90).num_milliseconds();

        // one query range:
        let mut start_at = begin_at.unwrap_or(now_at) - query_90days;
        let mut end_at = begin_at.unwrap_or(now_at);

        info!(
            "💰 one query range: [{:?}, {:?} ], stop_at: {},",
            Utc.timestamp_millis(start_at).to_rfc3339(),
            Utc.timestamp_millis(end_at).to_rfc3339(),
            Utc.timestamp_millis(stop_at).to_rfc3339(),
        );

        while start_at > stop_at {
            let deposit_req = DepositHistoryQuery {
                coin: coin.map(|c| c.to_string()),
                status: None,
                start_time: Some(start_at as u64),
                end_time: Some(end_at as u64),
                limit: None,
                offset: None,
            };

            match self.client.deposit_history(&deposit_req).await {
                Ok(answer) => {
                    let start = Utc.timestamp_millis(start_at).to_rfc3339();
                    let end = Utc.timestamp_millis(end_at).to_rfc3339();

                    info!("💰 deposit history [{}, {}]:", start, end);

                    for deposit in answer {
                        info!("💎 user deposit records: {:?}", deposit);
                    }
                },
                Err(e) => error!("Error: {:?}", e),
            }

            // next query range:
            start_at -= query_90days;
            end_at -= query_90days;
        }
    }

    // new withdraw history api:
    pub async fn withdraw_history(&self, coin: Option<&str>) {
        let req = WithdrawalHistoryQuery {
            coin: coin.map(|c| c.to_string()),
            withdraw_order_id: None,
            status: None,
            start_time: None,
            end_time: None,
            limit: None,
            offset: None,
        };

        match self.client.withdraw_history_quick(req, None, None).await {
            Ok(answer) => {
                for r in answer {
                    info!(
                        "💰 withdraw history: [{:?}, {:?}], length={}",
                        r.start_at,
                        r.end_at,
                        r.records.len()
                    );
                    for item in r.records {
                        info!("💎 withdraw: {:?}", item);
                    }
                }
            },
            Err(e) => error!("Error: {:?}", e),
        }
    }

    // 不传 coin 就是所有币种:
    pub async fn withdraw_history_quick(
        &self,
        coin: Option<&str>,
        years: Option<i64>,
        months: Option<i64>,   // = 1-12,
        begin_at: Option<i64>, // timestamp
    ) {
        let now_at = Utc::now().timestamp_millis();

        // min range = 2years
        let query_range =
            Duration::days(365 * years.unwrap_or(2)) + Duration::days(12 * months.unwrap_or(0));

        let stop_at = now_at - query_range.num_milliseconds();

        // one query step:
        let query_90days = Duration::days(90).num_milliseconds();

        // one query range:
        let mut start_at = begin_at.unwrap_or(now_at) - query_90days;
        let mut end_at = begin_at.unwrap_or(now_at);

        info!(
            "💰 one query range: [{:?}, {:?} ], stop_at: {},",
            Utc.timestamp_millis(start_at).to_rfc3339(),
            Utc.timestamp_millis(end_at).to_rfc3339(),
            Utc.timestamp_millis(stop_at).to_rfc3339(),
        );

        // stop at 2years ago:
        while start_at > stop_at {
            let withdraw_req = WithdrawalHistoryQuery {
                coin: coin.map(|c| c.to_string()),
                withdraw_order_id: None,
                status: None,
                start_time: Some(start_at as u64),
                end_time: Some(end_at as u64),
                limit: None,
                offset: None,
            };

            match self.client.withdraw_history(&withdraw_req).await {
                Ok(answer) => {
                    let start = Utc.timestamp_millis(start_at).to_rfc3339();
                    let end = Utc.timestamp_millis(end_at).to_rfc3339();

                    if answer.is_empty() {
                        debug!("💰 no withdraw history in range: [{:?}, {:?} ]", start, end);
                    } else {
                        info!("💰 withdraw history in range: [{:?}, {:?} ]", start, end);
                        for withdraw in answer {
                            info!("💎 withdraw record: {:?}", withdraw);
                        }
                    }
                },
                Err(e) => error!("Error: {:?}", e),
            }

            // iter:
            start_at -= query_90days;
            end_at -= query_90days;
        }
    }

    // 查询充值地址:
    pub async fn deposit_addresses(&self, coins: &Vec<&str>) {
        for coin in coins.iter() {
            let req = DepositAddressQuery { coin: coin.to_string(), network: None };

            match self.client.deposit_address(req).await {
                Ok(answer) => {
                    info!("💰 deposit address: {:?}", answer);
                },
                Err(e) => error!("Error: {:?}", e),
            }
        }
    }

    // todo x:
    pub async fn snapshot(&self) {
        let snapshot_req: AccountSnapshotQuery = AccountSnapshotQuery {
            start_time: None,
            end_time: None,
            limit: None,
            account_type: AccountSnapshotType::Spot, // todo x: fix bugs here
        };

        match self.client.daily_account_snapshot(snapshot_req).await {
            Ok(answer) => {
                for item in answer.snapshot_vos.iter() {
                    info!("💰 snapshot: {:?}", item.update_time);
                    match AccountSnapshotType::from_str(item.snapshot_type.as_str()) {
                        AccountSnapshotType::Spot => {
                            for balance in item.data.balances.iter() {
                                if balance.free + balance.locked >= 0.0 {
                                    info!("\t💰 coin: {:?}", balance);
                                }
                            }
                        },
                        AccountSnapshotType::Margin => {
                            info!("💰 margin snapshot: {:?}", item);
                        },
                        AccountSnapshotType::Futures => {
                            info!("💰 futures snapshot: {:?}", item);
                        },
                    }
                }
            },

            Err(e) => error!("Error: {:?}", e),
        }
    }

    // api key 权限检查:
    pub async fn api_key_permissions(&self) {
        match self.client.api_key_permissions().await {
            Ok(answer) => {
                info!("💰 api_key_permissions: {:?}", answer);
            },
            Err(e) => error!("Error: {:?}", e),
        }
    }
}

// auth:
pub async fn do_wallet_cmd(api_key: &str, secret_key: &str) {
    let wallet: Wallet = Binance::new(Some(api_key.into()), Some(secret_key.into()));

    let cli = WalletService::new(Some(api_key.into()), Some(secret_key.into()));

    // 支持查所有币种:
    // cli.withdraw_history_quick(None, Some(5), None, None).await;
    // cli.deposit_history_quick(None, Some(5), None, None).await;

    cli.deposit_history(None).await;
    cli.withdraw_history(None).await;

    let now_at = Utc::now().timestamp_millis();
    let ts_90days_ago: i64 = Utc::now().timestamp_millis() - (60 * 60 * 24 * 90);

    // 币安的充值地址:
    let coins = &vec!["USDT", "BUSD", "BTC", "ETH", "BNB", "DOT"];
    // cli.deposit_addresses(coins).await;

    // 币安的账户快照:
    // cli.snapshot().await;

    cli.api_key_permissions().await;

    warn!("do wallet cmd done.")
}

// auth:
pub async fn wallet_api(api_key: Option<String>, secret_key: Option<String>) {
    let wallet: Wallet = Binance::new(api_key, secret_key);

    let now_at = Utc::now().timestamp_millis();
    let ts_90days_ago: i64 = Utc::now().timestamp_millis() - (60 * 60 * 24 * 90);
    info!("💰 start time: {:?}", now_at);
    info!("💰 ts_90days_ago: {:?}", ts_90days_ago);

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

    let address_req = DepositAddressQuery { coin: "USDT".into(), network: None };

    let deposit_addrs = &vec![
        DepositAddressQuery { coin: "USDT".into(), network: None },
        DepositAddressQuery { coin: "BUSD".into(), network: None },
        DepositAddressQuery { coin: "BTC".into(), network: None },
        DepositAddressQuery { coin: "ETH".into(), network: None },
        DepositAddressQuery { coin: "BNB".into(), network: None },
        DepositAddressQuery { coin: "DOT".into(), network: None },
    ];

    // 币安的充值地址:
    let coins = &vec!["USDT", "BUSD", "BTC", "ETH", "BNB", "DOT"];

    for coin in coins.iter() {
        let req = DepositAddressQuery { coin: coin.to_string(), network: None };

        match wallet.deposit_address(req).await {
            Ok(answer) => {
                info!("💰 deposit address: {:?}", answer);
            },
            Err(e) => error!("Error: {:?}", e),
        }
    }
}
