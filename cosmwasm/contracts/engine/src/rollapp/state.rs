use cosmwasm_std::{Order, StdError, StdResult, Storage};
use cw_storage_plus::{Bound, Item, Map};
use dymension_std::types::cosmos::base::query::v1beta1::{PageRequest, PageResponse};
use dymension_std::types::dymensionxyz::dymension::rollapp::{
    BlockHeightToFinalizationQueue, DeployerParams, Params, Rollapp, StateInfo, StateInfoIndex,
};

use crate::rollapp::types;
use crate::utils::pagination;

// state
const ROLLAPP: Map<Vec<u8>, Rollapp> = Map::new(types::STATE_ROLLAPP);
const ROLLAPP_EIP155: Map<Vec<u8>, Rollapp> = Map::new(types::STATE_ROLLAPP_EIP155);
const STATE_INFO: Map<Vec<u8>, StateInfo> = Map::new(types::STATE_STATE_INFO);
const LATEST_STATE_INFO_INDEX: Map<Vec<u8>, StateInfoIndex> =
    Map::new(types::STATE_LATEST_STATE_INFO_INDEX);
const LATEST_FINALIZED_STATE_INDEX: Map<Vec<u8>, StateInfoIndex> =
    Map::new(types::STATE_LATEST_FINALIZED_STATE_INDEX);
const BLOCK_HEIGHT_TO_FINALIZATION_QUEUE: Map<Vec<u8>, BlockHeightToFinalizationQueue> =
    Map::new(types::STATE_BLOCK_HEIGHT_TO_FINALIZATION_QUEUE);
const PARAMS: Item<Params> = Item::new(types::STATE_PARAMS);

// rollapp
pub fn rollapp_key(rollapp_id: String) -> Vec<u8> {
    let mut key: Vec<u8> = vec![];

    key.append(&mut rollapp_id.into());
    key.append(&mut "/".into());

    key
}

pub fn rollapp_eip155_key(eip155: u64) -> Vec<u8> {
    let mut key: Vec<u8> = vec![];

    key.append(&mut eip155.to_le_bytes().to_vec());
    key.append(&mut "/".into());

    key
}

pub fn set_rollapp(storage: &mut dyn Storage, ra: Rollapp) {
    let _ = ROLLAPP.save(storage, rollapp_key(ra.rollapp_id.clone()), &ra.clone());

    let eip155 = match types::parse_chain_id(ra.clone().rollapp_id) {
        Ok(id) => match id {
            Some(id) => id,
            None => return,
        },
        Err(_) => return,
    };

    let _ = ROLLAPP_EIP155.save(storage, rollapp_eip155_key(eip155), &ra.clone());
}

pub fn get_rollapp(storage: &dyn Storage, rollapp_id: String) -> StdResult<Rollapp> {
    ROLLAPP.load(storage, rollapp_key(rollapp_id))
}

pub fn get_rollapp_eip155(storage: &dyn Storage, eip155: u64) -> StdResult<Rollapp> {
    ROLLAPP_EIP155.load(storage, rollapp_eip155_key(eip155))
}

pub fn get_all_rollapp(storage: &dyn Storage) -> StdResult<Vec<(Vec<u8>, Rollapp)>> {
    ROLLAPP
        .range(storage, None, None, Order::Ascending)
        .collect()
}

pub fn paginate_rollapp(
    storage: &dyn Storage,
    req_opt: Option<PageRequest>,
    on_result: impl FnMut((Vec<u8>, Rollapp)) -> Option<StdError>,
) -> StdResult<PageResponse> {
    pagination::paginate(storage, ROLLAPP, req_opt, on_result)
}

// state info
pub fn state_info_key(rollapp_id: String, index: u64) -> Vec<u8> {
    let mut key: Vec<u8> = vec![];

    key.append(&mut rollapp_id.as_bytes().to_vec());
    key.append(&mut "/".into());
    key.append(&mut index.to_string().as_bytes().to_vec());
    key.append(&mut "/".into());

    key
}

pub fn set_state_info(storage: &mut dyn Storage, si: StateInfo) {
    let _ = STATE_INFO.save(
        storage,
        state_info_key(
            si.clone().state_info_index.unwrap().rollapp_id,
            si.clone().state_info_index.unwrap().index,
        ),
        &si,
    );
}

pub fn get_state_info(
    storage: &dyn Storage,
    rollapp_id: String,
    index: u64,
) -> StdResult<StateInfo> {
    STATE_INFO.load(storage, state_info_key(rollapp_id, index))
}

pub fn get_all_state_info(storage: &dyn Storage) -> StdResult<Vec<(Vec<u8>, StateInfo)>> {
    STATE_INFO
        .range(storage, None, None, Order::Ascending)
        .collect()
}

pub fn paginate_state_info(
    storage: &dyn Storage,
    req_opt: Option<PageRequest>,
    on_result: impl FnMut((Vec<u8>, StateInfo)) -> Option<StdError>,
) -> StdResult<PageResponse> {
    pagination::paginate(storage, STATE_INFO, req_opt, on_result)
}

