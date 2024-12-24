
use anyhow::{Ok, Result};

mod utils;
mod eth_wallet;

#[tokio::main]
async fn main() -> Result<()> {

    let (secret_key, pub_key) = eth_wallet::generate_keypair();

    println!("secret key: {}", &secret_key.to_string());
    println!("public key: {}", &pub_key.to_string());

    let pub_address = eth_wallet::public_key_address(&pub_key);
    println!("public address: {:?}", pub_address);


    let crypto_wallet = eth_wallet::Wallet::new(&secret_key, &pub_key);
    println!("crypto wallet: {:?}", &crypto_wallet);


    let wallet_file_path = "crypto_wallet.json";
    crypto_wallet.save_to_file(&wallet_file_path)?;

    let loaded_wallet = eth_wallet::Wallet::from_file(wallet_file_path)?;
    println!("loaded_wallet: {:?}", loaded_wallet);


    let endpoint = env::var("INFURA_RINKEBY_WS")?;
    let web3_con = eth_wallet::establish_web3_connection(&endpoint).await?;
    



    crypto_wallet.save_to_file("crypto_wallet.json")?;

    Ok(())
}   