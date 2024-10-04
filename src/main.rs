use nostr_sdk::prelude::*;
use std::time::Duration;

const BECH32_SK: &str = "nsec**** # MODIFY ME!!!

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let secret_key = SecretKey::from_bech32(BECH32_SK)?;
    let my_keys = Keys::new(secret_key);

    let client = Client::new(&my_keys);
    client.add_relay("wss://relay.damus.io").await?;
    client.add_relay("wss://relay.primal.net").await?;
    client.add_relay("wss://relay.nostr.band").await?;
    client.add_relay("wss://purplepag.es").await?;

    client.connect().await;

    let filter: Filter = Filter::default()
        .authors([my_keys.public_key()])
        .kind(Kind::RelayList);


    let timeout = Duration::from_secs(5);

    let relays = client
        .get_events_of(vec![filter], EventSource::both(Some(timeout)))
        .await?;

    println!("relays:\n{:#?}", relays);

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
