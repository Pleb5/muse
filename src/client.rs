use nostr_sdk::prelude::{Client, Keys, SecretKey, Options};
use std::sync::{OnceLock, Arc};

use crate::config;


// A Singleton Client instance that is shared across threads and is thread-safe
static INSTANCE: OnceLock<Arc<Client>> = OnceLock::new();

pub fn initialize_client_singleton(option: ClientBuildOption) {
    let client = create_client(option);
    INSTANCE.set(Arc::new(client)).expect("Client Singleton already initialized!");
}

pub fn get_client() -> Arc<Client> {
    INSTANCE.get().expect("Client Singleton NOT initialized!").clone()
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

pub enum ClientBuildOption {
    WithNsec(SecretKey),
    NoNsec,
}
