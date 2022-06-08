use clap::Parser;
use log::info;
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
        BinanceCommands::General { empty: _ } => {
            binance_async::general().await;
        },
        // user account data:
        BinanceCommands::Auth { api_key, api_secret } => {
            println!("binance api key: {}\n\n", api_key);
        },

        // market data:
        BinanceCommands::Market { empty: _ } => {},
    }

    info!("cli finished");
}
