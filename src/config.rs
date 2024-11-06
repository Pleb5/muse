use std::time::Duration;

pub const FIATMAXI_NSEC: &str =
    "nsec1mq648dvh66lfn9v8rxqswtled6w779ptrev43fchdtptqfxse7xqthudsk";
pub const FIVE_HEXPUBKEY: &str =
    "d04ecf33a303a59852fdb681ed8b412201ba85d8d2199aec73cb62681d62aa90";

pub const SATSHOOT_HEXPUBKEY: &str =
    "e3244843f8ab6483827e305e5b9d7f61b9eb791aa274d2a36836f3999c767650";

pub const RELAY_CONNECTION_TIMEOUT: Duration = Duration::from_secs(4);
pub const PUBLISH_TIMEOUT: Duration = Duration::from_secs(4);

pub const BOOTSTRAP_RELAYS: [&str; 4] = [
    "wss://relay.damus.io",
    "wss://relay.primal.net",
    "wss://relay.nostr.band",
    "wss://purplepag.es",
];

pub const MIN_WOT_SCORE: i32 = 3;

pub const DIRECT_FOLLOW_WOT_SCORE: i32 = 4;
pub const INDIRECT_FOLLOW_WOT_SCORE: i32 = 2;

pub const DIRECT_MUTE_WOT_SCORE: i32 = -2;
pub const INDIRECT_MUTE_WOT_SCORE: i32 = -1;

pub const DIRECT_REPORT_WOT_SCORE: i32 = -2;
pub const INDIRECT_REPORT_WOT_SCORE: i32 = -1;