// latest state info index
pub fn latest_state_info_index_key(rollapp_id: String) -> Vec<u8> {
    let mut key: Vec<u8> = vec![];

    key.append(&mut rollapp_id.as_bytes().to_vec());
    key.append(&mut "/".into());

    key
}

pub fn set_latest_state_info_index(storage: &mut dyn Storage, sii: StateInfoIndex) {
    let _ = LATEST_STATE_INFO_INDEX.save(
        storage,
        latest_state_info_index_key(sii.rollapp_id.clone()),
        &sii,
    );
}

pub fn get_latest_state_info_index(
    storage: &dyn Storage,
    rollapp_id: String,
) -> StdResult<StateInfoIndex> {
    LATEST_STATE_INFO_INDEX.load(storage, latest_state_info_index_key(rollapp_id))
}

pub fn get_all_latest_state_info_index(
    storage: &dyn Storage,
) -> StdResult<Vec<(Vec<u8>, StateInfoIndex)>> {
    LATEST_STATE_INFO_INDEX
        .range(storage, None, None, Order::Ascending)
        .collect()
}

// latest finalized state index
pub fn latest_finalized_state_index_key(rollapp_id: String) -> Vec<u8> {
    let mut key: Vec<u8> = vec![];

    key.append(&mut rollapp_id.as_bytes().to_vec());
    key.append(&mut "/".into());

    key
}

pub fn set_latest_finalized_state_index(storage: &mut dyn Storage, sii: StateInfoIndex) {
    let _ = LATEST_FINALIZED_STATE_INDEX.save(
        storage,
        latest_state_info_index_key(sii.rollapp_id.clone()),
        &sii,
    );
}

pub fn get_latest_finalized_state_index(
    storage: &dyn Storage,
    rollapp_id: String,
) -> StdResult<StateInfoIndex> {
    LATEST_FINALIZED_STATE_INDEX.load(storage, latest_state_info_index_key(rollapp_id))
}

pub fn get_all_latest_finalized_state_index(
    storage: &dyn Storage,
) -> StdResult<Vec<(Vec<u8>, StateInfoIndex)>> {
    LATEST_FINALIZED_STATE_INDEX
        .range(storage, None, None, Order::Ascending)
        .collect()
}

// block height to finalization queue
pub fn block_height_to_finalization_queue_key(finalization_height: u64) -> Vec<u8> {
    let mut key: Vec<u8> = vec![];

    key.append(&mut finalization_height.to_be_bytes().to_vec());
    key.append(&mut "/".into());

    key
}

pub fn set_block_height_to_finalization_queue(
    storage: &mut dyn Storage,
    bh2fq: BlockHeightToFinalizationQueue,
) {
    let _ = BLOCK_HEIGHT_TO_FINALIZATION_QUEUE.save(
        storage,
        block_height_to_finalization_queue_key(bh2fq.finalization_height.clone()),
        &bh2fq,
    );
}

pub fn get_block_height_to_finalization_queue(
    storage: &dyn Storage,
    finalization_height: u64,
) -> StdResult<BlockHeightToFinalizationQueue> {
    BLOCK_HEIGHT_TO_FINALIZATION_QUEUE.load(
        storage,
        block_height_to_finalization_queue_key(finalization_height),
    )
}

pub fn get_block_height_to_finalization_queue_range(
    storage: &dyn Storage,
    min_height: u64,
    max_height: u64,
    limit: u8,
) -> StdResult<Vec<(Vec<u8>, BlockHeightToFinalizationQueue)>> {
    let min = Some(Bound::ExclusiveRaw(block_height_to_finalization_queue_key(
        min_height,
    )));
    let max = Some(Bound::ExclusiveRaw(block_height_to_finalization_queue_key(
        max_height,
    )));
    BLOCK_HEIGHT_TO_FINALIZATION_QUEUE
        .range(storage, min, max, Order::Ascending)
        .take(limit as usize)
        .collect()
}

pub fn get_all_block_height_to_finalization_queue(
    storage: &dyn Storage,
) -> StdResult<Vec<(Vec<u8>, BlockHeightToFinalizationQueue)>> {
    BLOCK_HEIGHT_TO_FINALIZATION_QUEUE
        .range(storage, None, None, Order::Ascending)
        .collect()
}

// params
pub fn set_params(storage: &mut dyn Storage, params: Params) {
    let _ = PARAMS.save(storage, &params);
}

pub fn get_params(storage: &dyn Storage) -> StdResult<Params> {
    PARAMS.load(storage)
}

pub fn rollapp_enable(storage: &dyn Storage) -> StdResult<bool> {
    let params = PARAMS.load(storage)?;
    Ok(params.rollapps_enabled)
}

pub fn deployer_whitelist(storage: &dyn Storage) -> StdResult<Vec<DeployerParams>> {
    let params = PARAMS.load(storage)?;
    Ok(params.deployer_whitelist)
}

pub fn dispute_period_in_blocks(storage: &dyn Storage) -> StdResult<u64> {
    let params = PARAMS.load(storage)?;
    Ok(params.dispute_period_in_blocks)
}
