use nostr_sdk::prelude::*;
use utils::{fetch_follows_of_pubkey, save_pubkeys_in_file};

mod client;
mod config;
mod utils;
mod wot;

use config::{FIATMAXI_NSEC, FIVE_HEXPUBKEY, SATSHOOT_HEXPUBKEY};
use client::{initialize_client_singleton, get_client, ClientBuildOption};
use wot::{update_wot, WOT};

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

    update_wot(&satshoot_pubkey).await?;

    let wot_vec: Vec<PublicKey> = WOT.iter().map(|pk| *pk).collect();

    save_pubkeys_in_file(&wot_vec, "satshoot_wot").await?;

    Ok(())
}


