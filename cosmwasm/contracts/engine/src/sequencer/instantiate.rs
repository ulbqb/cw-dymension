use cosmwasm_std::Storage;
use dymension_std::types::dymensionxyz::dymension::sequencer::GenesisState;

use crate::sequencer::state::{
    set_params, set_scheduler, set_sequencer, set_sequencers_by_rollapp,
};

pub fn instantiate(storage: &mut dyn Storage, msg: GenesisState) {
    for elem in msg.sequencer_list.into_iter() {
        set_sequencer(storage, elem);
    }

    for elem in msg.sequencers_by_rollapp_list.into_iter() {
        set_sequencers_by_rollapp(storage, elem);
    }

    for elem in msg.scheduler_list.into_iter() {
        set_scheduler(storage, elem)
    }

    set_params(storage, msg.params.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sequencer::state::{
        get_all_scheduler, get_all_sequencer, get_all_sequencers_by_rollapp, get_params,
    };
    use cosmwasm_std::testing::mock_dependencies;
    use dymension_std::types::dymensionxyz::dymension::sequencer::{
        Params, Scheduler, Sequencer, SequencersByRollapp,
    };

    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies();

        let params = Params::default();
        let sequencer = Sequencer::default();
        let sequencers_by_rollapp = SequencersByRollapp::default();
        let scheduler = Scheduler::default();
        let genesis_state = GenesisState {
            params: Some(params.clone()),
            sequencer_list: vec![sequencer.clone()],
            sequencers_by_rollapp_list: vec![sequencers_by_rollapp.clone()],
            scheduler_list: vec![scheduler.clone()],
        };

        instantiate(deps.as_mut().storage, genesis_state);

        let params_res = get_params(deps.as_mut().storage).unwrap();
        assert_eq!(params, params_res);

        let sequencer_list = get_all_sequencer(deps.as_mut().storage).unwrap();
        assert_eq!(1, sequencer_list.len());
        assert_eq!(sequencer, sequencer_list[0].1);

        let sequencers_by_rollapp_list =
            get_all_sequencers_by_rollapp(deps.as_mut().storage).unwrap();
        assert_eq!(1, sequencers_by_rollapp_list.len());
        assert_eq!(sequencers_by_rollapp, sequencers_by_rollapp_list[0].1);

        let scheduler_list = get_all_scheduler(deps.as_mut().storage).unwrap();
        assert_eq!(1, scheduler_list.len());
        assert_eq!(scheduler, scheduler_list[0].1);
    }
}
