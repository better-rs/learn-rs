use crate::commands::eth::{Cli, Commands};
use clap::Parser;

mod commands;

#[allow(unused_doc_comments)]
#[tokio::main]
async fn main() -> web3::Result<()> {
    /*
    cli 传参示例:
        cargo run --bin rs-eth-scanner -- scan "http://abc.url" "0xxxxx" "deposit" "100"
    */

    let args = Cli::parse();

    match &args.command {
        /// scan eth address:
        Commands::Scan {
            rpc_url,
            address,
            tx_type,
            count,
        } => {
            println!("scan eth address: {}", address);

            eth_scan_address(
                rpc_url.as_str(),
                address.as_str(),
                tx_type.as_str(),
                count.as_str(),
            )
            .await?;
        }
        Commands::Add { path } => {
            println!("Adding {:?}", path);
        }
    }

    return Ok(());
}

#[allow(unused_variables)]
async fn eth_scan_address(
    rpc_url: &str,
    address: &str,
    tx_type: &str,
    count: &str,
) -> web3::Result<()> {
    let transport = web3::transports::Http::new(rpc_url)?;
    let web3 = web3::Web3::new(transport);

    let mut accounts = web3.eth().accounts().await?;
    accounts.push(address.parse().unwrap());
    println!("Accounts: {:?}", accounts);

    for account in accounts {
        let balance = web3.eth().balance(account, None).await?;
        let count = web3.eth().transaction_count(account, None).await?;

        println!(
            "account: {:?}, balance: {}, tx count: {}",
            account, balance, count
        );
    }

    Ok(())
}
