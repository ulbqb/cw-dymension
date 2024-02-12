use crate::error::ContractError;
use regex::Regex;

// state
pub const STATE_ROLLAPP: &str = "Rollapp/Rollapp/";
pub const STATE_ROLLAPP_EIP155: &str = "Rollapp/RollappByEIP155/";
pub const STATE_STATE_INFO: &str = "Rollapp/StateInfo/";
pub const STATE_LATEST_STATE_INFO_INDEX: &str = "Rollapp/LatestStateInfoIndex/";
pub const STATE_LATEST_FINALIZED_STATE_INDEX: &str = "Rollapp/LatestFinalizedStateIndex/";
pub const STATE_BLOCK_HEIGHT_TO_FINALIZATION_QUEUE: &str =
    "Rollapp/BlockHeightToFinalizationQueue/";
pub const STATE_PARAMS: &str = "Rollapp/Params/";

// regex
const REGEX_CHAIN_ID: &str = "^([a-z]{1,})_{1}([1-9][0-9]*)-{1}([1-9][0-9]*)$";

// event
pub const EVENT_TYPE_STATE_UPDATE: &str = "state_update";
pub const EVENT_TYPE_STATUS_CHANGE: &str = "status_change";
pub const ATTRIBUTE_KEY_ROLLAPP_ID: &str = "rollapp_id";
pub const ATTRIBUTE_KEY_STATE_INFO_INDEX: &str = "state_info_index";
pub const ATTRIBUTE_KEY_START_HEIGHT: &str = "start_height";
pub const ATTRIBUTE_KEY_NUM_BLOCKS: &str = "num_blocks";
pub const ATTRIBUTE_KEY_DA_PATH: &str = "da_path";
pub const ATTRIBUTE_KEY_STATUS: &str = "status";

// error
pub const ERR_LOGIC: &str = "internal logic error";

pub const ERR_ROLLAPP_EXISTS: &str =
    "rollapp already exist for this rollapp-id; must use new rollapp-id";
pub const ERR_INVALID_MAX_SEQUENCERS: &str = "invalid max sequencers";
pub const ERR_INVALID_PERMISSIONED_ADDRESS: &str = "invalid permissioned address";
pub const ERR_PERMISSIONED_ADDRESSES_DUPLICATE: &str = "permissioned-address has duplicates";
pub const ERR_INVALID_NUM_BLOCKS: &str = "invalid number of blocks";
pub const ERR_INVALID_BLOCK_SEQUENCE: &str = "invalid block sequence";
pub const ERR_UNKNOWN_ROLLAPP_ID: &str = "rollapp does not exist";
pub const ERR_VERSION_MISMATCH: &str = "rollapp version mismatch";
pub const ERR_WRONG_BLOCK_HEIGHT: &str = "start-height does not match rollapps state";
pub const ERR_MULTI_UPDATE_STATE_IN_BLOCK: &str = "only one state update can take place per block";
pub const ERR_INVALID_STATE_ROOT: &str = "invalid blocks state root";
pub const ERR_INVALID_INTERMEDIATE_STATES_ROOT: &str = "invalid blocks intermediate states root";
pub const ERR_UNAUTHORIZED_ROLLAPP_CREATOR: &str = "rollapp creator not register in whitelist";
pub const ERR_INVALID_CLIENT_TYPE: &str = "client type of the rollapp isn't dymint";
pub const ERR_HEIGHT_STATE_NOT_FAINALIZED: &str =
    "rollapp block on this height was not finalized yet";
pub const ERR_INVALID_APP_HASH: &str = "the app hash is different from the finalized state root";
pub const ERR_STATE_NOT_EXISTS: &str = "state of this height doesn't exist";
pub const ERR_INVALID_HEIGHT: &str = "invalid rollapp height";
pub const ERR_ROLLAPP_CREATOR_EXCEED_MAXIMUM_ROLLAPPS: &str =
    "rollapp creator exceed maximum allowed rollapps as register in whitelist";
pub const RR_INVALID_ROLLAPP_ID: &str = "invalid rollapp-id";
pub const ERR_EIP155_EXISTS: &str = "EIP155 already exist; must use unique EIP155 identifier";
pub const ERR_ROLLAPPS_DISABLED: &str = "rollapps are disabled";

pub fn parse_chain_id(mut chain_id: String) -> Result<Option<u64>, ContractError> {
    chain_id.retain(|c| !c.is_whitespace());

    if chain_id.len() == 0 {
        return Err(ContractError::CustomError {
            val: "chain-id cannot be empty".into(),
        });
    }

    if chain_id.len() > 48 {
        return Err(ContractError::CustomError {
            val: format!("rollapp-id '{chain_id}' cannot exceed 48 chars"),
        });
    }

    let ethermint_chain_id = Regex::new(REGEX_CHAIN_ID).unwrap();
    let matches = ethermint_chain_id.captures(chain_id.as_str());

    let cap = match matches {
        // The division was valid
        Some(c) => {
            if c.len() != 4 || c[1].len() == 0 {
                return Ok(None);
            }
            c
        }
        // The division was invalid
        None => return Ok(None),
    };

    let chain_id_int = cap[2].parse::<u64>();
    match chain_id_int {
        Ok(id) => return Ok(Some(id)),
        Err(_) => {
            return Err(ContractError::CustomError {
                val: format!(
                    "epoch {} must be base-10 integer format",
                    cap[2].to_string()
                ),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_chain_id() {
        let result = parse_chain_id("dymension_100-1".to_string()).unwrap();
        assert_eq!(Some(100), result)
    }
}
