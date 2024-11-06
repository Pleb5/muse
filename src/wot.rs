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
use dashmap::DashSet;
use once_cell::sync::Lazy;

const KEY_TO_FOLLOWS: &str = "follows";
const KEY_TO_MUTES: &str = "mutes";
const KEY_TO_REPORTS: &str = "reports";

pub static WOT: Lazy<Arc<DashSet<PublicKey>>> = Lazy::new(|| {
    Arc::new(DashSet::new())
});

pub async fn update_wot(user_pubkey: &PublicKey) -> Result<(), Error> {
    let mut network_wot_scores: HashMap<PublicKey, i32> = HashMap::new();

    let mut authors = vec![user_pubkey.clone()];
    // TODO: follows of user should be saved here and add to authors vector before second fetch
    //
    let user_wot_events_map = fetch_wot_events_of_users(&authors)
        .await?;

    update_wot_scores(
        &mut network_wot_scores,
        user_wot_events_map,
        DIRECT_FOLLOW_WOT_SCORE,
        DIRECT_MUTE_WOT_SCORE,
        DIRECT_REPORT_WOT_SCORE
    );


    // TODO: update authors before this call!
    let user_wot_events_map = fetch_wot_events_of_users(&authors)
        .await?;

    update_wot_scores(
        &mut network_wot_scores,
        user_wot_events_map,
        DIRECT_FOLLOW_WOT_SCORE,
        DIRECT_MUTE_WOT_SCORE,
        DIRECT_REPORT_WOT_SCORE
    );

    for (public_key, wot_score) in network_wot_scores {
        // TODO: update global wot set if wot score of pubkey above min wot
    }

    Ok(())
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

fn update_wot_scores(
    network_wot_scores: &mut HashMap<PublicKey, i32>,
    wot_events_map: HashMap<&str, Vec<Event>>,
    follow_wot_score: i32,
    mute_wot_score: i32,
    report_wot_score: i32
) {

    if let Some(follow_events) = wot_events_map.get(KEY_TO_FOLLOWS) {
        for follow_event in follow_events {
            for tag in follow_event.tags.iter() {
                if let Some(TagStandard::PublicKey {
                    public_key,
                    uppercase:false,
                    .. 
                }) = tag.clone().to_standardized()
                {
                    *network_wot_scores
                        .entry(public_key)
                        .or_insert(follow_wot_score) += follow_wot_score;
                }
            }
        }
    }

    if let Some(mute_events) = wot_events_map.get(KEY_TO_MUTES) {
        for mute_event in mute_events {
            for tag in mute_event.tags.iter() {
                if let Some(TagStandard::PublicKey {
                    public_key,
                    uppercase:false,
                    .. 
                }) = tag.clone().to_standardized()
                {
                    *network_wot_scores
                        .entry(public_key)
                        .or_insert(mute_wot_score) += mute_wot_score;
                }
            }
        }
    }

    if let Some(report_events) = wot_events_map.get(KEY_TO_REPORTS) {
        for report_event in report_events {
            if let Some(public_key) = report_event.public_keys().next() {
                *network_wot_scores
                    .entry(public_key.clone())
                    .or_insert(mute_wot_score) += report_wot_score;
            }
        }
    }
}
