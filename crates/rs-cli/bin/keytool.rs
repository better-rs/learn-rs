use clap::Parser;
use log::info;
use pretty_env_logger;

use rs_cli::commands::keytool::{KeyToolCli, KeyToolCommand, QueryCommand};

fn main() {
    pretty_env_logger::init();

    let args = KeyToolCli::parse();

    match &args.command {
        KeyToolCommand::Generate { empty: _ } => {
            info!("GenerateAccount command");
        },

        // subcommands:
        KeyToolCommand::Query(cmd) => {
            info!("GenerateAccount command");
            match cmd {
                QueryCommand::Balance { chain_type, chain_id, address } => {
                    info!("Balance command: {}, {}, {}", chain_type, chain_id, address);
                },
                QueryCommand::Transaction { chain_type, chain_id, tx_id } => {
                    info!("Balance command: {}, {}, {}", chain_type, chain_id, tx_id);
                },
            }
        },
        KeyToolCommand::Send { chain_type, from, to, amount, fee } => {
            info!("GenerateAccount command: {}, {}, {}, {}, {}", chain_type, from, to, amount, fee);
        },
    }

    info!("cli finished");
}
