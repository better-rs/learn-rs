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
        // allow empty args
        empty: Option<String>,
    },

    /// User wallet info
    #[clap(arg_required_else_help = true)]
    #[clap(subcommand)]
    Wallet(WalletCommand),

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

#[derive(Subcommand)]
pub enum WalletCommand {
    /// User Deposit Records
    #[clap(arg_required_else_help = true)]
    Deposit {
        /// api key
        api_key: String,

        /// api secret
        api_secret: String,

        /// coin type
        coin: Option<String>,

        /// transaction status: 0:pending, 1:success, 6: credited but cannot withdraw
        status: Option<i32>,

        start_time: Option<i64>,
        end_time: Option<i64>,
        offset: Option<i64>,
        limit: Option<i64>,
        recv_window: Option<i64>,
        timestamp: Option<i64>,
    },

    /// User Withdraw Records
    Withdraw {
        /// api key
        api_key: String,

        /// api secret
        api_secret: String,

        /// coin type
        coin: Option<String>,

        withdraw_order_id: Option<String>,

        /// 0(0:已发送确认Email,1:已被用户取消 2:等待确认 3:被拒绝 4:处理中 5:提现交易失败 6
        /// 提现完成) transaction status: 0:pending, 1:success, 6: credited but cannot
        /// withdraw
        status: Option<i32>,

        start_time: Option<i64>,
        end_time: Option<i64>,
        offset: Option<i64>,
        limit: Option<i64>,
        recv_window: Option<i64>,
        timestamp: Option<i64>,
    },
}
