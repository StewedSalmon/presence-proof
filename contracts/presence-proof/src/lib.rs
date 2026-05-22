#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    // A tuple acting as a composite key: (Student Wallet, Class Session ID)
    Record(Address, u32), 
}

#[contract]
pub struct PresenceProof;

// This is a sample contract. Replace this placeholder with your own contract logic.
// A corresponding test example is available in `test.rs`.
//
// For comprehensive examples, visit <https://github.com/stellar/soroban-examples>.
// The repository includes use cases for the Stellar ecosystem, such as data storage on
// the blockchain, token swaps, liquidity pools, and more.
//
// Refer to the official documentation:
// <https://developers.stellar.org/docs/build/smart-contracts/overview>.
#[contractimpl]
impl PresenceProof {
    // Called by the student scanning the QR code
    pub fn log_attendance(env: Env, student: Address, session_id: u32) {
        // 1. Require a cryptographic signature from the student's wallet
        student.require_auth();

        let key = DataKey::Record(student.clone(), session_id);
        
        // 2. Prevent overwriting so students can't sign in twice or alter history
        if env.storage().persistent().has(&key) {
            panic!("Student already signed in for this session.");
        }

        // 3. Fetch the network's consensus timestamp (impossible to spoof)
        let timestamp = env.ledger().timestamp();
        
        // 4. Commit the record to on-chain state
        env.storage().persistent().set(&key, &timestamp);

        // Let's say 1,728,000 ledgers (~100 days for a semester).
        env.storage().persistent().extend_ttl(&key, 1_728_000, 1_728_000);

        // EVENTS: Shout to the frontend that a student checked in
        env.events().publish(("AttendanceLogged", session_id), student);
    }

    // Called by the instructor's dashboard to verify records
    pub fn check_attendance(env: Env, student: Address, session_id: u32) -> u64 {
        let key = DataKey::Record(student, session_id);
        
        // Returns the timestamp if it exists, or 0 if they were absent
        env.storage().persistent().get(&key).unwrap_or(0)
    }
}
