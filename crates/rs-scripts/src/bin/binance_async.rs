use clap::Parser;
use log::{info, warn};
use pretty_env_logger;

use crate::{
    commands::binance::{BinanceCli, BinanceCommands},
    modules::binance_async,
};

mod commands;
mod modules;

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
            binance_async::account_data(Some(api_key.into()), Some(api_secret.into())).await;
        },

        // market data:
        BinanceCommands::Market { empty: _ } => {
            binance_async::market_data().await;
        },

        _ => {
            warn!("not matched");
        },
    }

    info!("cli finished");
}
