#![no_std]
use soroban_sdk::{contract, contractimpl, Env, BytesN, Symbol, symbol_short};

// Key for storing data
const DATA_KEY: Symbol = symbol_short!("EXIST");

#[contract]
pub struct ProofOfExistence;

#[contractimpl]
impl ProofOfExistence {

    // Store a hash (proof)
    pub fn store_proof(env: Env, hash: BytesN<32>) {
        let timestamp = env.ledger().timestamp();

        // Store hash -> timestamp
        env.storage().instance().set(&hash, &timestamp);
    }

    // Verify if proof exists
    pub fn verify_proof(env: Env, hash: BytesN<32>) -> bool {
        env.storage().instance().has(&hash)
    }

    // Get stored timestamp
    pub fn get_proof(env: Env, hash: BytesN<32>) -> Option<u64> {
        env.storage().instance().get(&hash)
    }
}