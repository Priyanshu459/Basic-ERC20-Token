#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol, symbol_short, String};

#[contracttype]
#[derive(Clone)]
pub struct TokenMetadata {
    pub name: String,
    pub symbol: String,
    pub decimals: u32,
}

const BALANCES: Symbol = symbol_short!("BAL");
const METADATA: Symbol = symbol_short!("META");
const TOTAL_SUPPLY: Symbol = symbol_short!("TOTAL");

#[contract]
pub struct BasicERC20Token;

#[contractimpl]
impl BasicERC20Token {
    // Initialize token with metadata and initial supply
    pub fn initialize(
        env: Env,
        name: String,
        symbol: String,
        decimals: u32,
        initial_supply: i128,
        owner: Address,
    ) {
        // Set token metadata
        let meta = TokenMetadata {
            name,
            symbol,
            decimals,
        };
        env.storage().instance().set(&METADATA, &meta);

        // Set total supply
        env.storage().instance().set(&TOTAL_SUPPLY, &initial_supply);

        // Set balance for the owner
        env.storage().instance().set(&(BALANCES, &owner), &initial_supply);
    }

    // Get token balance for a user
    pub fn balance_of(env: Env, user: Address) -> i128 {
        env.storage().instance().get(&(BALANCES, &user)).unwrap_or(0)
    }

    // Transfer tokens from sender to recipient
    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();

        let mut sender_balance = Self::balance_of(env.clone(), from.clone());
        if sender_balance < amount {
            panic!("Insufficient balance");
        }

        sender_balance -= amount;
        let mut receiver_balance = Self::balance_of(env.clone(), to.clone());
        receiver_balance += amount;

        env.storage().instance().set(&(BALANCES, &from), &sender_balance);
        env.storage().instance().set(&(BALANCES, &to), &receiver_balance);
    }

    // Get token metadata
    pub fn get_metadata(env: Env) -> TokenMetadata {
        env.storage().instance().get(&METADATA).unwrap()
    }
}
