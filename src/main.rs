use nostr_sdk::prelude::*;

mod client;
mod config;
mod utils;
mod wot;

use crate::client::{get_client, initialize_client_singleton, ClientBuildOption};
use crate::config::SATSHOOT_HEXPUBKEY;
use crate::utils::{read_pubkeys_from_file, save_pubkeys_in_file};
use crate::wot::{update_wot, WOT};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    initialize_client_singleton(ClientBuildOption::NoNsec);

    let satshoot_pubkey = PublicKey::from_hex(SATSHOOT_HEXPUBKEY).unwrap();

    for &relay_url in &config::BOOTSTRAP_RELAYS {
        get_client().add_relay(relay_url.to_string()).await?;
    }

    get_client().connect().await;

    let now = Instant::now();
    update_wot(&satshoot_pubkey).await?;
    println!("WOT updated in {:.2} secs", now.elapsed().as_secs_f32());

    // println!("Reading WoT from file...");
    //
    // match read_pubkeys_from_file("satshoot_wot").await {
    //     Ok(wot_vec) => {
    //         for public_key in wot_vec {
    //             WOT.insert(public_key);
    //         }
    //     },
    //
    //     Err(e) => eprintln!("Error: {:?}", e)
    //
    // }

    println!("WOT size: {}", WOT.len());

    Ok(())
}
