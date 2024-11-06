use nostr_sdk::prelude::*;
use utils::{fetch_follows_of_pubkey, save_follows};

mod client;
mod config;
mod utils;
mod wot;

use config::{FIATMAXI_NSEC, FIVE_HEXPUBKEY, SATSHOOT_HEXPUBKEY};
use client::{initialize_client_singleton, get_client, ClientBuildOption};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    // let secret_key = SecretKey::from_bech32(FIATMAXI_NSEC)?;

    initialize_client_singleton(ClientBuildOption::NoNsec);

    let five_pubkey = PublicKey::from_hex(FIVE_HEXPUBKEY).unwrap();
    let satshoot_pubkey = PublicKey::from_hex(SATSHOOT_HEXPUBKEY).unwrap();

    for &relay_url in &config::BOOTSTRAP_RELAYS {
        get_client().add_relay(relay_url.to_string()).await?;
    }

    get_client().connect().await;

    match fetch_follows_of_pubkey(&satshoot_pubkey).await {
        Ok(follows) => {
            let file_name = "satshoot_follows.txt";
            println!(">>>>>>> Saving follows ... in File: {} <<<<<<<< ", file_name);

            save_follows(&follows, file_name).await?;
        },
        Err(e) => eprintln!("Could not fetch follows of five: {}",e.to_string())
    }

    Ok(())
}


