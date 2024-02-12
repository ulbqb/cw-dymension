#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{
    ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg, RollappExecute, RollappQuery,
    SequencerExecute, SequencerQuery,
};
use crate::rollapp::{
    execute as rollapp_exec, instantiate as rollapp_init, query as rollapp_query,
};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:engine";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Handling contract instantiation
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    rollapp_init::instantiate(deps.storage, msg.rollapp);

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

/// Handling contract migration
/// To make a contract migratable, you need
/// - this entry_point implemented
/// - only contract admin can migrate, so admin has to be set at contract initiation time
/// Handling contract execution
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    match msg {
        // Find matched incoming message variant and execute them with your custom logic.
        //
        // With `Response` type, it is possible to dispatch message to invoke external logic.
        // See: https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md#dispatching-messages
    }
}

/// Handling contract execution
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Rollapp(exec_type) => match exec_type {
            RollappExecute::CreateRollapp(exec_msg) => {
                rollapp_exec::create_rollapp(deps.storage, exec_msg)
            }
            RollappExecute::UpdateState(exec_msg) => {
                rollapp_exec::update_state(env, deps.storage, exec_msg)
            }
        },
        ExecuteMsg::Sequencer(exec_type) => match exec_type {
            SequencerExecute::CreateSequencer(_) => Ok(Response::new()),
        },
    }
}

/// Handling contract query
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Rollapp(q_type) => match q_type {
            RollappQuery::Params(_) => to_binary(&rollapp_query::params(deps.storage)?),
            RollappQuery::Rollapp(req) => to_binary(&rollapp_query::rollapp(deps.storage, req)?),
            RollappQuery::RollappByEip155(req) => {
                to_binary(&rollapp_query::rollapp_eip155(deps.storage, req)?)
            }
            RollappQuery::RollappAll(req) => {
                to_binary(&rollapp_query::rollapp_all(deps.storage, req)?)
            }
            RollappQuery::LatestStateIndex(req) => {
                to_binary(&rollapp_query::latest_state_index(deps.storage, req)?)
            }
            RollappQuery::StateInfo(req) => {
                to_binary(&rollapp_query::state_info(deps.storage, req)?)
            }
            RollappQuery::StateInfoAll(req) => {
                to_binary(&rollapp_query::state_info_all(deps.storage, req)?)
            }
        },
        QueryMsg::Sequencer(q_type) => match q_type {
            SequencerQuery::Params(_) => Ok(vec![].into()),
            SequencerQuery::Sequencer(_) => Ok(vec![].into()),
            SequencerQuery::SequencerAll(_) => Ok(vec![].into()),
            SequencerQuery::SequencersByRollapp(_) => Ok(vec![].into()),
            SequencerQuery::SequencersByRollappAll(_) => Ok(vec![].into()),
            SequencerQuery::Scheduler(_) => Ok(vec![].into()),
            SequencerQuery::SchedulerAll(_) => Ok(vec![].into()),
        },
    }
}

/// Handling submessage reply.
/// For more info on submessage and reply, see https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md#submessages
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(_deps: DepsMut, _env: Env, _msg: Reply) -> Result<Response, ContractError> {
    // With `Response` type, it is still possible to dispatch message to invoke external logic.
    // See: https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md#dispatching-messages

    todo!()
}
