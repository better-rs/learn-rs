#![allow(dead_code, unused_imports)] // global ignore useless warnings

use clap::Parser;
use log::{info, warn};
use pretty_env_logger;

use rs_scripts::{
    commands::binance::{BinanceCli, BinanceCommands, WalletCommand},
    modules::binance::{account, binance_async, market::do_market_cmd, wallet},
};

#[allow(unused_doc_comments)]
#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let args = BinanceCli::parse();

    match &args.command {
        // market data:
        BinanceCommands::General { empty } => {
            binance_async::general().await;
        },
        // user account data:
        BinanceCommands::Auth { api_key, api_secret } => {
            println!("binance api key: {}\n\n", api_key);
            // binance_async::account_data(Some(api_key.into()), Some(api_secret.into())).await;
            account::do_account_cmd(api_key, api_secret).await;
        },

        // wallet:
        BinanceCommands::Wallet(cmd) => match cmd {
            // 充值:
            WalletCommand::Deposit { api_key, api_secret, coin, status, start_time, .. } => {
                info!("deposit records: {}", api_key);
                wallet::do_wallet_cmd(api_key, api_secret).await;
            },

            // 提现:
            WalletCommand::Withdraw { api_key, api_secret, coin, status, start_time, .. } => {
                info!("withdraw records:");
            },

            _ => {
                warn!("unknown wallet command");
            },
        },

        // market data:
        BinanceCommands::Market { empty: _ } => {
            // binance_async::market_data().await;
            do_market_cmd().await;
        },

        _ => {
            warn!("not matched");
        },
    }

    info!("cli finished");
}
