use clap::{Parser, Subcommand};

/// A fictional versioning CLI
#[derive(Parser)]
#[clap(name = "binance", about = "A binance CLI", version = "0.1.0", long_about = "A binance CLI")]
pub struct BinanceCli {
    #[clap(subcommand)]
    pub command: BinanceCommands,
}

#[derive(Subcommand)]
pub enum BinanceCommands {
    /// get server status
    #[clap(arg_required_else_help = true)]
    General {
        /// allow empty
        empty: String,
    },

    /// auth: get user account data
    #[clap(arg_required_else_help = true)]
    Auth {
        /// api key
        api_key: String,

        /// api secret
        api_secret: String,
    },

    /// market: get market data
    #[clap(arg_required_else_help = true)]
    Market {
        /// allow empty
        empty: String,
    },
}
