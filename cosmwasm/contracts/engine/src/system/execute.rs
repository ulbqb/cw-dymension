use cosmwasm_std::{Env, Event, Response, StdError, Storage};
use dymension_std::types::dymensionxyz::dymension::rollapp::{
    BlockHeightToFinalizationQueue, StateStatus,
};

use crate::error::ContractError;
use crate::rollapp::state::*;
use crate::system::types::*;

pub fn end_blocks(storage: &mut dyn Storage, env: Env) -> Result<Response, ContractError> {
    let mut res = Response::new().add_attribute("method", "end_blocks");
    for queue in
        get_block_height_to_finalization_queue_range(storage, env.block.height, PROCESSING_MAX_NUM)?
    {
        let end_block_res = end_block(storage, queue.1)?;
        res = res.add_events(end_block_res.events);
    }

    return Ok(res);
}

fn end_block(
    storage: &mut dyn Storage,
    queue: BlockHeightToFinalizationQueue,
) -> Result<Response, ContractError> {
    let mut res = Response::new();
    for index in queue.finalization_queue.into_iter() {
        let mut state_info =
            match get_state_info(storage, index.rollapp_id.clone(), index.index.clone()) {
                Ok(x) => x,
                Err(_) => {
                    return Err(StdError::generic_err(format!(
                "Missing stateInfo data when trying to finalize, rollappID={}, height={}, index={}",
                index.rollapp_id.clone(), queue.finalization_height, index.index.clone()
            ))
                    .into())
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
