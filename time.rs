#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Address, Symbol, token};

// Define the contract struct
#[contract]
pub struct TimeLockedVault;

// Define a Vault struct to store deposits
#[derive(Clone)]
pub struct Vault {
    depositor: Address,
    beneficiary: Address,
    amount: u64,
    unlock_time: u64,
}

#[contractimpl]
impl TimeLockedVault {
    pub fn deposit(env: Env, depositor: Address, beneficiary: Address, amount: u64, unlock_time: u64) -> Symbol {
        let vault = Vault { depositor, beneficiary, amount, unlock_time };
        env.storage().set(&Symbol::new("vault"), &vault);
        Symbol::new("DepositSuccessful")
    }

    pub fn withdraw(env: Env, caller: Address) -> Symbol {
        let vault: Vault = env.storage().get(&Symbol::new("vault")).unwrap();

        // Ensure withdrawal is only after the unlock time
        if env.ledger().timestamp() < vault.unlock_time {
            return Symbol::new("TooEarly");
        }

        // Ensure only the beneficiary can withdraw
        if caller != vault.beneficiary {
            return Symbol::new("Unauthorized");
        }

        env.storage().remove(&Symbol::new("vault"));
        Symbol::new("WithdrawalSuccessful")
    }
}
