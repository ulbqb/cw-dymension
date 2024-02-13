use cosmwasm_std::{Env, Event, Response, StdError, Storage};
use dymension_std::types::dymensionxyz::dymension::rollapp::{
    BlockHeightToFinalizationQueue, MsgCreateRollapp, MsgUpdateState, Rollapp, StateInfo,
    StateInfoIndex, StateStatus,
};

use crate::error::ContractError;
use crate::rollapp::state::{
    deployer_whitelist, dispute_period_in_blocks, get_all_rollapp,
    get_block_height_to_finalization_queue, get_latest_state_info_index, get_rollapp,
    get_state_info, rollapp_enable, set_block_height_to_finalization_queue,
    set_latest_state_info_index, set_rollapp, set_state_info,
};
use crate::rollapp::types::{
    ATTRIBUTE_KEY_DA_PATH, ATTRIBUTE_KEY_NUM_BLOCKS, ATTRIBUTE_KEY_ROLLAPP_ID,
    ATTRIBUTE_KEY_START_HEIGHT, ATTRIBUTE_KEY_STATE_INFO_INDEX, ERR_LOGIC,
    ERR_MULTI_UPDATE_STATE_IN_BLOCK, ERR_ROLLAPPS_DISABLED,
    ERR_ROLLAPP_CREATOR_EXCEED_MAXIMUM_ROLLAPPS, ERR_ROLLAPP_EXISTS,
    ERR_UNAUTHORIZED_ROLLAPP_CREATOR, ERR_UNKNOWN_ROLLAPP_ID, ERR_VERSION_MISMATCH,
    ERR_WRONG_BLOCK_HEIGHT, EVENT_TYPE_STATE_UPDATE,
};
use crate::sequencer::handler as seq_handler;

pub fn create_rollapp(
    storage: &mut dyn Storage,
    msg: MsgCreateRollapp,
) -> Result<Response, ContractError> {
    if !rollapp_enable(storage)? {
        return Err(StdError::generic_err(ERR_ROLLAPPS_DISABLED).into());
    }

    if get_rollapp(storage, msg.rollapp_id.clone()).is_ok() {
        return Err(StdError::generic_err(ERR_ROLLAPP_EXISTS).into());
    }

    if let Ok(whitelist) = deployer_whitelist(storage) {
        if whitelist.len() > 0 {
            let mut b_in_whitelist = false;
            for item in whitelist.into_iter() {
                if item.address == msg.creator {
                    b_in_whitelist = true;

                    if item.max_rollapps > 0 {
                        let mut rollapps_num_of_creator = 0;
                        for (_, r) in get_all_rollapp(storage)?.into_iter() {
                            if r.creator == msg.creator {
                                rollapps_num_of_creator += 1;
                            }
                        }
                        if rollapps_num_of_creator >= item.max_rollapps {
                            return Err(StdError::generic_err(
                                ERR_ROLLAPP_CREATOR_EXCEED_MAXIMUM_ROLLAPPS,
                            )
                            .into());
                        }
                    }

                    break;
                }
            }
            if !b_in_whitelist {
                return Err(StdError::generic_err(ERR_UNAUTHORIZED_ROLLAPP_CREATOR).into());
            }
        }
    }

    set_rollapp(
        storage,
        Rollapp {
            rollapp_id: msg.rollapp_id.clone(),
            creator: msg.creator.clone(),
            version: 0,
            code_stamp: "".into(),
            genesis_path: "".into(),
            max_withholding_blocks: 0,
            max_sequencers: msg.max_sequencers.clone(),
            permissioned_addresses: msg.permissioned_addresses.clone(),
            token_metadata: msg.metadatas.clone(),
        },
    );

    Ok(Response::new().add_attribute("method", "create_rollapp"))
}

