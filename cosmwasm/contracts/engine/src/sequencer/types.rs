use cosmwasm_std::StdError;
use dymension_std::types::dymensionxyz::dymension::sequencer::Description;

// state
pub const STATE_SEQUENCER: &str = "Sequencer/Sequencer/";
pub const STATE_SEQUENCERS_BY_ROLLAPP: &str = "Sequencer/SequencersByRollapp";
pub const STATE_SCHEDULER: &str = "Sequencer/Scheduler";
pub const STATE_PARAMS: &str = "Sequencer/Params";

// error
pub const ERR_LOGIC: &str = "internal logic error";
pub const ERR_INVALID_PUB_KEY: &str = "invalid pubkey";
pub const ERR_INVALID_REQUEST: &str = "invalid request";

pub const ERR_SEQUENCER_EXISTS: &str =
    "sequencer already exist for this address; must use new sequencer address";
pub const ERR_INVALID_SEQUENCER_ADDRESS: &str = "invalid sequencer address";
pub const ERR_UNKNOWN_ROLLAPP_ID: &str = "rollapp does not exist";
pub const ERR_MAX_SEQUENCERS_LIMIT: &str = "too many sequencers for rollapp";
pub const ERR_SEQUENCER_NOT_PERMISSIONED: &str =
    "sequencer is not permissioned for serving the rollapp";
pub const ERR_UNKNOWN_SEQUENCER: &str = "sequencer was not registered";
pub const ERR_SEQUENCER_ROLLAPP_MISMATCH: &str = "sequencer was not registered for this rollapp";
pub const ERR_NOT_ACTIVE_SEQUENCER: &str = "sequencer is not active";
pub const ERR_SEQUENCER_ALREADY_REGISTERED: &str = "sequencer is already registered";

// Description
pub const DESC_MAX_MONIKER_LENGTH: usize = 70;
pub const DESC_MAX_IDENTITY_LENGTH: usize = 3000;
pub const DESC_MAX_WEBSITE_LENGTH: usize = 140;
pub const DESC_MAX_SECURITY_CONTACT_LENGTH: usize = 140;
pub const DESC_MAX_DETAILS_LENGTH: usize = 280;

pub fn validate_description(d: Description) -> Option<StdError> {
    if d.moniker.len() > DESC_MAX_MONIKER_LENGTH {
        return Some(StdError::generic_err(format!(
            "{}: invalid moniker length; got: {}, max: {}",
            ERR_INVALID_REQUEST,
            d.moniker.len(),
            DESC_MAX_MONIKER_LENGTH
        )));
    }

    if d.identity.len() > DESC_MAX_IDENTITY_LENGTH {
        return Some(StdError::generic_err(format!(
            "{}: invalid identity length; got: {}, max: {}",
            ERR_INVALID_REQUEST,
            d.identity.len(),
            DESC_MAX_IDENTITY_LENGTH
        )));
    }

    if d.website.len() > DESC_MAX_WEBSITE_LENGTH {
        return Some(StdError::generic_err(format!(
            "{}: invalid website length; got: {}, max: {}",
            ERR_INVALID_REQUEST,
            d.website.len(),
            DESC_MAX_WEBSITE_LENGTH
        )));
    }

    if d.security_contact.len() > DESC_MAX_SECURITY_CONTACT_LENGTH {
        return Some(StdError::generic_err(format!(
            "{}: invalid security contact length; got: {}, max: {}",
            ERR_INVALID_REQUEST,
            d.security_contact.len(),
            DESC_MAX_SECURITY_CONTACT_LENGTH
        )));
    }

    if d.details.len() > DESC_MAX_DETAILS_LENGTH {
        return Some(StdError::generic_err(format!(
            "{}: invalid details length; got: {}, max: {}",
            ERR_INVALID_REQUEST,
            d.details.len(),
            DESC_MAX_DETAILS_LENGTH
        )));
    }

    None
}
