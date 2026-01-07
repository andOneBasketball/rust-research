use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: u64,
    name: String,
    active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct AccountLike {
    owner: String,

    #[serde(with = "serde_bytes")]
    data: Vec<u8>,
}

#[derive(Debug, BorshSerialize, BorshDeserialize)]
struct Counter {
    count: u64,
}

fn main() {
    println!("====== SERDE JSON DEMO ======");

    let user = User {
        id: 1,
        name: "Alice".to_string(),
        active: true,
    };

    let json = serde_json::to_string_pretty(&user).unwrap();
    println!("User -> JSON:\n{}", json);

    let decoded_user: User = serde_json::from_str(&json).unwrap();
    println!("JSON -> User: {:?}\n", decoded_user);

    println!("====== SERDE BYTES DEMO ======");

    let account = AccountLike {
        owner: "SolanaProgram".to_string(),
        data: vec![1, 2, 3, 4, 5],
    };

    let account_json = serde_json::to_string(&account).unwrap();
    println!("AccountLike -> JSON:\n{}\n", account_json);

    println!("====== BORSH DEMO ======");

    let counter = Counter { count: 42 };

    let bytes = borsh::to_vec(&counter).unwrap();
    println!("Counter -> borsh bytes: {:?}", bytes);
    println!("borsh bytes length: {}", bytes.len());

    let decoded_counter = Counter::try_from_slice(&bytes).unwrap();
    println!("borsh bytes -> Counter: {:?}", decoded_counter);
}
