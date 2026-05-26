#![no_std]

use soroban_sdk::{contract, contractimpl, symbol_short, Env, String};

#[contract]
pub struct NotesContract;

#[contractimpl]
impl NotesContract {

    // Save a note into blockchain storage
    pub fn save_note(env: Env, note: String) {
        env.storage().instance().set(&symbol_short!("NOTE"), &note);
    }

    // Read the saved note
    pub fn get_note(env: Env) -> String {
        env.storage()
            .instance()
            .get(&symbol_short!("NOTE"))
            .unwrap_or(String::from_str(&env, "No note found"))
    }
}