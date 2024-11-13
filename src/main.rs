use nostr_sdk::prelude::*;
use utils::save_kind1_events_in_file;

use std::time::Duration;

mod client;
mod config;
mod utils;
mod wot;

use crate::client::{get_client, initialize_client_singleton, ClientBuildOption};
use crate::config::SATSHOOT_HEXPUBKEY;
use crate::utils::{read_pubkeys_from_file, save_pubkeys_in_file, SavingMethod};
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

    println!("fetching kind1s of WoT...");

    let filter = Filter::new()
        .authors(WOT.iter().map(|p| p.clone()))
        .kind(Kind::TextNote)
        .since(Timestamp::now() - Duration::from_secs(30 * 24 * 60 * 60));

    let kind1_events = get_client()
        .fetch_events(vec![filter], Some(Duration::from_secs(15)))
        .await?;

    println!("Fetching kind1s Completed! Saving in file...");

    save_kind1_events_in_file(
        kind1_events,
        "satshoot_wot_all_kind1s_content_only.txt",
        SavingMethod::ContentOnly
    ).await?;




    Ok(())
}
