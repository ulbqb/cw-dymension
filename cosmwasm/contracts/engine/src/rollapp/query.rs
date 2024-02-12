use cosmwasm_std::{StdError, StdResult, Storage};
use dymension_std::types::dymensionxyz::dymension::rollapp::{
    QueryAllRollappRequest, QueryAllRollappResponse, QueryAllStateInfoRequest,
    QueryAllStateInfoResponse, QueryGetLatestStateIndexRequest, QueryGetLatestStateIndexResponse,
    QueryGetRollappByEip155Request, QueryGetRollappRequest, QueryGetRollappResponse,
    QueryGetStateInfoRequest, QueryGetStateInfoResponse, QueryParamsResponse, RollappSummary,
    StateInfo, StateInfoIndex, StateInfoSummary,
};

use crate::rollapp::{state::*, types::*};

pub fn params(storage: &dyn Storage) -> StdResult<QueryParamsResponse> {
    Ok(QueryParamsResponse {
        params: Some(get_params(storage)?),
    })
}

pub fn rollapp(
    storage: &dyn Storage,
    req: QueryGetRollappRequest,
) -> StdResult<QueryGetRollappResponse> {
    let ra = get_rollapp(storage, req.rollapp_id.clone()).ok();
    if ra.is_none() {
        return Err(StdError::not_found("rollapp"));
    }
    let lsii = get_latest_state_info_index(storage, req.rollapp_id.clone()).ok();
    let lfsi = get_latest_finalized_state_index(storage, req.rollapp_id.clone()).ok();
    Ok(QueryGetRollappResponse {
        rollapp: ra,
        latest_state_index: lsii,
        latest_finalized_state_index: lfsi,
    })
}

pub fn rollapp_eip155(
    storage: &dyn Storage,
    req: QueryGetRollappByEip155Request,
) -> StdResult<QueryGetRollappResponse> {
    let ra = match get_rollapp_eip155(storage, req.eip155.clone()) {
        Ok(x) => x,
        Err(_) => return Err(StdError::not_found("rollapp by eip155")),
    };
    let lsii = get_latest_state_info_index(storage, ra.rollapp_id.clone()).ok();
    let lfsi = get_latest_finalized_state_index(storage, ra.rollapp_id.clone()).ok();
    Ok(QueryGetRollappResponse {
        rollapp: Some(ra),
        latest_state_index: lsii,
        latest_finalized_state_index: lfsi,
    })
}

pub fn rollapp_all(
    storage: &dyn Storage,
    req: QueryAllRollappRequest,
) -> StdResult<QueryAllRollappResponse> {
    let mut summary: Vec<RollappSummary> = vec![];
    let res = paginate_rollapp(storage, req.pagination, |kv| -> Option<StdError> {
        let lsii = get_latest_state_info_index(storage, kv.1.rollapp_id.clone()).ok();
        let lfsi = get_latest_finalized_state_index(storage, kv.1.rollapp_id.clone()).ok();
        summary.push(RollappSummary {
            rollapp_id: kv.1.rollapp_id,
            latest_state_index: lsii,
            latest_finalized_state_index: lfsi,
        });
        None
    })
    .ok();
    Ok(QueryAllRollappResponse {
        rollapp: summary,
        pagination: res,
    })
}

pub fn latest_state_index(
    storage: &dyn Storage,
    req: QueryGetLatestStateIndexRequest,
) -> StdResult<QueryGetLatestStateIndexResponse> {
    let val: Option<StateInfoIndex>;
    if req.finalized {
        val = get_latest_finalized_state_index(storage, req.rollapp_id).ok();
    } else {
        val = get_latest_state_info_index(storage, req.rollapp_id).ok();
    }

    if val.is_none() {
        return Err(StdError::not_found("state info index"));
    }

    Ok(QueryGetLatestStateIndexResponse { state_index: val })
}

pub fn state_info(
    storage: &dyn Storage,
    req: QueryGetStateInfoRequest,
) -> StdResult<QueryGetStateInfoResponse> {
    let mut index: u64 = req.index.clone();
    if req.height == 0 && index == 0 {
        if req.finalized {
            let lfsi = get_latest_finalized_state_index(storage, req.rollapp_id.clone());
            index = match lfsi {
                Ok(x) => x.index,
                Err(_) => {
                    return Err(StdError::generic_err(format!(
                        "{}: LatestFinalizedStateIndex wasn't found for rollappId={}",
                        ERR_LOGIC, req.rollapp_id,
                    )))
                }
            };
        } else {
            let lsi = get_latest_state_info_index(storage, req.rollapp_id.clone());
            index = match lsi {
                Ok(x) => x.index,
                Err(_) => {
                    return Err(StdError::generic_err(format!(
                        "{}: LatestStateInfoIndex wasn't found for rollappId={}",
                        ERR_LOGIC, req.rollapp_id,
                    )))
                }
            };
        }
    }

    let mut state_info = Some(StateInfo::default());
    if index != 0 {
        state_info = get_state_info(storage, req.rollapp_id.clone(), req.index).ok();
        if state_info.is_none() {
            return Err(StdError::not_found("state info"));
        }
    } else if req.height != 0 {
        state_info = Some(find_state_info_by_height(
            storage,
            req.rollapp_id.clone(),
            req.height,
        )?);
    }

    Ok(QueryGetStateInfoResponse {
        state_info: state_info,
    })
}

