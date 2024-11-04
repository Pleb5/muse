use nostr_sdk::prelude::*;

use crate::client::get_client;

use std::time::Duration;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;


pub async fn get_kind1_events_of_user(
    user_pubkey: &PublicKey
) -> Result<Vec<Event>, Error> {
    let filter = Filter::new()
        .authors([PublicKey::from_hex(user_pubkey.to_hex()).unwrap()])
        .kind(Kind::TextNote);

    let events = get_client()
        .get_events_of(
            vec![filter],
            EventSource::both(Some(Duration::from_secs(6))),
        )
        .await;
    match events {
        Ok(kind1_events) => {
            for kind1_event in &kind1_events {
                println!("fetched kind1_events: {}", kind1_event.as_json());
            }
            Ok(kind1_events)
        },

        Err(e) => {
            Err(e)
        }
    }
}

pub async fn save_kind1_events_in_file(
    events: &Vec<Event>,
    file_name: &str
) -> Result<()> {
    let mut file = File::create(file_name).await?;

    for (index, event) in events.into_iter().enumerate() {
        file.write_all(
            format!("{}.\n{}\n", index, event.as_json())
            .as_bytes()).await?;
    }
    Ok(())
}

pub async fn fetch_follows_of_pubkey(
    pubkey: &PublicKey
) -> Result<Vec<PublicKey>, Error> {

    let filter: Filter = Filter::new()
        .authors(vec![pubkey.clone()])
        .kind(Kind::ContactList)
        .limit(1);

    let fetch_timeout = Duration::from_secs(5);
    let mut followed_pubkeys: Vec<PublicKey> = Vec::new();

    match get_client()
        .get_events_of(vec![filter], EventSource::relays(Some(fetch_timeout)))
        .await
    {
        Ok(contact_list_event) => {
            if let Some(contact_list_event) = contact_list_event.into_iter().next() {
                for tag in contact_list_event.tags.into_iter() {
                    if let Some(TagStandard::PublicKey {
                        public_key,
                        uppercase:false,
                        .. 
                    }) = tag.to_standardized()
                    {
                        followed_pubkeys.push(public_key);
                    }
                }
            }
            Ok(followed_pubkeys)
        }
        Err(e) => Err(e)
    }
}

pub async fn save_follows(
    follows: &Vec<PublicKey>,
    file_name: &str
) -> Result<()> {

    let mut file = File::create(file_name).await?;

    for (index, pubkey) in follows.into_iter().enumerate() {
        file.write_all(
            format!("{}.\n{}\n", index, pubkey)
            .as_bytes()
        ).await?;
    }
    Ok(())
}
