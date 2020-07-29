use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CanonicalAddr, StdResult, Storage, Uint128};
use cosmwasm_storage::{Bucket, ReadonlyBucket, ReadonlySingleton, Singleton};
use std::ops::Add;

pub static CONFIG_KEY: &[u8] = b"config";
pub static TOTAL_AMOUNT_KEY: &[u8] = b"total_amount";
pub static LOCK_MAP_KEY: &[u8] = b"lock_map";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub name: String,
    pub owner: CanonicalAddr,
}

pub fn config_set<S: Storage>(storage: &mut S, config: &Config) -> StdResult<()> {
    Singleton::new(storage, CONFIG_KEY).save(config)
}

pub fn config_get<S: Storage>(storage: &S) -> StdResult<Config> {
    ReadonlySingleton::new(storage, CONFIG_KEY).load()
}

pub fn total_amount_get<S: Storage>(storage: &S) -> Uint128 {
    match ReadonlySingleton::new(storage, TOTAL_AMOUNT_KEY).load() {
        Ok(Some(amount)) => amount,
        _ => Uint128::zero(),
    }
}

pub fn total_amount_increase<S: Storage>(storage: &mut S, amount: Uint128) -> StdResult<()> {
    let current_amount = total_amount_get(storage);
    let new_amount = current_amount.add(amount);
    Singleton::new(storage, TOTAL_AMOUNT_KEY).save(&new_amount);
    Ok(())
}

pub fn lock_map_get<S: Storage>(storage: &S, address: &CanonicalAddr) -> Uint128 {
    match ReadonlyBucket::new(LOCK_MAP_KEY, storage).may_load(address.as_slice()) {
        Ok(Some(amount)) => amount,
        _ => Uint128::zero(),
    }
}

pub fn lock_map_set<S: Storage>(
    storage: &mut S,
    address: &CanonicalAddr,
    amount: &Uint128,
) -> StdResult<()> {
    Bucket::new(LOCK_MAP_KEY, storage).save(address.as_slice(), amount)
}

#[cfg(test)]
mod test {
    use super::*;
    use cosmwasm_std::testing::MockStorage;

    #[test]
    fn total_amount() {
        let mut store = MockStorage::new();
        let old_amount = total_amount_get(&store);
        assert_eq!(old_amount, Uint128::zero());
        total_amount_increase(&mut store, Uint128(100));
        let new_amount = total_amount_get(&store);
        assert_eq!(new_amount, Uint128(100));
    }

    #[test]
    fn lock_map() {
        unimplemented!()
    }
}
