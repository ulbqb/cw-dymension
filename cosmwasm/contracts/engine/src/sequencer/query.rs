use cosmwasm_std::{StdError, StdResult, Storage};
use dymension_std::types::dymensionxyz::dymension::sequencer::{
    QueryAllSchedulerRequest, QueryAllSchedulerResponse, QueryAllSequencerRequest,
    QueryAllSequencerResponse, QueryAllSequencersByRollappRequest,
    QueryAllSequencersByRollappResponse, QueryGetSchedulerRequest, QueryGetSchedulerResponse,
    QueryGetSequencerRequest, QueryGetSequencerResponse, QueryGetSequencersByRollappRequest,
    QueryGetSequencersByRollappResponse, QueryParamsResponse, Scheduler, SequencerInfo,
};

use crate::sequencer::state::{
    get_params, get_scheduler, get_sequencer, get_sequencers_by_rollapp, paginate_scheduler,
    paginate_sequencer, paginate_sequencers_by_rollapp,
};
use crate::sequencer::types::ERR_LOGIC;

pub fn params(storage: &dyn Storage) -> StdResult<QueryParamsResponse> {
    Ok(QueryParamsResponse {
        params: Some(get_params(storage)?),
    })
}

// sequencer
pub fn sequencer(
    storage: &dyn Storage,
    req: QueryGetSequencerRequest,
) -> StdResult<QueryGetSequencerResponse> {
    Ok(QueryGetSequencerResponse {
        sequencer_info: Some(get_sequencer_info(storage, req.sequencer_address)?),
    })
}

pub fn sequencer_all(
    storage: &dyn Storage,
    req: QueryAllSequencerRequest,
) -> StdResult<QueryAllSequencerResponse> {
    let mut sequencer_info_list: Vec<SequencerInfo> = vec![];
    let res = paginate_sequencer(storage, req.pagination, |kv| -> Option<StdError> {
        let seq = kv.1;
        let sch = match get_scheduler(storage, seq.sequencer_address.clone()) {
            Ok(x) => x,
            Err(_) => {
                return Some(StdError::generic_err(format!(
                    "{}: scheduler was not found for sequencer {}",
                    ERR_LOGIC, seq.sequencer_address,
                )))
            }
        };
        sequencer_info_list.push(SequencerInfo {
            sequencer: Some(seq),
            status: sch.status,
        });
        None
    })
    .ok();

    Ok(QueryAllSequencerResponse {
        sequencer_info_list: sequencer_info_list,
        pagination: res,
    })
}

// sequencers by rollapp
pub fn sequencers_by_rollapp(
    storage: &dyn Storage,
    req: QueryGetSequencersByRollappRequest,
) -> StdResult<QueryGetSequencersByRollappResponse> {
    let val = match get_sequencers_by_rollapp(storage, req.rollapp_id.clone()) {
        Ok(x) => x,
        Err(_) => return Err(StdError::not_found("sequencers by rollapp")),
    };

    match val.sequencers {
        Some(x) => {
            let mut list: Vec<SequencerInfo> = vec![];
            for addr in x.addresses.into_iter() {
                list.push(get_sequencer_info(storage, addr)?)
            }
            Ok(QueryGetSequencersByRollappResponse {
                rollapp_id: req.rollapp_id.clone(),
                sequencer_info_list: list,
            })
        }
        None => Ok(QueryGetSequencersByRollappResponse {
            rollapp_id: req.rollapp_id,
            sequencer_info_list: vec![],
        }),
    }
}

pub fn sequencers_by_rollapp_all(
    storage: &dyn Storage,
    req: QueryAllSequencersByRollappRequest,
) -> StdResult<QueryAllSequencersByRollappResponse> {
    let mut sequencers_by_rollapp_list: Vec<QueryGetSequencersByRollappResponse> = vec![];
    let res = paginate_sequencers_by_rollapp(storage, req.pagination, |kv| -> Option<StdError> {
        let sequencers = match sequencers_by_rollapp(
            storage,
            QueryGetSequencersByRollappRequest {
                rollapp_id: kv.1.rollapp_id,
            },
        ) {
            Ok(x) => x,
            Err(e) => return Some(e),
        };
        sequencers_by_rollapp_list.push(sequencers);
        None
    })
    .ok();

    Ok(QueryAllSequencersByRollappResponse {
        sequencers_by_rollapp: sequencers_by_rollapp_list,
        pagination: res,
    })
}

// scheduler
pub fn scheduler(
    storage: &dyn Storage,
    req: QueryGetSchedulerRequest,
) -> StdResult<QueryGetSchedulerResponse> {
    Ok(QueryGetSchedulerResponse {
        scheduler: Some(get_scheduler(storage, req.sequencer_address)?),
    })
}

pub fn scheduler_all(
    storage: &dyn Storage,
    req: QueryAllSchedulerRequest,
) -> StdResult<QueryAllSchedulerResponse> {
    let mut schedulers: Vec<Scheduler> = vec![];
    let res = paginate_scheduler(storage, req.pagination, |kv| -> Option<StdError> {
        schedulers.push(kv.1);
        None
    })
    .ok();

    Ok(QueryAllSchedulerResponse {
        scheduler: schedulers,
        pagination: res,
    })
}

// helper
fn get_sequencer_info(
    storage: &dyn Storage,
    sequencer_address: String,
) -> StdResult<SequencerInfo> {
    let val = match get_sequencer(storage, sequencer_address.clone()) {
        Ok(x) => x,
        Err(_) => return Err(StdError::not_found("sequencer")),
    };

    let scheduler = match get_scheduler(storage, sequencer_address.clone()) {
        Ok(x) => x,
        Err(_) => {
            return Err(StdError::generic_err(format!(
                "{}: scheduler was not found for sequencer {}",
                ERR_LOGIC, sequencer_address
            )))
        }
    };

    Ok(SequencerInfo {
        sequencer: Some(val),
        status: scheduler.status,
    })
}