pub fn update_state(
    env: Env,
    storage: &mut dyn Storage,
    msg: MsgUpdateState,
) -> Result<Response, ContractError> {
    if !rollapp_enable(storage)? {
        return Err(StdError::generic_err(ERR_ROLLAPPS_DISABLED).into());
    }

    let rollapp = match get_rollapp(storage, msg.rollapp_id.clone()) {
        Ok(ra) => ra,
        Err(_) => return Err(StdError::generic_err(ERR_UNKNOWN_ROLLAPP_ID).into()),
    };

    if rollapp.version != msg.version {
        return Err(StdError::generic_err(format!(
            "{}: rollappId({}) current version is {}, but got {}",
            ERR_VERSION_MISMATCH, msg.rollapp_id, rollapp.version, msg.version
        ))
        .into());
    }

    // TODO: BeforeUpdateState
    seq_handler::before_update_state(storage, msg.creator.clone(), msg.rollapp_id.clone())?;

    if !rollapp
        .permissioned_addresses
        .contains(&msg.creator.clone())
    {
        return Err(StdError::generic_err(format!(
            "{}: unpermissioned sequencer ({}) is registered for rollappId({})",
            ERR_LOGIC, msg.creator, msg.rollapp_id
        ))
        .into());
    }

    let new_index: u64 = match get_latest_state_info_index(storage, msg.rollapp_id.clone()) {
        Ok(latest_state_info_index) => {
            match get_state_info(
                storage,
                msg.rollapp_id.clone(),
                latest_state_info_index.index,
            ) {
                Ok(state_info) => {
                    if state_info.creation_height == env.block.height {
                        return Err(StdError::generic_err(ERR_MULTI_UPDATE_STATE_IN_BLOCK).into());
                    }
                    let expected_start_height = state_info.start_height + state_info.num_blocks;
                    if expected_start_height != msg.start_height {
                        return Err(StdError::generic_err(format!(
                            "{}: expected height ({}), but received ({})",
                            ERR_WRONG_BLOCK_HEIGHT, expected_start_height, msg.start_height
                        ))
                        .into());
                    }
                    latest_state_info_index.index + 1
                }
                Err(_) => {
                    return Err(StdError::generic_err(format!(
                        "{}: missing stateInfo for state-index ({}) of rollappId({})",
                        ERR_LOGIC, latest_state_info_index.index, msg.rollapp_id
                    ))
                    .into())
                }
            }
        }
        Err(_) => {
            if msg.start_height != 1 {
                return Err(StdError::generic_err(format!(
                    "{}: expected height 1, but received ({})",
                    ERR_WRONG_BLOCK_HEIGHT, msg.start_height
                ))
                .into());
            }
            1
        }
    };

    let state_info_index = StateInfoIndex {
        rollapp_id: msg.rollapp_id.clone(),
        index: new_index,
    };
    set_latest_state_info_index(storage, state_info_index.clone());

    set_state_info(
        storage,
        StateInfo {
            state_info_index: Some(state_info_index.clone()),
            sequencer: msg.creator,
            start_height: msg.start_height,
            num_blocks: msg.num_blocks,
            da_path: msg.da_path.clone(),
            version: msg.version,
            creation_height: env.block.height,
            status: StateStatus::Received.into(),
            b_ds: msg.b_ds,
        },
    );

    let finalization_height = env.block.height + dispute_period_in_blocks(storage)?;

    let new_finalization_queue =
        match get_block_height_to_finalization_queue(storage, finalization_height) {
            Ok(queue) => {
                let mut new = queue.finalization_queue;
                new.push(state_info_index.clone());
                new
            }
            Err(_) => vec![state_info_index.clone()],
        };

    set_block_height_to_finalization_queue(
        storage,
        BlockHeightToFinalizationQueue {
            finalization_height: finalization_height,
            finalization_queue: new_finalization_queue,
        },
    );

    Ok(Response::new()
        .add_attribute("method", "update_state")
        .add_event(
            Event::new(EVENT_TYPE_STATE_UPDATE)
                .add_attribute(ATTRIBUTE_KEY_ROLLAPP_ID, msg.rollapp_id)
                .add_attribute(
                    ATTRIBUTE_KEY_STATE_INFO_INDEX,
                    state_info_index.index.to_string(),
                )
                .add_attribute(ATTRIBUTE_KEY_START_HEIGHT, msg.start_height.to_string())
                .add_attribute(ATTRIBUTE_KEY_NUM_BLOCKS, msg.num_blocks.to_string())
                .add_attribute(ATTRIBUTE_KEY_DA_PATH, msg.da_path),
        ))
}
