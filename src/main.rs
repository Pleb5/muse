use nostr_sdk::prelude::*;
use utils::{ read_pubkeys_from_file };

mod client;
mod config;
mod utils;
mod wot;

use config::{ SATSHOOT_HEXPUBKEY};
use client::{initialize_client_singleton, get_client, ClientBuildOption};
use wot::{update_wot, WOT};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    initialize_client_singleton(ClientBuildOption::NoNsec);

    let satshoot_pubkey = PublicKey::from_hex(SATSHOOT_HEXPUBKEY).unwrap();

    for &relay_url in &config::BOOTSTRAP_RELAYS {
        get_client().add_relay(relay_url.to_string()).await?;
    }

    get_client().connect().await;

    println!("Reading WoT from file...");

    match read_pubkeys_from_file("satshoot_wot").await {
        Ok(wot_vec) => {
            for public_key in wot_vec {
                WOT.insert(public_key);
            }
        },

        Err(e) => eprintln!("Error: {:?}", e)
        
    }

    println!("WOT size: {}", WOT.len());

    Ok(())
}


