use binance_async::{
    account::*,
    api::*,
    config::Config,
    errors::Error as BinanceLibError,
    general::*,
    market::*,
    rest_model::{UniversalTransferType, *},
    wallet::*,
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
                        "π° deposit history: [{:?}, {:?}], length={}",
                        r.start_at,
                        r.end_at,
                        r.records.len()
                    );
                    for item in r.records {
                        info!("π deposit: {:?}", item);
                    }
                }
            },
            Err(e) => error!("Error: {:?}", e),
        }
    }

    // ζ₯θ―’εεΌεε²:
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
            "π° one query range: [{:?}, {:?} ], stop_at: {},",
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

                    info!("π° deposit history [{}, {}]:", start, end);

                    for deposit in answer {
                        info!("π user deposit records: {:?}", deposit);
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
                        "π° withdraw history: [{:?}, {:?}], length={}",
                        r.start_at,
                        r.end_at,
                        r.records.len()
                    );
                    for item in r.records {
                        info!("π withdraw: {:?}", item);
                    }
                }
            },
            Err(e) => error!("Error: {:?}", e),
        }
    }

    // δΈδΌ  coin ε°±ζ―ζζεΈη§:
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
            "π° one query range: [{:?}, {:?} ], stop_at: {},",
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
                        debug!("π° no withdraw history in range: [{:?}, {:?} ]", start, end);
                    } else {
                        info!("π° withdraw history in range: [{:?}, {:?} ]", start, end);
                        for withdraw in answer {
                            info!("π withdraw record: {:?}", withdraw);
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
                    info!("π° snapshot: {:?}", item.update_time);
                    match AccountSnapshotType::from_str(item.snapshot_type.as_str()) {
                        AccountSnapshotType::Spot => {
                            for balance in item.data.balances.iter() {
                                if balance.free + balance.locked >= 0.0 {
                                    info!("\tπ° coin: {:?}", balance);
                                }
                            }
                        },
                        AccountSnapshotType::Margin => {
                            info!("π° margin snapshot: {:?}", item);
                        },
                        AccountSnapshotType::Futures => {
                            info!("π° futures snapshot: {:?}", item);
                        },
                    }
                }
            },

            Err(e) => error!("Error: {:?}", e),
        }
    }

    // api key ζιζ£ζ₯:
    pub async fn api_key_permissions(&self) {
        match self.client.api_key_permissions().await {
            Ok(answer) => {
                info!("π° api_key_permissions: {:?}", answer);
            },
            Err(e) => error!("Error: {:?}", e),
        }
    }

    pub async fn system_status(&self) {
        match self.client.system_status().await {
            Ok(answer) => {
                info!("π° system_status: {:?}", answer);
            },
            Err(e) => error!("Error: {:?}", e),
        }
    }

    pub async fn all_coin_info(&self) {
        match self.client.all_coin_info().await {
            Ok(answer) => {
                info!("π° all_coin_info: {:?}", answer);
            },
            Err(e) => error!("Error: {:?}", e),
        }
    }

    // todo x: need check // Unauthorized
    pub async fn disable_fast_withdraw_switch(&self) {
        match self.client.disable_fast_withdraw_switch().await {
            Ok(answer) => {
                info!("π° disable_fast_withdraw_switch: {:?}", answer);
            },
            Err(e) => error!("Error: {:?}", e),
        }
    }

    // todo x: need check // Unauthorized
    pub async fn enable_fast_withdraw_switch(&self) {
        match self.client.enable_fast_withdraw_switch().await {
            Ok(answer) => {
                info!("π° enable_fast_withdraw_switch: {:?}", answer);
            },
            Err(e) => error!("Error: {:?}", e),
        }
    }

    // todo x: Unauthorized
    pub async fn withdraw(&self) {
        let req = CoinWithdrawalQuery::default();

        match self.client.withdraw(req).await {
            Ok(answer) => {
                info!("π° withdraw: {:?}", answer);
            },
            Err(e) => error!("Error: {:?}", e),
        }
    }

    // ζ₯θ―’εεΌε°ε:
    pub async fn deposit_addresses(&self, coins: &Vec<&str>) {
        for coin in coins.iter() {
            let req = DepositAddressQuery { coin: coin.to_string(), network: None };

            match self.client.deposit_address(req).await {
                Ok(answer) => {
                    info!("π° deposit address: {:?}", answer);
                },
                Err(e) => error!("Error: {:?}", e),
            }
        }
    }

    pub async fn account_status(&self) {
        match self.client.account_status().await {
            Ok(answer) => {
                info!("π° account_status: {:?}", answer);
            },
            Err(e) => error!("Error: {:?}", e),
        }
    }

    pub async fn api_trading_status(&self) {
        match self.client.api_trading_status().await {
            Ok(answer) => {
                info!("π° api_trading_status: {:?}", answer);
            },
            Err(e) => error!("Error: {:?}", e),
        }
    }

    pub async fn dust_log(&self, start_at: Option<u64>, end_at: Option<u64>) {
        match self.client.dust_log(start_at, start_at).await {
            Ok(answer) => {
                info!("π° dust_log: {:?}", answer);
            },
            Err(e) => error!("Error: {:?}", e),
        }
    }

    pub async fn convertible_assets(&self) {
        match self.client.convertible_assets().await {
            Ok(answer) => {
                info!("π° convertible_assets: {:?}", answer);
            },
            Err(e) => error!("Error: {:?}", e),
        }
    }

    // todo x:  need check, api key permission // Error: Unauthorized
    pub async fn dust_transfer(&self, assets: Vec<String>) {
        match self.client.dust_transfer(assets).await {
            Ok(answer) => {
                info!("π° dust_transfer: {:?}", answer);
            },
            Err(e) => error!("Error: {:?}", e),
        }
    }

    pub async fn asset_dividends(&self, query: AssetDividendQuery) {
        match self.client.asset_dividends(query).await {
            Ok(answer) => {
                info!("π° asset_dividends: {:?}", answer);
            },
            Err(e) => error!("Error: {:?}", e),
        }
    }

    pub async fn asset_detail(&self, asset: Option<String>) {
        match self.client.asset_detail(asset).await {
            Ok(answer) => {
                info!("π° asset_detail: {:?}", answer);
            },
            Err(e) => error!("Error: {:?}", e),
        }
    }

    pub async fn trade_fees(&self, symbol: Option<String>) {
        match self.client.trade_fees(symbol).await {
            Ok(answer) => {
                info!("π° trade_fees: {:?}", answer);
            },
            Err(e) => error!("Error: {:?}", e),
        }
    }

    // todo x: Error: Unauthorized
    pub async fn universal_transfer(
        &self,
        asset: String,
        amount: f64,
        from_symbol: Option<String>,
        to_symbol: Option<String>,
        transfer_type: UniversalTransferType,
    ) {
        match self
            .client
            .universal_transfer(asset, amount, from_symbol, to_symbol, transfer_type)
            .await
        {
            Ok(answer) => {
                info!("π° universal_transfer: {:?}", answer);
            },
            Err(e) => error!("Error: {:?}", e),
        }
    }

    pub async fn universal_transfer_history(&self, query: UniversalTransferHistoryQuery) {
        match self.client.universal_transfer_history(query).await {
            Ok(answer) => {
                info!("π° universal_transfer_history: {:?}", answer);
            },
            Err(e) => error!("Error: {:?}", e),
        }
    }

    pub async fn funding_wallet(&self, asset: Option<String>, need_btc_valuation: Option<bool>) {
        match self.client.funding_wallet(asset, need_btc_valuation).await {
            Ok(answer) => {
                info!("π° funding_wallet: {:?}", answer);
            },
            Err(e) => error!("Error: {:?}", e),
        }
    }
}

