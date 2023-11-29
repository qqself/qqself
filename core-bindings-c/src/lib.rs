use qqself_core::encryption::hash::StableHash;

uniffi::include_scaffolding!("qqself");

pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

pub fn hello() -> String {
    "Shouldn't it be be public??".to_string()
}

pub fn string_hash(input: String) -> String {
    StableHash::hash_string(&input).to_string()
}
