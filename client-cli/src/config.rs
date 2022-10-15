use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    keys: Keys,
}

impl Config {
    pub fn new(public_key: String, private_key: String) -> Self {
        Config {
            keys: Keys {
                public_key,
                private_key,
            },
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Keys {
    public_key: String,
    private_key: String,
}
