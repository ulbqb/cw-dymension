use cosmwasm_schema::cw_serde;
use cosmwasm_std::{StdResult, Storage};
use cw_storage_plus::Item;

pub const PROCESSING_MAX_NUM: u8 = 20;

// state
const STATE_HEIGHT: &str = "System/MinHeight/";
const MIN_HEIGHT: Item<u64> = Item::new(STATE_HEIGHT);

pub fn get_min_height(storage: &dyn Storage) -> StdResult<u64> {
    MIN_HEIGHT.load(storage)
}

pub fn set_min_height(storage: &mut dyn Storage, height: u64) {
    let _ = MIN_HEIGHT.save(storage, &height);
}

#[cw_serde]
pub struct MsgEndBlocks {
    pub num: Option<u8>,
}
