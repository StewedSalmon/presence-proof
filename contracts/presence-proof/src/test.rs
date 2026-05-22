#![cfg(test)] // This tells the compiler to ONLY include this file during testing

use super::*; // Imports everything from your lib.rs
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn test_attendance_flow() {
    // 1. Initialize the mock blockchain environment
    let env = Env::default();
    
    // 2. Register your contract in this mock environment
    let contract_id = env.register_contract(None, PresenceProof);
    
    // The SDK automatically generates a "Client" to interact with your contract
    let client = PresenceProofClient::new(&env, &contract_id);

    // 3. Generate a fake student wallet address for testing
    let student = Address::generate(&env);
    
    // Mock the student authenticating the transaction
    env.mock_all_auths(); 

    let session_id = 101;

    // 4. Action: The student logs their attendance
    client.log_attendance(&student, &session_id);

    // 5. Verification: The instructor checks the attendance
    let timestamp = client.check_attendance(&student, &session_id);
    
    // 6. Assertion: The timestamp should be greater than 0 (meaning it exists)
    assert!(timestamp > 0);
    
    // Optional: Print it to the console during tests to see it working
    std::println!("Success! Student logged at ledger time: {}", timestamp);
}