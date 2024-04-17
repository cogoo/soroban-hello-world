#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, vec, Env, Symbol, Vec};

#[contract]
pub struct HelloContract;

#[contractimpl]
impl HelloContract {
    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
        vec![
            &env,
            symbol_short!("Hello"),
            to,
            symbol_short!("Saidu"),
            symbol_short!("Hawa"),
            symbol_short!("Mitch"),
        ]
    }

    pub fn welcome(env: Env, to: Symbol) -> Vec<Symbol> {
        vec![&env, to]
    }
}

mod test;