pub fn state_info_all(
    storage: &dyn Storage,
    req: QueryAllStateInfoRequest,
) -> StdResult<QueryAllStateInfoResponse> {
    let mut summary: Vec<StateInfoSummary> = vec![];
    let res = paginate_state_info(storage, req.pagination, |kv| -> Option<StdError> {
        if kv.1.state_info_index.clone()?.rollapp_id == req.rollapp_id {
            summary.push(StateInfoSummary {
                state_info_index: kv.1.state_info_index.clone(),
                status: kv.1.status.clone(),
                creation_height: kv.1.creation_height.clone(),
            });
        }
        None
    })
    .ok();
    Ok(QueryAllStateInfoResponse {
        state_info: summary,
        pagination: res,
    })
}

// helper
fn find_state_info_by_height(
    storage: &dyn Storage,
    rollapp_id: String,
    height: u64,
) -> StdResult<StateInfo> {
    if height == 0 {
        return Err(StdError::generic_err(ERR_INVALID_HEIGHT));
    }

    if get_rollapp(storage, rollapp_id.clone()).is_err() {
        return Err(StdError::generic_err(ERR_UNKNOWN_ROLLAPP_ID));
    };

    let state_info_index = match get_latest_state_info_index(storage, rollapp_id.clone()) {
        Ok(x) => x,
        Err(_) => {
            return Err(StdError::generic_err(format!(
                "{}: LatestStateInfoIndex wasn't found for rollappId={}",
                ERR_LOGIC, rollapp_id,
            )))
        }
    };
    let mut start_info_index: u64 = 1;
    let mut end_info_index = state_info_index.index;

    let latest_state_info = match get_state_info(storage, rollapp_id.clone(), end_info_index) {
        Ok(x) => x,
        Err(_) => {
            return Err(StdError::generic_err(format!(
                "{}: StateInfo wasn't found for rollappId={}, index={}",
                ERR_LOGIC, rollapp_id, end_info_index
            )))
        }
    };

    if height > latest_state_info.start_height + latest_state_info.num_blocks {
        return Err(StdError::generic_err(format!(
            "{}: rollappId={}, height={}",
            ERR_STATE_NOT_EXISTS, rollapp_id, height
        )));
    }

    if height >= latest_state_info.start_height {
        return Ok(latest_state_info);
    }

    let max_number_of_steps = end_info_index - start_info_index + 1;
    let mut step_num: u64 = 0;
    while step_num < max_number_of_steps {
        if end_info_index <= start_info_index {
            return Err(StdError::generic_err(format!(
                "{}: endInfoIndex should be != than startInfoIndex rollappId={}, startInfoIndex={}, endInfoIndex={}",
                ERR_LOGIC, rollapp_id, start_info_index, end_info_index
            )));
        }

        // 1
        let start_state_info = match get_state_info(storage, rollapp_id.clone(), start_info_index) {
            Ok(x) => x,
            Err(_) => {
                return Err(StdError::generic_err(format!(
                    "{}: StateInfo wasn't found for rollappId={}, index={}",
                    ERR_LOGIC, rollapp_id, start_info_index
                )))
            }
        };

        let end_state_info = match get_state_info(storage, rollapp_id.clone(), end_info_index) {
            Ok(x) => x,
            Err(_) => {
                return Err(StdError::generic_err(format!(
                    "{}: StateInfo wasn't found for rollappId={}, index={}",
                    ERR_LOGIC, rollapp_id, end_info_index
                )))
            }
        };

        let start_height = start_state_info.start_height;
        let end_height = end_state_info.start_height + end_state_info.num_blocks - 1;

        // 2
        if height >= start_state_info.start_height
            && (start_state_info.start_height + start_state_info.num_blocks) > height
        {
            return Ok(end_state_info);
        }

        // 3
        if height >= end_state_info.start_height
            && (end_state_info.start_height + end_state_info.num_blocks) > height
        {
            return Ok(end_state_info);
        }

        // 4
        let avg_blocks_per_batch =
            (end_height - start_height + 1) / (end_info_index - start_info_index + 1);
        if avg_blocks_per_batch == 0 {
            return Err(StdError::generic_err(format!(
                "{}: avgBlocksPerBatch is zero!!! rollappId={}, endHeight={}, startHeight={}, endInfoIndex={}, startInfoIndex={}",
                ERR_LOGIC, rollapp_id, end_height, start_height, end_info_index ,start_info_index,
            )));
        }

        // 5
        let mut info_index_step = (height - start_height) / avg_blocks_per_batch;
        if info_index_step == 0 {
            info_index_step = 1;
        }
        let mut candidate_info_index = start_info_index + info_index_step;
        if candidate_info_index > end_info_index {
            candidate_info_index = end_info_index;
        }
        if candidate_info_index == end_info_index {
            candidate_info_index = end_info_index - 1;
        }
        let candidate_state_info =
            match get_state_info(storage, rollapp_id.clone(), candidate_info_index) {
                Ok(x) => x,
                Err(_) => {
                    return Err(StdError::generic_err(format!(
                        "{}: StateInfo wasn't found for rollappId={}, index={}",
                        ERR_LOGIC, rollapp_id, candidate_info_index
                    )))
                }
            };

        // 6
        if candidate_state_info.start_height > height {
            end_info_index = candidate_info_index - 1;
        } else {
            if candidate_state_info.start_height + candidate_state_info.num_blocks - 1 < height {
                start_info_index = candidate_info_index + 1;
            } else {
                return Ok(candidate_state_info);
            }
        }
        step_num += 1;
    }

    Err(StdError::generic_err(format!(
        "{}: More searching steps than indexes! rollappId={}, stepNum={}, maxNumberOfSteps={}",
        ERR_LOGIC, rollapp_id, step_num, max_number_of_steps
    )))
}
