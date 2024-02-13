use cosmwasm_std::{Response, StdError, Storage};
use dymension_std::types::dymensionxyz::dymension::sequencer::{
    MsgCreateSequencer, OperatingStatus, Scheduler, Sequencer, Sequencers, SequencersByRollapp,
};

use crate::error::ContractError;
use crate::rollapp::state::get_rollapp;
use crate::sequencer::state::{
    get_sequencer, get_sequencers_by_rollapp, set_scheduler, set_sequencer,
    set_sequencers_by_rollapp,
};
use crate::sequencer::types::{
    validate_description, ERR_INVALID_PUB_KEY, ERR_LOGIC, ERR_MAX_SEQUENCERS_LIMIT,
    ERR_SEQUENCER_ALREADY_REGISTERED, ERR_SEQUENCER_NOT_PERMISSIONED, ERR_UNKNOWN_ROLLAPP_ID,
};

pub fn create_sequencer(
    storage: &mut dyn Storage,
    msg: MsgCreateSequencer,
) -> Result<Response, ContractError> {
    if msg.dymint_pub_key.is_none() {
        return Err(StdError::generic_err(format!(
            "{}: sequencer pubkey can not be empty",
            ERR_INVALID_PUB_KEY
        ))
        .into());
    }

    let rollapp = match get_rollapp(storage, msg.rollapp_id.clone()) {
        Ok(x) => x,
        Err(_) => return Err(StdError::generic_err(ERR_UNKNOWN_ROLLAPP_ID).into()),
    };

    if !rollapp
        .permissioned_addresses
        .contains(&msg.creator.clone())
    {
        return Err(StdError::generic_err(ERR_SEQUENCER_NOT_PERMISSIONED).into());
    }

    let sequencer = match get_sequencer(storage, msg.creator.clone()) {
        Ok(mut x) => {
            if !x.dymint_pub_key.eq(&msg.dymint_pub_key) {
                return Err(StdError::generic_err(format!(
                    "{}: sequencer pubkey does not match",
                    ERR_INVALID_PUB_KEY
                ))
                .into());
            }

            if x.rollapp_i_ds.contains(&msg.rollapp_id) {
                return Err(StdError::generic_err(ERR_SEQUENCER_ALREADY_REGISTERED).into());
            }
            x.rollapp_i_ds.push(msg.rollapp_id.clone());
            x
        }
        Err(_) => {
            let new_seq = Sequencer {
                sequencer_address: msg.creator.clone(),
                dymint_pub_key: msg.dymint_pub_key,
                rollapp_i_ds: vec![msg.rollapp_id.clone()],
                description: msg.description.clone(),
            };
            set_sequencer(storage, new_seq.clone());
            new_seq
        }
    };

    let sequencers_by_rollapp = match get_sequencers_by_rollapp(storage, msg.rollapp_id.clone()) {
        Ok(mut x) => {
            let max_sequencers = rollapp.max_sequencers;
            let active_sequencers = x.sequencers.clone();
            let current_num_sequencers = match active_sequencers {
                Some(x) => x.addresses.len() as u64,
                None => 0,
            };
            if max_sequencers < current_num_sequencers {
                return Err(StdError::generic_err(format!(
                    "{}: rollapp id: {} cannot have more than {} sequencers but got: {}",
                    ERR_LOGIC, msg.rollapp_id, max_sequencers, current_num_sequencers
                ))
                .into());
            }
            if max_sequencers == current_num_sequencers {
                return Err(StdError::generic_err(ERR_MAX_SEQUENCERS_LIMIT).into());
            }

            x.sequencers = match x.sequencers.clone() {
                Some(mut y) => {
                    y.addresses.push(sequencer.sequencer_address);
                    Some(y)
                }
                None => Some(Sequencers {
                    addresses: vec![sequencer.sequencer_address],
                }),
            };

            set_scheduler(
                storage,
                Scheduler {
                    sequencer_address: msg.creator.clone(),
                    status: OperatingStatus::Inactive.into(),
                },
            );
            x
        }
        Err(_) => {
            set_scheduler(
                storage,
                Scheduler {
                    sequencer_address: msg.creator.clone(),
                    status: OperatingStatus::Proposer.into(),
                },
            );
            SequencersByRollapp {
                rollapp_id: msg.rollapp_id,
                sequencers: Some(Sequencers {
                    addresses: vec![msg.creator],
                }),
            }
        }
    };
    set_sequencers_by_rollapp(storage, sequencers_by_rollapp);

    if let Some(desc) = msg.description {
        if let Some(err) = validate_description(desc) {
            return Err(err.into());
        }
    };

    Ok(Response::new().add_attribute("method", "create_sequencer"))
}
