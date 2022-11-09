use clap::Parser;

use rs_cli::{
    commands::eth::{EthCli, EthCommands},
    core::eth,
};

#[allow(unused_doc_comments)]
#[tokio::main]
async fn main() -> web3::Result<()> {
    /*
    cli 传参示例:
        cargo run --bin rs-eth-scanner -- scan "http://abc.url" "0xxxxx" "deposit" "100"
    */

    let args = EthCli::parse();

    match &args.command {
        /// scan eth address:
        EthCommands::Scan { rpc_url, address, tx_type, count } => {
            println!("scan eth address: {}", address);

            eth::eth_scan_address(
                rpc_url.as_str(),
                address.as_str(),
                tx_type.as_str(),
                count.as_str(),
            )
            .await?;
        },
        EthCommands::Add { path } => {
            println!("Adding {:?}", path);
        },
    }

    return Ok(())
}
