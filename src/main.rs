use nostr_sdk::prelude::*;
use std::time::Duration;

mod config;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    // let secret_key = SecretKey::from_bech32(config::FIATMAXI_NSEC)?;
    // let my_keys = Keys::new(secret_key);
    //
    // let client = Client::new(&my_keys);
    let opts = Options::new()
        .gossip(true)
        .connection_timeout(Some(config::RELAY_CONNECTION_TIMEOUT))
        .send_timeout(Some(config::PUBLISH_TIMEOUT));

    let client = Client::builder().opts(opts).build();
    
    client.add_relay("wss://relay.damus.io").await?;
    client.add_relay("wss://relay.primal.net").await?;
    // client.add_relay("wss://relay.nostr.band").await?;
    client.add_relay("wss://purplepag.es").await?;

    client.connect().await;

    let filter: Filter = Filter::new()
        .authors([PublicKey::from_hex(config::FIVE_HEXPUBKEY).unwrap()])
        .kind(Kind::ContactList)
        .limit(1);



    let fetch_timeout = Duration::from_secs(10);
    let mut pubkeys: Vec<String> = Vec::new();

    match client
        .get_events_of(vec![filter], EventSource::relays(Some(fetch_timeout)))
        .await
    {
        Ok(contact_list_event) => {
            if let Some(contact_list_event) = contact_list_event.into_iter().next() {
                for (index, tag) in contact_list_event.tags.into_iter().enumerate() {
                    if let Some(TagStandard::PublicKey {
                        public_key,
                        uppercase:false,
                        .. 
                    }) = tag.to_standardized()
                    {
                        let index_string = index.to_string();
                        let pubkey_string = public_key.to_string();

                        pubkeys.push(format!("{}:{}", index_string, pubkey_string))
                    }
                }

                println!("contact_list:\n{:#?}", pubkeys);
            }
        }
        Err(e) => println!("{}", e.to_string()),
    }


                    // Get some kind1 events
    // let filter = Filter::new()
    //     .authors([PublicKey::from_hex(config::FIVE_HEXPUBKEY).unwrap()])
    //     .kind(Kind::TextNote)
    //     .limit(8);
    //
    // let events = client
    //     .get_events_of(
    //         vec![filter],
    //         EventSource::both(Some(Duration::from_secs(10))),
    //     )
    //     .await?;
    //
    // for event in events.into_iter() {
    //     println!("{}", event.as_json());
    // }


                    // Publish a text note
    // let output = client.publish_text_note("Hello world", []).await?;
    // println!("Event ID: {}", output.id().to_bech32()?);
    // println!("Sent to: {:?}", output.success);
    // println!("Not sent to: {:?}", output.failed);

    // Create a text note POW event and broadcast to all connected relays
    //
    // let event: Event = EventBuilder::text_note("POW text note from rust-nostr", [])
    //     .pow(20)
    //     .to_event(&my_keys)?;
    // client.send_event(event).await?;

    // Send multiple events at once (to all relays)
    // let mut events: Vec<Event> = Vec::new();
    // for i in 0..10 {
    //     events.push(EventBuilder::text_note(format!("Event #{i}"), []).to_event(&my_keys)?);
    // }
    // let opts = RelaySendOptions::default();
    // client.batch_event(events, opts).await?;
    //
    // // Send event to specific relays
    // let event: Event = EventBuilder::text_note("POW text note from rust-nostr 16", [])
    //     .pow(16)
    //     .to_event(&my_keys)?;
    // client
    //     .send_event_to(["wss://relay.damus.io", "wss://relay.rip"], event)
    //     .await?;

    Ok(())
}
