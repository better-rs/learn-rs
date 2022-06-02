use web3;

#[allow(dead_code)]
#[allow(unused_variables)]
pub async fn eth_scan_address(
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

        println!("account: {:?}, balance: {}, tx count: {}", account, balance, count);
    }

    Ok(())
}