// auth:
pub async fn do_wallet_cmd(api_key: &str, secret_key: &str) {
    let cli = WalletService::new(Some(api_key.into()), Some(secret_key.into()));

    // ζ―ζζ₯ζζεΈη§:
    // cli.withdraw_history_quick(None, Some(5), None, None).await;
    // cli.deposit_history_quick(None, Some(5), None, None).await;

    // cli.deposit_history(None).await;
    // cli.withdraw_history(None).await;

    // εΈε?ηεεΌε°ε:
    let coins = &vec!["USDT", "BUSD", "BTC", "ETH", "BNB", "DOT"];

    // εΈε?ηθ΄¦ζ·εΏ«η§:
    // cli.snapshot().await;
    // cli.api_key_permissions().await;
    // cli.system_status().await;
    // cli.all_coin_info().await;
    // cli.disable_fast_withdraw_switch().await; // todo x: need check
    // cli.enable_fast_withdraw_switch().await; // todo x: need check
    // cli.withdraw().await; // todo x: need check
    // cli.deposit_addresses(coins).await;

    // cli.account_status().await;
    // cli.api_trading_status().await; // fix
    // cli.dust_log(None, None).await;
    // cli.convertible_assets().await; // fix

    // cli.dust_transfer(vec!["LUNC".to_string()]).await; // todo x:  Error: Unauthorized

    cli.asset_dividends(AssetDividendQuery::default()).await;
    cli.trade_fees(None).await;

    // cli.universal_transfer(
    //     "LUNC".to_string(),
    //     0.0,
    //     None,
    //     None,
    //     UniversalTransferType::MainUmfuture,
    // )
    // .await; // todo x: Error: Unauthorized

    let query: UniversalTransferHistoryQuery = UniversalTransferHistoryQuery {
        start_time: None,
        end_time: None,
        transfer_type: UniversalTransferType::FundingMain,
        current: None,
        from_symbol: None,
        to_symbol: None,
        size: None,
    };

    // cli.universal_transfer_history(query).await; // fix
    // cli.funding_wallet(None, None).await; // fix
    // cli.asset_detail(None).await;

    warn!("do wallet cmd done.")
}
