use nostr_sdk::prelude::*;

use crate::client::get_client;

use crate::config::{
    MIN_WOT_SCORE,
    DIRECT_FOLLOW_WOT_SCORE, INDIRECT_FOLLOW_WOT_SCORE,
    DIRECT_MUTE_WOT_SCORE, INDIRECT_MUTE_WOT_SCORE,
    DIRECT_REPORT_WOT_SCORE, INDIRECT_REPORT_WOT_SCORE
};

use std::time::Duration;
use std::sync::Arc;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::collections::HashMap;

const KEY_TO_FOLLOWS: &str = "follows";
const KEY_TO_MUTES: &str = "mutes";
const KEY_TO_REPORTS: &str = "reports";

pub async fn update_wot(user_pubkey: &PublicKey) {
    let mut network_wot_scores: HashMap<PublicKey, i32> = HashMap::new();

    match fetch_wot_events_of_users(&vec![user_pubkey.clone()]).await {
        Ok(wot_events_map) => {
            let follow_events = wot_events_map.entry(KEY_TO_FOLLOWS).or_default();
        },
        Err(e) => Err(e)
    }
}

pub async fn fetch_wot_events_of_users(
    user_pubkeys: &[PublicKey]
) -> Result<HashMap<&str, Vec<Event>>, Error> {
    let mut wot_events_map: HashMap<&str, Vec<Event>> = HashMap::new();

    let filter: Filter = Filter::new()
        .authors(user_pubkeys.iter().cloned())
        .kind(Kind::ContactList)
        .kind(Kind::MuteList)
        .kind(Kind::Reporting);

    let fetch_timeout = Duration::from_secs(5);

    match get_client()
        .get_events_of(vec![filter], EventSource::relays(Some(fetch_timeout)))
        .await
    {
        Ok(wot_events) => {
            for wot_event in wot_events {
                match wot_event.kind {
                    Kind::ContactList => wot_events_map
                        .entry(KEY_TO_FOLLOWS)
                        .or_insert(Vec::new())
                        .push(wot_event),

                    Kind::MuteList => wot_events_map
                        .entry(KEY_TO_MUTES)
                        .or_insert(Vec::new())
                        .push(wot_event),

                    Kind::Reporting => wot_events_map
                        .entry(KEY_TO_REPORTS)
                        .or_insert(Vec::new())
                        .push(wot_event),

                    _ => ()
                }
            }
            Ok(wot_events_map)
        }
        Err(e) => Err(e)
    }
}
