use dymension_std_derive::CosmwasmExt;
/// DenomUnit represents a struct that describes a given
/// denomination unit of the basic token.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/dymensionxyz.dymension.rollapp.DenomUnit")]
pub struct DenomUnit {
    /// denom represents the string name of the given denom unit (e.g uatom).
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    /// exponent represents power of 10 exponent that one must
    /// raise the base_denom to in order to equal the given DenomUnit's denom
    /// 1 denom = 10^exponent base_denom
    /// (e.g. with a base_denom of uatom, one can create a DenomUnit of 'atom' with
    /// exponent = 6, thus: 1 atom = 10^6 uatom).
    #[prost(uint32, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub exponent: u32,
    /// aliases is a list of string aliases for the given denom
    #[prost(string, repeated, tag = "3")]
    pub aliases: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Metadata represents a struct that describes
/// a basic token.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/dymensionxyz.dymension.rollapp.TokenMetadata")]
pub struct TokenMetadata {
    #[prost(string, tag = "1")]
    pub description: ::prost::alloc::string::String,
    /// denom_units represents the list of DenomUnit's for a given coin
    #[prost(message, repeated, tag = "2")]
    pub denom_units: ::prost::alloc::vec::Vec<DenomUnit>,
    /// base represents the base denom (should be the DenomUnit with exponent = 0).
    #[prost(string, tag = "3")]
    pub base: ::prost::alloc::string::String,
    /// display indicates the suggested denom that should be
    /// displayed in clients.
    #[prost(string, tag = "4")]
    pub display: ::prost::alloc::string::String,
    /// name defines the name of the token (eg: Cosmos Atom)
    ///
    /// Since: cosmos-sdk 0.43
    #[prost(string, tag = "5")]
    pub name: ::prost::alloc::string::String,
    /// symbol is the token symbol usually shown on exchanges (eg: ATOM). This can
    /// be the same as the display.
    ///
    /// Since: cosmos-sdk 0.43
    #[prost(string, tag = "6")]
    pub symbol: ::prost::alloc::string::String,
    /// URI to a document (on or off-chain) that contains additional information. Optional.
    ///
    /// Since: cosmos-sdk 0.46
    #[prost(string, tag = "7")]
    pub uri: ::prost::alloc::string::String,
    /// URIHash is a sha256 hash of a document pointed by URI. It's used to verify that
    /// the document didn't change. Optional.
    ///
    /// Since: cosmos-sdk 0.46
    #[prost(string, tag = "8")]
    pub uri_hash: ::prost::alloc::string::String,
}
/// BlockDescriptor defines a single rollapp chain block description.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/dymensionxyz.dymension.rollapp.BlockDescriptor")]
pub struct BlockDescriptor {
    /// height is the height of the block
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub height: u64,
    /// stateRoot is a 32 byte array of the hash of the block (state root of the block)
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub state_root: ::prost::alloc::vec::Vec<u8>,
    /// intermediateStatesRoot is a 32 byte array representing
    /// the root of a Merkle tree built from the ISRs of the block (Intermediate State Roots)
    #[prost(bytes = "vec", tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub intermediate_states_root: ::prost::alloc::vec::Vec<u8>,
}
/// BlockDescriptors defines list of BlockDescriptor.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/dymensionxyz.dymension.rollapp.BlockDescriptors")]
pub struct BlockDescriptors {
    #[prost(message, repeated, tag = "1")]
    pub bd: ::prost::alloc::vec::Vec<BlockDescriptor>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/dymensionxyz.dymension.rollapp.DeployerParams")]
pub struct DeployerParams {
    /// address is a bech32-encoded address of the
    /// accounts that are allowed to create a rollapp.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// max_rollapps is the maximum number of rollapps that address allowed to deploy
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub max_rollapps: u64,
}
/// Params defines the parameters for the module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/dymensionxyz.dymension.rollapp.Params")]
pub struct Params {
    /// dispute_period_in_blocks the number of blocks it takes
    /// to change a status of a state from received to finalized.
    /// during that period, any user could submit fraud proof
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub dispute_period_in_blocks: u64,
    /// deployer_whitelist is a list of the
    /// accounts that are allowed to create a rollapp and maximum number of rollapps.
    /// In the case of an empty list, there are no restrictions
    #[prost(message, repeated, tag = "2")]
    pub deployer_whitelist: ::prost::alloc::vec::Vec<DeployerParams>,
    #[prost(bool, tag = "3")]
    pub rollapps_enabled: bool,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum StateStatus {
    /// zero-value for status ordering
    Unspecified = 0,
    /// STATE_STATUS_RECEIVED defines a rollapp state where the state update transaction was published on dYmension chain
    Received = 1,
    /// STATE_STATUS_FINALIZED defines a rollapp state where the the "Dispute Period" has ended and this state is considered final
    Finalized = 2,
}
impl StateStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            StateStatus::Unspecified => "STATE_STATUS_UNSPECIFIED",
            StateStatus::Received => "STATE_STATUS_RECEIVED",
            StateStatus::Finalized => "STATE_STATUS_FINALIZED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "STATE_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "STATE_STATUS_RECEIVED" => Some(Self::Received),
            "STATE_STATUS_FINALIZED" => Some(Self::Finalized),
            _ => None,
        }
    }
}
/// StateInfoIndex is the data used for indexing and retrieving a StateInfo
/// it updated and saved with every UpdateState in StateInfo.
/// We use the this structure also for:
/// 1. LatestStateInfoIndex which defines the rollapps' current (latest) index of the last UpdateState
/// 2. LatestFinalizedStateIndex which defines the rollapps' current (latest) index of the latest StateInfo that was finalized
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/dymensionxyz.dymension.rollapp.StateInfoIndex")]
pub struct StateInfoIndex {
    /// rollappId is the rollapp that the sequencer belongs to and asking to update
    /// it used to identify the what rollapp a StateInfo belongs
    /// The rollappId follows the same standard as cosmos chain_id
    #[prost(string, tag = "1")]
    #[serde(alias = "rollappID")]
    pub rollapp_id: ::prost::alloc::string::String,
    /// index is a sequential increasing number, updating on each
    /// state update used for indexing to a specific state info, the first index is 1
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub index: u64,
}
/// StateInfo defines a rollapps' state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/dymensionxyz.dymension.rollapp.StateInfo")]
pub struct StateInfo {
    /// stateInfoIndex defines what rollapp the state belongs to
    /// and in which index it can be referenced
    #[prost(message, optional, tag = "1")]
    pub state_info_index: ::core::option::Option<StateInfoIndex>,
    /// sequencer is the bech32-encoded address of the sequencer sent the update
    #[prost(string, tag = "2")]
    pub sequencer: ::prost::alloc::string::String,
    /// startHeight is the block height of the first block in the batch
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub start_height: u64,
    /// numBlocks is the number of blocks included in this batch update
    #[prost(uint64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub num_blocks: u64,
    /// DAPath is the description of the location on the DA layer
    #[prost(string, tag = "5")]
    pub da_path: ::prost::alloc::string::String,
    /// version is the version of the rollapp
    #[prost(uint64, tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub version: u64,
    /// creationHeight is the height at which the UpdateState took place
    #[prost(uint64, tag = "7")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub creation_height: u64,
    /// status is the status of the state update
    #[prost(enumeration = "StateStatus", tag = "8")]
    #[serde(with = "StateStatus")]
    pub status: i32,
    /// BDs is a list of block description objects (one per block)
    /// the list must be ordered by height, starting from startHeight to startHeight+numBlocks-1
    #[prost(message, optional, tag = "9")]
    pub b_ds: ::core::option::Option<BlockDescriptors>,
}
/// StateInfoSummary is a compact representation of StateInfo
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/dymensionxyz.dymension.rollapp.StateInfoSummary")]
pub struct StateInfoSummary {
    /// stateInfoIndex defines what rollapp the state belongs to
    /// and in which index it can be referenced
    #[prost(message, optional, tag = "1")]
    pub state_info_index: ::core::option::Option<StateInfoIndex>,
    /// status is the status of the state update
    #[prost(enumeration = "StateStatus", tag = "2")]
    #[serde(with = "StateStatus")]
    pub status: i32,
    /// creationHeight is the height at which the UpdateState took place
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub creation_height: u64,
}
/// BlockHeightToFinalizationQueue defines a map from block height to list of states to finalized
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/dymensionxyz.dymension.rollapp.BlockHeightToFinalizationQueue")]
pub struct BlockHeightToFinalizationQueue {
    /// finalizationHeight is the block height that the state should be finalized
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub finalization_height: u64,
    /// finalizationQueue is a list of states that are waiting to be finalized
    /// when the block height becomes finalizationHeight
    #[prost(message, repeated, tag = "2")]
    pub finalization_queue: ::prost::alloc::vec::Vec<StateInfoIndex>,
}
/// Rollapp defines a rollapp object. First the RollApp is created and then
/// sequencers can be created and attached. The RollApp is identified by rollappId
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/dymensionxyz.dymension.rollapp.Rollapp")]
pub struct Rollapp {
    /// The unique identifier of the rollapp chain.
    /// The rollappId follows the same standard as cosmos chain_id.
    #[prost(string, tag = "1")]
    #[serde(alias = "rollappID")]
    pub rollapp_id: ::prost::alloc::string::String,
    /// creator is the bech32-encoded address of the rollapp creator.
    #[prost(string, tag = "2")]
    pub creator: ::prost::alloc::string::String,
    /// version is the software and configuration version.
    /// starts from 1 and increases by one on every MsgUpdateState
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub version: u64,
    /// codeStamp is a generated hash for unique identification of the rollapp code.
    #[deprecated]
    #[prost(string, tag = "4")]
    pub code_stamp: ::prost::alloc::string::String,
    /// genesisPath is the description of the genesis file location on the DA.
    #[deprecated]
    #[prost(string, tag = "5")]
    pub genesis_path: ::prost::alloc::string::String,
    /// maxWithholdingBlocks is the maximum number of blocks for
    /// an active sequencer to send a state update (MsgUpdateState).
    #[deprecated]
    #[prost(uint64, tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub max_withholding_blocks: u64,
    /// maxSequencers is the maximum number of sequencers.
    #[prost(uint64, tag = "7")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub max_sequencers: u64,
    /// permissionedAddresses is a bech32-encoded address list of the sequencers that are allowed to serve this rollappId.
    /// In the case of an empty list, the rollapp is considered permissionless.
    #[prost(string, repeated, tag = "8")]
    pub permissioned_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// tokenMetadata is a list of TokenMetadata that are registered on this rollapp
    #[prost(message, repeated, tag = "9")]
    pub token_metadata: ::prost::alloc::vec::Vec<TokenMetadata>,
}
/// Rollapp summary is a compact representation of Rollapp
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/dymensionxyz.dymension.rollapp.RollappSummary")]
pub struct RollappSummary {
    /// The unique identifier of the rollapp chain.
    /// The rollappId follows the same standard as cosmos chain_id.
    #[prost(string, tag = "1")]
    #[serde(alias = "rollappID")]
    pub rollapp_id: ::prost::alloc::string::String,
    /// Defines the index of the last rollapp UpdateState.
    #[prost(message, optional, tag = "2")]
    pub latest_state_index: ::core::option::Option<StateInfoIndex>,
    /// Defines the index of the last rollapp UpdateState that was finalized.
    #[prost(message, optional, tag = "3")]
    pub latest_finalized_state_index: ::core::option::Option<StateInfoIndex>,
}
/// GenesisState defines the rollapp module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/dymensionxyz.dymension.rollapp.GenesisState")]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    #[prost(message, repeated, tag = "2")]
    pub rollapp_list: ::prost::alloc::vec::Vec<Rollapp>,
    #[prost(message, repeated, tag = "3")]
    pub state_info_list: ::prost::alloc::vec::Vec<StateInfo>,
    #[prost(message, repeated, tag = "4")]
    pub latest_state_info_index_list: ::prost::alloc::vec::Vec<StateInfoIndex>,
    #[prost(message, repeated, tag = "5")]
    pub latest_finalized_state_index_list: ::prost::alloc::vec::Vec<StateInfoIndex>,
    /// this line is used by starport scaffolding # genesis/proto/state
    #[prost(message, repeated, tag = "6")]
    pub block_height_to_finalization_queue_list:
        ::prost::alloc::vec::Vec<BlockHeightToFinalizationQueue>,
}
/// QueryParamsRequest is request type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/dymensionxyz.dymension.rollapp.QueryParamsRequest")]
#[proto_query(
    path = "/dymensionxyz.dymension.rollapp.Query/Params",
    response_type = QueryParamsResponse
)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is response type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/dymensionxyz.dymension.rollapp.QueryParamsResponse")]
pub struct QueryParamsResponse {
    /// params holds all the parameters of this module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/dymensionxyz.dymension.rollapp.QueryGetRollappRequest")]
#[proto_query(
    path = "/dymensionxyz.dymension.rollapp.Query/Rollapp",
    response_type = QueryGetRollappResponse
)]
pub struct QueryGetRollappRequest {
    #[prost(string, tag = "1")]
    #[serde(alias = "rollappID")]
    pub rollapp_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/dymensionxyz.dymension.rollapp.QueryGetRollappByEIP155Request")]
#[proto_query(
    path = "/dymensionxyz.dymension.rollapp.Query/RollappByEIP155",
    response_type = QueryGetRollappResponse
)]
pub struct QueryGetRollappByEip155Request {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub eip155: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/dymensionxyz.dymension.rollapp.QueryGetLatestStateIndexRequest")]
#[proto_query(
    path = "/dymensionxyz.dymension.rollapp.Query/LatestStateIndex",
    response_type = QueryGetLatestStateIndexResponse
)]
pub struct QueryGetLatestStateIndexRequest {
    #[prost(string, tag = "1")]
    #[serde(alias = "rollappID")]
    pub rollapp_id: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub finalized: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/dymensionxyz.dymension.rollapp.QueryGetLatestStateIndexResponse")]
pub struct QueryGetLatestStateIndexResponse {
    #[prost(message, optional, tag = "1")]
    pub state_index: ::core::option::Option<StateInfoIndex>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/dymensionxyz.dymension.rollapp.QueryGetRollappResponse")]
pub struct QueryGetRollappResponse {
    #[prost(message, optional, tag = "1")]
    pub rollapp: ::core::option::Option<Rollapp>,
    /// Defines the index of the last rollapp UpdateState.
    #[prost(message, optional, tag = "2")]
    pub latest_state_index: ::core::option::Option<StateInfoIndex>,
    /// Defines the index of the last rollapp UpdateState that was finalized.
    #[prost(message, optional, tag = "3")]
    pub latest_finalized_state_index: ::core::option::Option<StateInfoIndex>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/dymensionxyz.dymension.rollapp.QueryAllRollappRequest")]
#[proto_query(
    path = "/dymensionxyz.dymension.rollapp.Query/RollappAll",
    response_type = QueryAllRollappResponse
)]
pub struct QueryAllRollappRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/dymensionxyz.dymension.rollapp.QueryAllRollappResponse")]
pub struct QueryAllRollappResponse {
    #[prost(message, repeated, tag = "1")]
    pub rollapp: ::prost::alloc::vec::Vec<RollappSummary>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/dymensionxyz.dymension.rollapp.QueryGetStateInfoRequest")]
#[proto_query(
    path = "/dymensionxyz.dymension.rollapp.Query/StateInfo",
    response_type = QueryGetStateInfoResponse
)]
pub struct QueryGetStateInfoRequest {
    #[prost(string, tag = "1")]
    #[serde(alias = "rollappID")]
    pub rollapp_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub index: u64,
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub height: u64,
    #[prost(bool, tag = "4")]
    pub finalized: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/dymensionxyz.dymension.rollapp.QueryGetStateInfoResponse")]
pub struct QueryGetStateInfoResponse {
    #[prost(message, optional, tag = "1")]
    pub state_info: ::core::option::Option<StateInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/dymensionxyz.dymension.rollapp.QueryAllStateInfoRequest")]
#[proto_query(
    path = "/dymensionxyz.dymension.rollapp.Query/StateInfoAll",
    response_type = QueryAllStateInfoResponse
)]
pub struct QueryAllStateInfoRequest {
    #[prost(string, tag = "1")]
    #[serde(alias = "rollappID")]
    pub rollapp_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/dymensionxyz.dymension.rollapp.QueryAllStateInfoResponse")]
pub struct QueryAllStateInfoResponse {
    #[prost(message, repeated, tag = "1")]
    pub state_info: ::prost::alloc::vec::Vec<StateInfoSummary>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// ===================== MsgCreateRollapp
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/dymensionxyz.dymension.rollapp.MsgCreateRollapp")]
pub struct MsgCreateRollapp {
    /// creator is the bech32-encoded address of the rollapp creator
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    /// rollappId is the unique identifier of the rollapp chain.
    /// The rollappId follows the same standard as cosmos chain_id
    ///
    /// [(gogoproto.customname) = "ChainID"];
    #[prost(string, tag = "2")]
    #[serde(alias = "rollappID")]
    pub rollapp_id: ::prost::alloc::string::String,
    /// codeStamp is the description of the genesis file location on the DA
    #[deprecated]
    #[prost(string, tag = "3")]
    pub code_stamp: ::prost::alloc::string::String,
    /// genesisPath is the description of the genesis file location on the DA
    #[deprecated]
    #[prost(string, tag = "4")]
    pub genesis_path: ::prost::alloc::string::String,
    /// maxWithholdingBlocks is the maximum number of blocks for
    /// an active sequencer to send a state update (MsgUpdateState)
    #[deprecated]
    #[prost(uint64, tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub max_withholding_blocks: u64,
    /// maxSequencers is the maximum number of sequencers
    #[prost(uint64, tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub max_sequencers: u64,
    /// permissionedAddresses is a bech32-encoded address list of the
    /// sequencers that are allowed to serve this rollappId.
    /// In the case of an empty list, the rollapp is considered permissionless
    #[prost(string, repeated, tag = "7")]
    pub permissioned_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// metadata provides the client information for all the registered tokens.
    #[prost(message, repeated, tag = "8")]
    pub metadatas: ::prost::alloc::vec::Vec<TokenMetadata>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/dymensionxyz.dymension.rollapp.MsgCreateRollappResponse")]
pub struct MsgCreateRollappResponse {}
/// ===================== MsgUpdateState
/// Updating a rollapp state with a block batch
/// a block batch is a list of ordered blocks (by height)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/dymensionxyz.dymension.rollapp.MsgUpdateState")]
pub struct MsgUpdateState {
    /// creator is the bech32-encoded address of the sequencer sending the update
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    /// rollappId is the rollapp that the sequencer belongs to and asking to update
    /// The rollappId follows the same standard as cosmos chain_id
    #[prost(string, tag = "2")]
    #[serde(alias = "rollappID")]
    pub rollapp_id: ::prost::alloc::string::String,
    /// startHeight is the block height of the first block in the batch
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub start_height: u64,
    /// numBlocks is the number of blocks included in this batch update
    #[prost(uint64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub num_blocks: u64,
    /// DAPath is the description of the location on the DA layer
    #[prost(string, tag = "5")]
    pub da_path: ::prost::alloc::string::String,
    /// version is the version of the rollapp
    #[prost(uint64, tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub version: u64,
    /// BDs is a list of block description objects (one per block)
    /// the list must be ordered by height, starting from startHeight to startHeight+numBlocks-1
    #[prost(message, optional, tag = "7")]
    pub b_ds: ::core::option::Option<BlockDescriptors>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/dymensionxyz.dymension.rollapp.MsgUpdateStateResponse")]
pub struct MsgUpdateStateResponse {}
pub struct RollappQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> RollappQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn params(&self) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
    pub fn rollapp(
        &self,
        rollapp_id: ::prost::alloc::string::String,
    ) -> Result<QueryGetRollappResponse, cosmwasm_std::StdError> {
        QueryGetRollappRequest { rollapp_id }.query(self.querier)
    }
    pub fn rollapp_by_eip155(
        &self,
        eip155: u64,
    ) -> Result<QueryGetRollappResponse, cosmwasm_std::StdError> {
        QueryGetRollappByEip155Request { eip155 }.query(self.querier)
    }
    pub fn rollapp_all(
        &self,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryAllRollappResponse, cosmwasm_std::StdError> {
        QueryAllRollappRequest { pagination }.query(self.querier)
    }
    pub fn latest_state_index(
        &self,
        rollapp_id: ::prost::alloc::string::String,
        finalized: bool,
    ) -> Result<QueryGetLatestStateIndexResponse, cosmwasm_std::StdError> {
        QueryGetLatestStateIndexRequest {
            rollapp_id,
            finalized,
        }
        .query(self.querier)
    }
    pub fn state_info(
        &self,
        rollapp_id: ::prost::alloc::string::String,
        index: u64,
        height: u64,
        finalized: bool,
    ) -> Result<QueryGetStateInfoResponse, cosmwasm_std::StdError> {
        QueryGetStateInfoRequest {
            rollapp_id,
            index,
            height,
            finalized,
        }
        .query(self.querier)
    }
    pub fn state_info_all(
        &self,
        rollapp_id: ::prost::alloc::string::String,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryAllStateInfoResponse, cosmwasm_std::StdError> {
        QueryAllStateInfoRequest {
            rollapp_id,
            pagination,
        }
        .query(self.querier)
    }
}
impl StateStatus {
    pub fn serialize<S>(value: &i32, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = Self::try_from(*value).map_err(serde::ser::Error::custom)?;
        serializer.serialize_str(s.as_str_name())
    }
    pub fn deserialize<'de, D>(deserializer: D) -> core::result::Result<i32, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::Deserialize;
        let s = String::deserialize(deserializer)?;
        let e = Self::from_str_name(s.as_str())
            .ok_or("cannot transform")
            .map_err(serde::de::Error::custom)?;
        Ok(e as i32)
    }
}
