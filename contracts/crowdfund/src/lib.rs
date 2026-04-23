#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    TargetAmount,
    Deadline,
    Creator,
    Pledge(Address),
    TotalPledged,
    State, // 0 = Active, 1 = Succeeded, 2 = Failed
}

#[contract]
pub struct CrowdfundContract;

#[contractimpl]
impl CrowdfundContract {
    pub fn init(env: Env, creator: Address, target_amount: i128, deadline: u64) {
        creator.require_auth();
        assert!(!env.storage().instance().has(&DataKey::Creator), "Already initialized");
        
        env.storage().instance().set(&DataKey::Creator, &creator);
        env.storage().instance().set(&DataKey::TargetAmount, &target_amount);
        env.storage().instance().set(&DataKey::Deadline, &deadline);
        env.storage().instance().set(&DataKey::TotalPledged, &0i128);
        env.storage().instance().set(&DataKey::State, &0u32);
    }

    pub fn pledge(env: Env, donor: Address, amount: i128) {
        donor.require_auth();
        assert!(amount > 0, "Pledge amount must be positive");
        
        // Check deadline
        let deadline: u64 = env.storage().instance().get(&DataKey::Deadline).unwrap();
        assert!(env.ledger().timestamp() < deadline, "Campaign has ended");
        
        let state: u32 = env.storage().instance().get(&DataKey::State).unwrap();
        assert!(state == 0, "Campaign is not active");

        // We would transfer native XLM or a specific token here using standard SAC interfaces.
        // For simple demonstration, we only track the amount pledged.
        
        let mut user_pledged: i128 = env.storage().persistent().get(&DataKey::Pledge(donor.clone())).unwrap_or(0);
        user_pledged += amount;
        env.storage().persistent().set(&DataKey::Pledge(donor.clone()), &user_pledged);

        let mut total_pledged: i128 = env.storage().instance().get(&DataKey::TotalPledged).unwrap();
        total_pledged += amount;
        env.storage().instance().set(&DataKey::TotalPledged, &total_pledged);
    }

    pub fn withdraw(env: Env) {
        let creator: Address = env.storage().instance().get(&DataKey::Creator).unwrap();
        creator.require_auth();

        let target: i128 = env.storage().instance().get(&DataKey::TargetAmount).unwrap();
        let total: i128 = env.storage().instance().get(&DataKey::TotalPledged).unwrap();
        let deadline: u64 = env.storage().instance().get(&DataKey::Deadline).unwrap();

        assert!(env.ledger().timestamp() >= deadline, "Deadline has not passed");
        assert!(total >= target, "Target not met");

        env.storage().instance().set(&DataKey::State, &1u32); // Success state
    }

    pub fn refund(env: Env, donor: Address) {
        donor.require_auth();
        let target: i128 = env.storage().instance().get(&DataKey::TargetAmount).unwrap();
        let total: i128 = env.storage().instance().get(&DataKey::TotalPledged).unwrap();
        let deadline: u64 = env.storage().instance().get(&DataKey::Deadline).unwrap();

        assert!(env.ledger().timestamp() >= deadline, "Deadline has not passed");
        assert!(total < target, "Target was met, cannot refund");

        let user_pledged: i128 = env.storage().persistent().get(&DataKey::Pledge(donor.clone())).unwrap_or(0);
        assert!(user_pledged > 0, "No funds to refund");

        env.storage().persistent().set(&DataKey::Pledge(donor), &0i128);
    }
}

mod test;
