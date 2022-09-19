use clap::Parser;
use log::{info, warn};
use pretty_env_logger;

use rs_scripts::{
    commands::binance::{BinanceCli, BinanceCommands},
    modules::binance::binance::{account_data, market_data},
};

fn main() {
    pretty_env_logger::init();

    let args = BinanceCli::parse();

    match &args.command {
        // user account data:
        BinanceCommands::Auth { api_key, api_secret } => {
            println!("binance api key: {}\n\n", api_key);

            account_data(Some(api_key.into()), Some(api_secret.into()));
        },

        // market data:
        BinanceCommands::Market { empty: _ } => {
            market_data();
        },

        _ => {
            warn!("not matched");
        },
    }

    info!("cli finished");
}
