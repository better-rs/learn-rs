use clap::{Parser, Subcommand};

/// A fictional versioning CLI
#[derive(Parser)]
#[clap(
    name = "keytool",
    about = "A Cli Crypto Wallet Tool",
    version = "0.1.0",
    long_about = None,
)]
pub struct KeyToolCli {
    #[clap(subcommand)]
    pub command: KeyToolCommand,
}

#[derive(Subcommand)]
pub enum KeyToolCommand {
    /*
    todo x:
        1. 生成 HD 钱包, 助记词/私钥/地址
        2. 多链支持: BTC/ETH/DOT
        3. 命令行查询
            - 钱包余额
            - 钱包交易记录
            - 单笔交易状态
        4. 命令行转账:
            - 发起交易
            - 交易状态查询
    */
    /// Generate a new crypto wallet account: mnemonic/private key/address
    #[clap(arg_required_else_help = true)]
    Generate {
        /// allow emtpy
        empty: String,
    },

    /// Query blockchain info
    #[clap(arg_required_else_help = true)]
    #[clap(subcommand)]
    Query(QueryCommand),

    /// Send a transaction
    #[clap(arg_required_else_help = true)]
    Send {
        /// chain name
        chain_type: String,

        /// from address
        from: String,

        /// to address
        to: String,

        /// amount
        amount: String,

        /// miner fee
        fee: String,
    },
}

#[derive(Subcommand)]
pub enum QueryCommand {
    /// Query balance
    #[clap(arg_required_else_help = true)]
    Balance {
        /// blockchain type
        chain_type: String,

        /// blockchain id:
        chain_id: String,

        /// address
        address: String,
    },

    /// Query transaction
    Transaction {
        /// blockchain type
        chain_type: String,

        /// blockchain id:
        chain_id: String,

        /// transaction id
        tx_id: String,
    },
}
