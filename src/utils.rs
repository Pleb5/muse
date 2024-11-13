use nostr_sdk::prelude::*;

use crate::client::get_client;

use std::time::Duration;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

pub async fn fetch_kind1_events_of_user(user_pubkey: &PublicKey) -> Result<Events> {
    let filter = Filter::new()
        .authors([PublicKey::from_hex(user_pubkey.to_hex()).unwrap()])
        .kind(Kind::TextNote);

    let events = get_client()
        .fetch_events(vec![filter], Some(Duration::from_secs(6)))
        .await?;
    Ok(events)
}

pub async fn save_kind1_events_in_file(
    events: Events,
    file_name: &str,
    method: SavingMethod
) -> Result<()> {
    let mut file = File::create(file_name).await?;

    for (index, event) in events.iter().enumerate() {
        file.write_all(format!("{}.\n{}\n", index, event.content).as_bytes())
            .await?;
    }
    Ok(())
}

pub enum SavingMethod {
    JSON,
    ContentOnly
}

pub async fn fetch_follows_of_pubkey(pubkey: &PublicKey) -> Result<Vec<PublicKey>> {
    let filter: Filter = Filter::new()
        .authors(vec![*pubkey])
        .kind(Kind::ContactList)
        .limit(1);

    let fetch_timeout = Duration::from_secs(5);
    let mut followed_pubkeys: Vec<PublicKey> = Vec::new();

    let contact_list_event = get_client()
        .fetch_events(vec![filter], Some(fetch_timeout))
        .await?;

    if let Some(contact_list_event) = contact_list_event.into_iter().next() {
        // Reserve capacity
        followed_pubkeys.reserve_exact(contact_list_event.tags.public_keys().count());

        // Extend
        followed_pubkeys.extend(contact_list_event.tags.public_keys());
    }

    Ok(followed_pubkeys)
}

pub async fn save_pubkeys_in_file(follows: &[PublicKey], file_name: &str) -> Result<()> {
    let mut file = File::create(file_name).await?;

    for pubkey in follows.iter() {
        file.write_all(format!("{}\n", pubkey).as_bytes()).await?;
    }

    Ok(())
}

pub async fn read_pubkeys_from_file(path: &str) -> Result<Vec<PublicKey>> {
    let mut pubkey_result: Vec<PublicKey> = Vec::new();

    let file = File::open(path).await?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await? {
        if let Ok(public_key) = PublicKey::from_hex(line) {
            pubkey_result.push(public_key);
        }
    }

    Ok(pubkey_result)
}
