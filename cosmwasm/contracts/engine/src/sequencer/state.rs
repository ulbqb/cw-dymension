use cosmwasm_std::{Order, StdError, StdResult, Storage};
use cw_storage_plus::{Item, Map};
use dymension_std::types::cosmos::base::query::v1beta1::{PageRequest, PageResponse};
use dymension_std::types::dymensionxyz::dymension::sequencer::{
    Params, Scheduler, Sequencer, SequencersByRollapp,
};

use crate::sequencer::types;
use crate::utils::pagination;

// state
const SEQUENCER: Map<Vec<u8>, Sequencer> = Map::new(types::STATE_SEQUENCER);
const SEQUENCERS_BY_ROLLAPP: Map<Vec<u8>, SequencersByRollapp> =
    Map::new(types::STATE_SEQUENCERS_BY_ROLLAPP);
const SCHEDULER: Map<Vec<u8>, Scheduler> = Map::new(types::STATE_SCHEDULER);
const PARAMS: Item<Params> = Item::new(types::STATE_PARAMS);

// sequencer
pub fn sequencer_key(sequencer_address: String) -> Vec<u8> {
    let mut key: Vec<u8> = vec![];

    key.append(&mut sequencer_address.into());
    key.append(&mut "/".into());

    key
}

pub fn set_sequencer(storage: &mut dyn Storage, sequencer: Sequencer) {
    let _ = SEQUENCER.save(
        storage,
        sequencer_key(sequencer.sequencer_address.clone()),
        &sequencer,
    );
}

pub fn get_sequencer(storage: &dyn Storage, sequencer_address: String) -> StdResult<Sequencer> {
    SEQUENCER.load(storage, sequencer_key(sequencer_address))
}

pub fn paginate_sequencer(
    storage: &dyn Storage,
    req_opt: Option<PageRequest>,
    on_result: impl FnMut((Vec<u8>, Sequencer)) -> Option<StdError>,
) -> StdResult<PageResponse> {
    pagination::paginate(storage, SEQUENCER, req_opt, on_result)
}

pub fn get_all_sequencer(storage: &dyn Storage) -> StdResult<Vec<(Vec<u8>, Sequencer)>> {
    SEQUENCER
        .range(storage, None, None, Order::Ascending)
        .collect()
}

// sequencers by rollapp
pub fn sequencers_by_rollapp_key(rollapp_id: String) -> Vec<u8> {
    let mut key: Vec<u8> = vec![];

    key.append(&mut rollapp_id.into());
    key.append(&mut "/".into());

    key
}

pub fn set_sequencers_by_rollapp(storage: &mut dyn Storage, sequencers: SequencersByRollapp) {
    let _ = SEQUENCERS_BY_ROLLAPP.save(
        storage,
        sequencers_by_rollapp_key(sequencers.rollapp_id.clone()),
        &sequencers,
    );
}

pub fn get_sequencers_by_rollapp(
    storage: &dyn Storage,
    rollapp_id: String,
) -> StdResult<SequencersByRollapp> {
    SEQUENCERS_BY_ROLLAPP.load(storage, sequencers_by_rollapp_key(rollapp_id))
}

pub fn paginate_sequencers_by_rollapp(
    storage: &dyn Storage,
    req_opt: Option<PageRequest>,
    on_result: impl FnMut((Vec<u8>, SequencersByRollapp)) -> Option<StdError>,
) -> StdResult<PageResponse> {
    pagination::paginate(storage, SEQUENCERS_BY_ROLLAPP, req_opt, on_result)
}

pub fn get_all_sequencers_by_rollapp(
    storage: &dyn Storage,
) -> StdResult<Vec<(Vec<u8>, SequencersByRollapp)>> {
    SEQUENCERS_BY_ROLLAPP
        .range(storage, None, None, Order::Ascending)
        .collect()
}

// scheduler
pub fn scheduler_key(sequencer_address: String) -> Vec<u8> {
    let mut key: Vec<u8> = vec![];

    key.append(&mut sequencer_address.into());
    key.append(&mut "/".into());

    key
}

pub fn set_scheduler(storage: &mut dyn Storage, scheduler: Scheduler) {
    let _ = SCHEDULER.save(
        storage,
        scheduler_key(scheduler.sequencer_address.clone()),
        &scheduler,
    );
}

pub fn get_scheduler(storage: &dyn Storage, sequencer_address: String) -> StdResult<Scheduler> {
    SCHEDULER.load(storage, scheduler_key(sequencer_address))
}

pub fn paginate_scheduler(
    storage: &dyn Storage,
    req_opt: Option<PageRequest>,
    on_result: impl FnMut((Vec<u8>, Scheduler)) -> Option<StdError>,
) -> StdResult<PageResponse> {
    pagination::paginate(storage, SCHEDULER, req_opt, on_result)
}

pub fn get_all_scheduler(storage: &dyn Storage) -> StdResult<Vec<(Vec<u8>, Scheduler)>> {
    SCHEDULER
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
