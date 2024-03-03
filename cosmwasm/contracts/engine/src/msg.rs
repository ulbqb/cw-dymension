use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Binary;
use dymension_std::types::dymensionxyz::dymension::{rollapp, sequencer};

use crate::system::types::MsgEndBlocks;

/// Message type for `instantiate` entry_point
#[cw_serde]
pub struct InstantiateMsg {
    pub rollapp: rollapp::GenesisState,
    pub sequencer: sequencer::GenesisState,
}

/// Message type for `execute` entry_point
#[cw_serde]
pub enum ExecuteMsg {
    Rollapp(RollappExecute),
    Sequencer(SequencerExecute),
    System(SystemExecute),
}

#[cw_serde]
pub enum RollappExecute {
    CreateRollapp(rollapp::MsgCreateRollapp),
    UpdateState(rollapp::MsgUpdateState),
}

#[cw_serde]
pub enum SequencerExecute {
    CreateSequencer(sequencer::MsgCreateSequencer),
}

#[cw_serde]
pub enum SystemExecute {
    EndBlocks(MsgEndBlocks),
}

/// Message type for `migrate` entry_point
#[cw_serde]
pub enum MigrateMsg {}

/// Message type for `query` entry_point
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Binary)]
    Rollapp(RollappQuery),

    #[returns(Binary)]
    Sequencer(SequencerQuery),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum RollappQuery {
    #[returns(rollapp::QueryParamsResponse)]
    Params(rollapp::QueryParamsRequest),

    #[returns(rollapp::QueryGetRollappResponse)]
    Rollapp(rollapp::QueryGetRollappRequest),

    #[returns(rollapp::QueryGetRollappResponse)]
    RollappByEip155(rollapp::QueryGetRollappByEip155Request),

    #[returns(rollapp::QueryAllRollappResponse)]
    RollappAll(rollapp::QueryAllRollappRequest),

    #[returns(rollapp::QueryGetLatestStateIndexResponse)]
    LatestStateIndex(rollapp::QueryGetLatestStateIndexRequest),

    #[returns(rollapp::QueryGetStateInfoResponse)]
    StateInfo(rollapp::QueryGetStateInfoRequest),

    #[returns(rollapp::QueryAllRollappResponse)]
    StateInfoAll(rollapp::QueryAllStateInfoRequest),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum SequencerQuery {
    #[returns(sequencer::QueryParamsResponse)]
    Params(sequencer::QueryParamsRequest),

    #[returns(sequencer::QueryGetSequencerResponse)]
    Sequencer(sequencer::QueryGetSequencerRequest),

    #[returns(sequencer::QueryAllSequencerResponse)]
    SequencerAll(sequencer::QueryAllSequencerRequest),

    #[returns(sequencer::QueryGetSequencersByRollappResponse)]
    SequencersByRollapp(sequencer::QueryGetSequencersByRollappRequest),

    #[returns(sequencer::QueryAllSequencersByRollappResponse)]
    SequencersByRollappAll(sequencer::QueryAllSequencersByRollappRequest),

    #[returns(sequencer::QueryGetSchedulerResponse)]
    Scheduler(sequencer::QueryGetSchedulerRequest),

    #[returns(sequencer::QueryAllSchedulerResponse)]
    SchedulerAll(sequencer::QueryAllSchedulerRequest),
}
