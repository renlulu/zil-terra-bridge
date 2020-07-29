use cosmwasm_std::{
    to_binary, Api, Binary, Coin, Env, Extern, HandleResponse, InitResponse, Querier, StdError,
    StdResult, Storage,
};

use crate::msg::InitMsg;
use crate::state::{config_set, Config};

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
