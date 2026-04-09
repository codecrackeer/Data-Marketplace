#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, String, Address};

#[derive(Clone)]
#[contracttype]
pub struct DataAsset {
    pub owner: Address,
    pub name: String,
    pub price: i128,
}

#[contracttype]
pub enum DataKey {
    Asset(u64),
    Counter,
}

#[contract] // ✅ REQUIRED
pub struct DataMarketplace;

#[contractimpl]
impl DataMarketplace {

    pub fn add_data(env: Env, owner: Address, name: String, price: i128) -> u64 {
        owner.require_auth();

        let mut counter: u64 = env.storage()
            .instance()
            .get(&DataKey::Counter)
            .unwrap_or(0);

        counter += 1;

        let asset = DataAsset {
            owner: owner.clone(),
            name,
            price,
        };

        env.storage().instance().set(&DataKey::Asset(counter), &asset);
        env.storage().instance().set(&DataKey::Counter, &counter);

        counter
    }

    pub fn get_data(env: Env, id: u64) -> DataAsset {
        env.storage()
            .instance()
            .get(&DataKey::Asset(id))
            .unwrap()
    }

    pub fn buy_data(env: Env, buyer: Address, id: u64) {
        buyer.require_auth();

        let asset: DataAsset = env.storage()
            .instance()
            .get(&DataKey::Asset(id))
            .unwrap();

        // Placeholder for payment logic

        env.events().publish(
            (Symbol::new(&env, "purchase"),),
            (buyer, asset.owner, id),
        );
    }
}