use nostr_sdk::prelude::*;
use std::time::Duration;

use tokio::fs::File;
use tokio::io::{self, AsyncWriteExt, AsyncReadExt};

mod config;


enum ClientBuildOption {
    WithNsec(SecretKey),
    NoNsec,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    // let secret_key = SecretKey::from_bech32(config::FIATMAXI_NSEC)?;

    let client = create_client(ClientBuildOption::NoNsec);
    let five_pubkey = PublicKey::from_hex(config::FIVE_HEXPUBKEY).unwrap();

    for &relay_url in &config::BOOTSTRAP_RELAYS {
        client.add_relay(relay_url.to_string()).await?;
    }

    client.connect().await;

    // Get kind1 events of five
    if let Ok(kind1_events_of_five) = get_kind1_events_of_user(&client, &five_pubkey).await {
        // Write events into file
        save_kind1_events_in_file(
            &kind1_events_of_five,
            "five_all_kind1_events.txt"
        )
        .await?
    } else {
        eprintln!("An error happened while fetching kind1 events")
    }

    // let filter: Filter = Filter::new()
    //     .authors([PublicKey::from_hex(config::FIVE_HEXPUBKEY).unwrap()])
    //     .kind(Kind::ContactList)
    //     .limit(1);
    //
    //
    //
    // let fetch_timeout = Duration::from_secs(10);
    // let mut pubkeys: Vec<String> = Vec::new();
    //
    // match client
    //     .get_events_of(vec![filter], EventSource::relays(Some(fetch_timeout)))
    //     .await
    // {
    //     Ok(contact_list_event) => {
    //         if let Some(contact_list_event) = contact_list_event.into_iter().next() {
    //             for (index, tag) in contact_list_event.tags.into_iter().enumerate() {
    //                 if let Some(TagStandard::PublicKey {
    //                     public_key,
    //                     uppercase:false,
    //                     .. 
    //                 }) = tag.to_standardized()
    //                 {
    //                     let index_string = index.to_string();
    //                     let pubkey_string = public_key.to_string();
    //
    //                     pubkeys.push(format!("{}:{}", index_string, pubkey_string))
    //                 }
    //             }
    //
    //             println!("contact_list:\n{:#?}", pubkeys);
    //         }
    //     }
    //     Err(e) => println!("{}", e.to_string()),
    // }
    //
    //
    //
    //                 // Publish a text note
    // // let output = client.publish_text_note("Hello world", []).await?;
    // // println!("Event ID: {}", output.id().to_bech32()?);
    // // println!("Sent to: {:?}", output.success);
    // // println!("Not sent to: {:?}", output.failed);
    //
    // // Create a text note POW event and broadcast to all connected relays
    // //
    // // let event: Event = EventBuilder::text_note("POW text note from rust-nostr", [])
    // //     .pow(20)
    // //     .to_event(&my_keys)?;
    // // client.send_event(event).await?;
    //
    // // Send multiple events at once (to all relays)
    // // let mut events: Vec<Event> = Vec::new();
    // // for i in 0..10 {
    // //     events.push(EventBuilder::text_note(format!("Event #{i}"), []).to_event(&my_keys)?);
    // // }
    // // let opts = RelaySendOptions::default();
    // // client.batch_event(events, opts).await?;
    // //
    // // // Send event to specific relays
    // // let event: Event = EventBuilder::text_note("POW text note from rust-nostr 16", [])
    // //     .pow(16)
    // //     .to_event(&my_keys)?;
    // // client
    // //     .send_event_to(["wss://relay.damus.io", "wss://relay.rip"], event)
    // //     .await?;

    Ok(())
}

fn create_client(option: ClientBuildOption) -> Client {
    let opts = Options::new()
        .gossip(true)
        .connection_timeout(Some(config::RELAY_CONNECTION_TIMEOUT))
        .send_timeout(Some(config::PUBLISH_TIMEOUT));

    match option {
        ClientBuildOption::WithNsec(secret_key) => {
            let my_keys = Keys::new(secret_key);

            Client::with_opts(&my_keys, opts)
        }
        ClientBuildOption::NoNsec => {
            Client::builder().opts(opts).build()
        }
    }
}

async fn get_kind1_events_of_user(
    client: &Client,
    user_pubkey: &PublicKey
) -> Result<Vec<Event>, Error> {
    let filter = Filter::new()
        .authors([PublicKey::from_hex(user_pubkey.to_hex()).unwrap()])
        .kind(Kind::TextNote);

    let events = client
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

async fn save_kind1_events_in_file(
    events: &Vec<Event>,
    file_name: &str
) -> Result<()> {
    let mut file = File::create(file_name).await?;

    for (index, event) in events.into_iter().enumerate() {
        file.write_all(format!("{}.\n{}\n", index, event.as_json()).as_bytes()).await?;
    }
    Ok(())
}


