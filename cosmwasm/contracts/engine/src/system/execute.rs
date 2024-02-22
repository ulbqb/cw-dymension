use cosmwasm_std::{Env, Event, Response, StdError, StdResult, Storage};
use dymension_std::types::dymensionxyz::dymension::rollapp::{
    BlockHeightToFinalizationQueue, StateStatus,
};

use crate::rollapp::state::{
    get_block_height_to_finalization_queue_range, get_state_info, set_latest_finalized_state_index,
    set_state_info,
};
use crate::rollapp::types::{
    ATTRIBUTE_KEY_NUM_BLOCKS, ATTRIBUTE_KEY_ROLLAPP_ID, ATTRIBUTE_KEY_START_HEIGHT,
    ATTRIBUTE_KEY_STATE_INFO_INDEX, ATTRIBUTE_KEY_STATUS, EVENT_TYPE_STATUS_CHANGE,
};
use crate::system::types::{get_min_height, set_min_height, MsgEndBlocks, PROCESSING_MAX_NUM};

pub fn end_blocks(storage: &mut dyn Storage, env: Env, msg: MsgEndBlocks) -> StdResult<Response> {
    let mut num_to_process = PROCESSING_MAX_NUM;
    if let Some(n) = msg.num {
        if n < PROCESSING_MAX_NUM {
            num_to_process = n;
        }
    }
    let min_height = get_min_height(storage).unwrap_or(1);
    let queue_list = get_block_height_to_finalization_queue_range(
        storage,
        min_height,
        env.block.height,
        num_to_process,
    )?;
    if queue_list.len() == 0 {
        return Err(StdError::generic_err("No block to finalize"));
    }

    let mut res = Response::new().add_attribute("method", "end_blocks");
    for queue in queue_list.clone() {
        let end_block_res = end_block(storage, queue.1)?;
        res = res.add_events(end_block_res.events);
    }

    set_min_height(storage, queue_list.last().unwrap().1.finalization_height);

    return Ok(res);
}

fn end_block(
    storage: &mut dyn Storage,
    queue: BlockHeightToFinalizationQueue,
) -> StdResult<Response> {
    let mut res = Response::new();
    for index in queue.finalization_queue.into_iter() {
        let mut state_info =
            match get_state_info(storage, index.rollapp_id.clone(), index.index.clone()) {
                Ok(x) => x,
                Err(_) => {
                    return Err(StdError::generic_err(format!(
                "Missing stateInfo data when trying to finalize, rollappID={}, height={}, index={}",
                index.rollapp_id.clone(), queue.finalization_height, index.index.clone()
            )))
                }
            };

        state_info.status = StateStatus::Finalized.into();

        set_state_info(storage, state_info.clone());
        set_latest_finalized_state_index(storage, index.clone());

        // TODO: AfterStateFinalized

        res = res.add_event(
            Event::new(EVENT_TYPE_STATUS_CHANGE)
                .add_attribute(ATTRIBUTE_KEY_ROLLAPP_ID, index.rollapp_id)
                .add_attribute(ATTRIBUTE_KEY_STATE_INFO_INDEX, index.index.to_string())
                .add_attribute(
                    ATTRIBUTE_KEY_START_HEIGHT,
                    state_info.start_height.to_string(),
                )
                .add_attribute(ATTRIBUTE_KEY_NUM_BLOCKS, state_info.num_blocks.to_string())
                .add_attribute(ATTRIBUTE_KEY_STATUS, state_info.status.to_string()),
        )
    }

    Ok(res)
}
