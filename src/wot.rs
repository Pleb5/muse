use nostr_sdk::prelude::*;

use crate::client::get_client;

use crate::config::{
    DIRECT_FOLLOW_WOT_SCORE, DIRECT_MUTE_WOT_SCORE, DIRECT_REPORT_WOT_SCORE,
    INDIRECT_FOLLOW_WOT_SCORE, INDIRECT_MUTE_WOT_SCORE, INDIRECT_REPORT_WOT_SCORE, MIN_WOT_SCORE,
};

use std::collections::HashMap;
use std::time::Duration;

use dashmap::DashSet;
use once_cell::sync::Lazy;

const KEY_TO_FOLLOWS: &str = "follows";
const KEY_TO_MUTES: &str = "mutes";
const KEY_TO_REPORTS: &str = "reports";

pub static WOT: Lazy<DashSet<PublicKey>> = Lazy::new(DashSet::new);

pub async fn update_wot(user_pubkey: &PublicKey) -> Result<()> {
    let mut network_wot_scores: HashMap<PublicKey, i32> = HashMap::new();

    let mut authors = vec![*user_pubkey];

    let user_wot_events_map = fetch_wot_events_of_users(authors.clone()).await?;

    authors.clear();

    if let Some(follow_events) = user_wot_events_map.get(KEY_TO_FOLLOWS) {
        for follow_event in follow_events.iter() {
            // Reserve capacity
            authors.reserve_exact(follow_event.tags.public_keys().count());

            // Extend
            authors.extend(follow_event.tags.public_keys());
        }
    }

    update_wot_scores(
        &mut network_wot_scores,
        user_wot_events_map,
        DIRECT_FOLLOW_WOT_SCORE,
        DIRECT_MUTE_WOT_SCORE,
        DIRECT_REPORT_WOT_SCORE,
    );

    let follows_wot_events_map = fetch_wot_events_of_users(authors.clone()).await?;

    update_wot_scores(
        &mut network_wot_scores,
        follows_wot_events_map,
        INDIRECT_FOLLOW_WOT_SCORE,
        INDIRECT_MUTE_WOT_SCORE,
        INDIRECT_REPORT_WOT_SCORE,
    );

    for (public_key, wot_score) in network_wot_scores.into_iter() {
        if wot_score >= MIN_WOT_SCORE {
            WOT.insert(public_key);
        }
    }

    Ok(())
}

pub async fn fetch_wot_events_of_users(
    user_pubkeys: Vec<PublicKey>,
) -> Result<HashMap<&'static str, Vec<Event>>> {
    let mut wot_events_map: HashMap<&'static str, Vec<Event>> = HashMap::new();

    let filter: Filter = Filter::new()
        .authors(user_pubkeys.iter().cloned())
        .kind(Kind::ContactList)
        .kind(Kind::MuteList)
        .kind(Kind::Reporting);

    let fetch_timeout = Duration::from_secs(5);

    let wot_events = get_client()
        .fetch_events(vec![filter], Some(fetch_timeout))
        .await?;

    for wot_event in wot_events.into_iter() {
        match wot_event.kind {
            Kind::ContactList => wot_events_map
                .entry(KEY_TO_FOLLOWS)
                .or_default()
                .push(wot_event),

            Kind::MuteList => wot_events_map
                .entry(KEY_TO_MUTES)
                .or_default()
                .push(wot_event),

            Kind::Reporting => wot_events_map
                .entry(KEY_TO_REPORTS)
                .or_default()
                .push(wot_event),

            _ => (),
        }
    }
    Ok(wot_events_map)
}

fn update_wot_scores(
    network_wot_scores: &mut HashMap<PublicKey, i32>,
    wot_events_map: HashMap<&'static str, Vec<Event>>,
    follow_wot_score: i32,
    mute_wot_score: i32,
    report_wot_score: i32,
) {
    if let Some(follow_events) = wot_events_map.get(KEY_TO_FOLLOWS) {
        for follow_event in follow_events.iter() {
            for tag in follow_event.tags.iter() {
                if let Some(TagStandard::PublicKey {
                    public_key,
                    uppercase: false,
                    ..
                }) = tag.as_standardized()
                {
                    *network_wot_scores
                        .entry(*public_key)
                        .or_insert(follow_wot_score) += follow_wot_score;
                }
            }
        }
    }

    if let Some(mute_events) = wot_events_map.get(KEY_TO_MUTES) {
        for mute_event in mute_events.iter() {
            for tag in mute_event.tags.iter() {
                if let Some(TagStandard::PublicKey {
                    public_key,
                    uppercase: false,
                    ..
                }) = tag.as_standardized()
                {
                    *network_wot_scores
                        .entry(*public_key)
                        .or_insert(mute_wot_score) += mute_wot_score;
                }
            }
        }
    }

    if let Some(report_events) = wot_events_map.get(KEY_TO_REPORTS) {
        for report_event in report_events {
            if let Some(public_key) = report_event.tags.public_keys().next() {
                *network_wot_scores
                    .entry(*public_key)
                    .or_insert(mute_wot_score) += report_wot_score;
            }
        }
    }
}
