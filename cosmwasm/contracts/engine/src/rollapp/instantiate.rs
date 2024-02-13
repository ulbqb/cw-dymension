use cosmwasm_std::Storage;
use dymension_std::types::dymensionxyz::dymension::rollapp::GenesisState;

use crate::rollapp::state::{
    set_block_height_to_finalization_queue, set_latest_finalized_state_index,
    set_latest_state_info_index, set_params, set_rollapp, set_state_info,
};

pub fn instantiate(storage: &mut dyn Storage, msg: GenesisState) {
    for elem in msg.rollapp_list.into_iter() {
        set_rollapp(storage, elem);
    }

    for elem in msg.state_info_list.into_iter() {
        set_state_info(storage, elem);
    }

    for elem in msg.latest_state_info_index_list.into_iter() {
        set_latest_state_info_index(storage, elem)
    }

    for elem in msg.latest_finalized_state_index_list.into_iter() {
        set_latest_finalized_state_index(storage, elem)
    }

    for elem in msg.block_height_to_finalization_queue_list.into_iter() {
        set_block_height_to_finalization_queue(storage, elem)
    }

    set_params(storage, msg.params.unwrap())
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::mock_dependencies;
    use dymension_std::types::dymensionxyz::dymension::rollapp::{
        BlockHeightToFinalizationQueue, DeployerParams, GenesisState, Params, Rollapp, StateInfo,
        StateInfoIndex,
    };

    use crate::rollapp::instantiate::instantiate;
    use crate::rollapp::state::{
        get_all_block_height_to_finalization_queue, get_all_latest_finalized_state_index,
        get_all_latest_state_info_index, get_all_rollapp, get_all_state_info, get_params,
    };

    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies();

        let mut params = Params::default();
        params.deployer_whitelist = vec![DeployerParams::default()];
        let rollapp = Rollapp::default();
        let mut state_info = StateInfo::default();
        state_info.state_info_index = Some(StateInfoIndex::default());
        let state_info_index = StateInfoIndex::default();
        let mut queue = BlockHeightToFinalizationQueue::default();
        queue.finalization_queue = vec![StateInfoIndex::default()];
        let genesis_state = GenesisState {
            params: Some(params.clone()),
            rollapp_list: vec![rollapp.clone()],
            state_info_list: vec![state_info.clone()],
            latest_state_info_index_list: vec![state_info_index.clone()],
            latest_finalized_state_index_list: vec![state_info_index.clone()],
            block_height_to_finalization_queue_list: vec![queue.clone()],
        };

        instantiate(deps.as_mut().storage, genesis_state);

        let params_res = get_params(deps.as_mut().storage).unwrap();
        assert_eq!(params, params_res);

        let rollapp_list = get_all_rollapp(deps.as_mut().storage).unwrap();
        assert_eq!(1, rollapp_list.len());
        assert_eq!(rollapp, rollapp_list[0].1);

        let state_info_list = get_all_state_info(deps.as_mut().storage).unwrap();
        assert_eq!(1, state_info_list.len());
        assert_eq!(state_info, state_info_list[0].1);

        let latest_state_info_index_list =
            get_all_latest_state_info_index(deps.as_mut().storage).unwrap();
        assert_eq!(1, latest_state_info_index_list.len());
        assert_eq!(state_info_index.clone(), latest_state_info_index_list[0].1);

        let latest_finalized_state_index_list =
            get_all_latest_finalized_state_index(deps.as_mut().storage).unwrap();
        assert_eq!(1, latest_finalized_state_index_list.len());
        assert_eq!(
            state_info_index.clone(),
            latest_finalized_state_index_list[0].1
        );

        let block_height_to_finalization_queue_list =
            get_all_block_height_to_finalization_queue(deps.as_mut().storage).unwrap();
        assert_eq!(1, block_height_to_finalization_queue_list.len());
        assert_eq!(queue, block_height_to_finalization_queue_list[0].1);
    }
}
