use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// A fictional versioning CLI
#[derive(Parser)]
#[clap(name = "eth")]
#[clap(about = "A ETH Cli Tool", long_about = None)]
pub(crate) struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    /// Scan ETH Address
    #[clap(arg_required_else_help = true)]
    Scan {
        /// eth rpc url:
        rpc_url: String,

        /// address
        address: String,

        /// transaction type
        tx_type: String,

        /// count of address
        count: String,
    },

    /// adds things
    #[clap(arg_required_else_help = true)]
    Add {
        /// Stuff to add
        #[clap(required = true, parse(from_os_str))]
        path: Vec<PathBuf>,
    },
}
