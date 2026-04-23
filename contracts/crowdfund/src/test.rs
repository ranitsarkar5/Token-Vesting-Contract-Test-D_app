#![cfg(test)]

use super::*;
use soroban_sdk::{Env, testutils::{Address as _, Ledger}};

#[test]
fn test_init_and_pledge() {
    let env = Env::default();
    let contract_id = env.register_contract(None, CrowdfundContract);
    let client = CrowdfundContractClient::new(&env, &contract_id);
    
    let creator = Address::generate(&env);
    let donor = Address::generate(&env);
    
    env.mock_all_auths();
    client.init(&creator, &100, &1000);
    
    // set ledger time
    env.ledger().with_mut(|li| li.timestamp = 500);

    client.pledge(&donor, &50);
}

#[test]
#[should_panic(expected = "Campaign has ended")]
fn test_edge_case_pledge_after_deadline() {
    let env = Env::default();
    let contract_id = env.register_contract(None, CrowdfundContract);
    let client = CrowdfundContractClient::new(&env, &contract_id);
    
    let creator = Address::generate(&env);
    let donor = Address::generate(&env);
    
    env.mock_all_auths();
    client.init(&creator, &100, &1000);
    
    env.ledger().with_mut(|li| li.timestamp = 1500); // Past deadline
    client.pledge(&donor, &50); // Should panic
}

#[test]
fn test_interaction_withdraw_success() {
    let env = Env::default();
    let contract_id = env.register_contract(None, CrowdfundContract);
    let client = CrowdfundContractClient::new(&env, &contract_id);
    
    let creator = Address::generate(&env);
    let donor = Address::generate(&env);
    
    env.mock_all_auths();
    client.init(&creator, &100, &1000);
    
    env.ledger().with_mut(|li| li.timestamp = 500);
    client.pledge(&donor, &150); // Met target
    
    env.ledger().with_mut(|li| li.timestamp = 1500); // Past deadline
    client.withdraw(); // Should succeed
}

#[test]
#[should_panic(expected = "Deadline has not passed")]
fn test_edge_case_refund_before_deadline() {
    let env = Env::default();
    let contract_id = env.register_contract(None, CrowdfundContract);
    let client = CrowdfundContractClient::new(&env, &contract_id);

    let creator = Address::generate(&env);
    let donor = Address::generate(&env);

    env.mock_all_auths();
    client.init(&creator, &100, &1000);
    client.pledge(&donor, &50);

    env.ledger().with_mut(|li| li.timestamp = 500); // Before deadline
    client.refund(&donor); // Should panic
}
