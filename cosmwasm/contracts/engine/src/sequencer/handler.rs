use cosmwasm_std::{StdError, StdResult, Storage};
use dymension_std::types::dymensionxyz::dymension::sequencer::OperatingStatus;

use crate::sequencer::state::{get_scheduler, get_sequencer};
use crate::sequencer::types::{
    ERR_LOGIC, ERR_NOT_ACTIVE_SEQUENCER, ERR_SEQUENCER_ROLLAPP_MISMATCH, ERR_UNKNOWN_SEQUENCER,
};

pub fn before_update_state(
    storage: &mut dyn Storage,
    seq_addr: String,
    rollapp_id: String,
) -> StdResult<()> {
    let sequencer = match get_sequencer(storage, seq_addr.clone()) {
        Ok(x) => x,
        Err(_) => return Err(StdError::generic_err(ERR_UNKNOWN_SEQUENCER)),
    };

    if !sequencer.rollapp_i_ds.contains(&rollapp_id) {
        return Err(StdError::generic_err(ERR_SEQUENCER_ROLLAPP_MISMATCH));
    }

    let scheduler = match get_scheduler(storage, seq_addr.clone()) {
        Ok(x) => x,
        Err(_) => {
            return Err(StdError::generic_err(format!(
                "{}: sequencer address: {} not registered in scheduler",
                ERR_LOGIC,
                seq_addr.clone()
            )))
        }
    };
    if scheduler.status != OperatingStatus::Proposer as i32 {
        return Err(StdError::generic_err(ERR_NOT_ACTIVE_SEQUENCER));
    }

    Ok(())
}
