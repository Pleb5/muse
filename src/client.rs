use std::sync::OnceLock;

use nostr_sdk::prelude::*;

// A Singleton Client instance that is shared across threads and is thread-safe
static INSTANCE: OnceLock<Client> = OnceLock::new();

pub fn initialize_client_singleton(option: ClientBuildOption) {
    let client = create_client(option);
    INSTANCE
        .set(client)
        .expect("Client Singleton already initialized!");
}

pub fn get_client() -> &'static Client {
    INSTANCE.get().expect("Client Singleton NOT initialized!")
}

fn create_client(option: ClientBuildOption) -> Client {
    let opts = Options::new().gossip(true);

    match option {
        ClientBuildOption::WithNsec(secret_key) => {
            let keys = Keys::new(secret_key);
            Client::builder().signer(keys).opts(opts).build()
        }
        ClientBuildOption::NoNsec => Client::builder().opts(opts).build(),
    }
}

pub enum ClientBuildOption {
    WithNsec(SecretKey),
    NoNsec,
}
