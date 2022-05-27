use clap::{Parser, Subcommand};

/// A fictional versioning CLI
#[derive(Parser)]
#[clap(
    name = "binance",
    about = "A binance CLI",
    version = "0.1.0",
    long_about = "A binance CLI"
)]
pub struct BinanceCli {
    #[clap(subcommand)]
    pub command: BinanceCommands,
}

#[derive(Subcommand)]
pub enum BinanceCommands {
    /// auth:
    #[clap(arg_required_else_help = true)]
    Auth {
        /// api key
        api_key: String,

        /// api secret
        api_secret: String,
    },
}