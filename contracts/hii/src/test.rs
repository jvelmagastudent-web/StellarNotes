#![cfg(test)]

use super::*;
use soroban_sdk::{Env, String};

#[test]
fn test_save_and_get_note() {

    let env = Env::default();

    let contract_id = env.register(NotesContract, ());
    let client = NotesContractClient::new(&env, &contract_id);

    client.save_note(&String::from_str(&env, "Hello Stellar"));

    let result = client.get_note();

    assert_eq!(result, String::from_str(&env, "Hello Stellar"));
}