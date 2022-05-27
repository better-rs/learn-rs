use clap::Parser;

use crate::commands::binance::{BinanceCli, BinanceCommands};
use crate::modules::binance::{account_data, market_data};

mod commands;
mod modules;

fn main() {
    let args = BinanceCli::parse();

    match &args.command {
        // user account data:
        BinanceCommands::Auth {
            api_key,
            api_secret,
        } => {
            println!("binance api key: {}\n\n", api_key);

            account_data(Some(api_key.into()), Some(api_secret.into()));
        }

        // market data:
        BinanceCommands::Market { empty: _ } => {
            market_data();
        }
    }
}
