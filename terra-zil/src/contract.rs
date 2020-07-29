use cosmwasm_std::{
    log, to_binary, Api, Binary, Coin, CosmosMsg, Env, Extern, HandleResponse, HumanAddr,
    InitResponse, Querier, StdError, StdResult, Storage, Uint128,
};

use crate::msg::{HandleMsg, QueryMsg, ConfigResponse};
use crate::msg::InitMsg;
use crate::state::{config_set, lock_map_get, lock_map_set, total_amount_increase, Config, config_get};
use std::ops::Add;

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    config_set(
        &mut deps.storage,
        &Config {
            name: msg.name,
            owner: env.message.sender,
        },
    )?;

    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::Lock {  } => try_lock(deps, env),
        HandleMsg::Unlock { amount } => try_unlock(deps, env, &amount),
    }
}

fn try_lock<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
) -> StdResult<HandleResponse> {
    // canonical address
    let sender_address = &env.message.sender;
    let funds = &env.message.sent_funds;

    // for now, we only allow use transfer token krw
    if funds.len() != 1 || funds[0].denom != "ukrw" {
        return Err(StdError::generic_err("dd"));
    }

    let fund = &env.message.sent_funds[0];
    let amount = fund.amount;
    total_amount_increase(&mut deps.storage, amount);

    let old_lock_amount = lock_map_get(&deps.storage, sender_address);
    let new_lock_amount = old_lock_amount.add(amount);
    lock_map_set(&mut deps.storage, sender_address, &new_lock_amount);

    let res = HandleResponse {
        messages: vec![],
        log: vec![
            log("action", "lock"),
            log("address", sender_address),
            log("amount", amount),
        ],
        data: None,
    };

    Ok(res)
}

fn try_unlock<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    amount: &Uint128,
) -> StdResult<HandleResponse> {
    let res = HandleResponse {
        messages: vec![],
        log: vec![log("action", "unlock"), log("amount", amount)],
        data: None,
    };

    Ok(res)
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => {
            let config = config_get(&deps.storage)?;
            let out = to_binary(&ConfigResponse {
                name: config.name,
                owner: deps.api.human_address(&config.owner)?,
            })?;
            Ok(out)
        }
    }
}
