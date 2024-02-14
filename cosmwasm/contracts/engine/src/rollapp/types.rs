use crate::error::ContractError;

// state
pub const STATE_ROLLAPP: &str = "Rollapp/Rollapp/";
pub const STATE_ROLLAPP_EIP155: &str = "Rollapp/RollappByEIP155/";
pub const STATE_STATE_INFO: &str = "Rollapp/StateInfo/";
pub const STATE_LATEST_STATE_INFO_INDEX: &str = "Rollapp/LatestStateInfoIndex/";
pub const STATE_LATEST_FINALIZED_STATE_INDEX: &str = "Rollapp/LatestFinalizedStateIndex/";
pub const STATE_BLOCK_HEIGHT_TO_FINALIZATION_QUEUE: &str =
    "Rollapp/BlockHeightToFinalizationQueue/";
pub const STATE_PARAMS: &str = "Rollapp/Params/";

// chain id format
const CHAIN_ID_FORMAT_A_Z: &str = "abcdefghijklmnopqrstuvwxyz";
const CHAIN_ID_FORMAT_0_9: &str = "0123456789";
const CHAIN_ID_FORMAT_1_9: &str = "123456789";

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

    let matches = capture_chain_id(chain_id.clone());

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

// The regex package is so large that it needs to have a dedicated capture function
fn capture_chain_id(chain_id: String) -> Option<Vec<String>> {
    let mut cap: Vec<String> = vec![chain_id.clone()];
    let mut buf: String = "".into();
    for i in 0..chain_id.len() {
        let char: String = chain_id.chars().skip(i).take(1).collect();
        if cap.len() == 1 {
            if char == "_" {
                cap.push(buf.to_string());
                buf = "".into();
                continue;
            }
            if !CHAIN_ID_FORMAT_A_Z.contains(char.as_str()) {
                return None;
            }
            buf.push_str(char.as_str());
        }

        if cap.len() == 2 {
            if char == "-" {
                cap.push(buf.to_string());
                buf = "".into();
                continue;
            }
            if buf.len() == 0 {
                if !CHAIN_ID_FORMAT_1_9.contains(char.as_str()) {
                    return None;
                }
            } else {
                if !CHAIN_ID_FORMAT_0_9.contains(char.as_str()) {
                    return None;
                }
            }
            buf.push_str(char.as_str());
        }

        if cap.len() == 3 {
            if buf.len() == 0 {
                if !CHAIN_ID_FORMAT_1_9.contains(char.as_str()) {
                    return None;
                }
            } else {
                if !CHAIN_ID_FORMAT_0_9.contains(char.as_str()) {
                    return None;
                }
            }
            buf.push_str(char.as_str());
            if i + 1 == chain_id.len() {
                cap.push(buf.to_string());
            }
        }
    }
    if cap.len() == 1 {
        return None;
    }
    Some(cap)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_chain_id() {
        let result = parse_chain_id("dymension_100-1".to_string()).unwrap();
        assert_eq!(Some(100), result)
    }

    #[test]
    fn test_capture_chain_id() {
        let cap = capture_chain_id("dymension_100-1".into()).unwrap();
        assert_eq!(4, cap.len());
        assert_eq!("dymension_100-1", cap[0]);
        assert_eq!("dymension", cap[1]);
        assert_eq!("100", cap[2]);
        assert_eq!("1", cap[3]);

        let cap = capture_chain_id("dymension@100-1".into());
        assert_eq!(None, cap);

        let cap = capture_chain_id("dymension_100@1".into());
        assert_eq!(None, cap);

        let cap = capture_chain_id("dymension@100@1".into());
        assert_eq!(None, cap);

        let cap = capture_chain_id("dymension_000-1".into());
        assert_eq!(None, cap);

        let cap = capture_chain_id("dymension_100-0".into());
        assert_eq!(None, cap);

        let cap = capture_chain_id("dymension_000-0".into());
        assert_eq!(None, cap);
    }
}
