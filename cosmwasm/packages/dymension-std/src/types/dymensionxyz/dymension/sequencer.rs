use dymension_std_derive::CosmwasmExt;
/// Description defines a sequencer description.
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
#[proto_message(type_url = "/dymensionxyz.dymension.sequencer.Description")]
pub struct Description {
    /// moniker defines a human-readable name for the sequencer.
    #[prost(string, tag = "1")]
    pub moniker: ::prost::alloc::string::String,
    /// identity defines an optional identity signature (ex. UPort or Keybase).
    #[prost(string, tag = "2")]
    pub identity: ::prost::alloc::string::String,
    /// website defines an optional website link.
    #[prost(string, tag = "3")]
    pub website: ::prost::alloc::string::String,
    /// securityContact defines an optional email for security contact.
    #[prost(string, tag = "4")]
    pub security_contact: ::prost::alloc::string::String,
    /// details define other optional details.
    #[prost(string, tag = "5")]
    pub details: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/dymensionxyz.dymension.sequencer.Params")]
pub struct Params {}
/// Sequencer defines a sequencer identified by its' address (sequencerAddress).
/// The sequencer could be attached to only one rollapp (rollappId).
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
#[proto_message(type_url = "/dymensionxyz.dymension.sequencer.Sequencer")]
pub struct Sequencer {
    /// sequencerAddress is the bech32-encoded address of the sequencer account which is the account that the message was sent from.
    #[prost(string, tag = "1")]
    pub sequencer_address: ::prost::alloc::string::String,
    /// pubkey is the public key of the sequencers' dymint client, as a Protobuf Any.
    #[prost(message, optional, tag = "2")]
    pub dymint_pub_key: ::core::option::Option<crate::shim::Any>,
    /// rollappId defines the rollapp to which the sequencer belongs.
    #[prost(string, repeated, tag = "3")]
    pub rollapp_i_ds: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// description defines the descriptive terms for the sequencer.
    #[prost(message, optional, tag = "4")]
    pub description: ::core::option::Option<Description>,
}
/// SequencersByRollapp defines an map between rollappId to a list of
/// all sequencers that belongs to it.
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
#[proto_message(type_url = "/dymensionxyz.dymension.sequencer.SequencersByRollapp")]
pub struct SequencersByRollapp {
    /// rollappId is the unique identifier of the rollapp chain.
    /// The rollappId follows the same standard as cosmos chain_id.
    #[prost(string, tag = "1")]
    #[serde(alias = "rollappID")]
    pub rollapp_id: ::prost::alloc::string::String,
    /// list of sequencers' account address
    /// repeated string sequencers = 2;
    #[prost(message, optional, tag = "2")]
    pub sequencers: ::core::option::Option<Sequencers>,
}
/// Sequencers defines list of sequencers addresses.
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
#[proto_message(type_url = "/dymensionxyz.dymension.sequencer.Sequencers")]
pub struct Sequencers {
    #[prost(string, repeated, tag = "1")]
    pub addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// OperatingStatus defines the operating status of a sequencer
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum OperatingStatus {
    /// OPERATING_STATUS_UNSPECIFIED defines zero-value for status ordering
    Unspecified = 0,
    /// OPERATING_STATUS_PROPOSER defines a sequencer that is active and can propose state updates
    Proposer = 1,
    /// OPERATING_STATUS_INACTIVE defines a sequencer that is not active and won't be scheduled
    Inactive = 2,
}
impl OperatingStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OperatingStatus::Unspecified => "OPERATING_STATUS_UNSPECIFIED",
            OperatingStatus::Proposer => "OPERATING_STATUS_PROPOSER",
            OperatingStatus::Inactive => "OPERATING_STATUS_INACTIVE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OPERATING_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "OPERATING_STATUS_PROPOSER" => Some(Self::Proposer),
            "OPERATING_STATUS_INACTIVE" => Some(Self::Inactive),
            _ => None,
        }
    }
}
/// Scheduler defines the operating status of a sequencer
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
#[proto_message(type_url = "/dymensionxyz.dymension.sequencer.Scheduler")]
pub struct Scheduler {
    /// sequencerAddress is the bech32-encoded address of the sequencer account, identifying the sequencer
    #[prost(string, tag = "1")]
    pub sequencer_address: ::prost::alloc::string::String,
    /// status is the operating status of this sequencer
    #[prost(enumeration = "OperatingStatus", tag = "2")]
    #[serde(with = "OperatingStatus")]
    pub status: i32,
}
/// GenesisState defines the sequencer module's genesis state.
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
#[proto_message(type_url = "/dymensionxyz.dymension.sequencer.GenesisState")]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    #[prost(message, repeated, tag = "2")]
    pub sequencer_list: ::prost::alloc::vec::Vec<Sequencer>,
    #[prost(message, repeated, tag = "3")]
    pub sequencers_by_rollapp_list: ::prost::alloc::vec::Vec<SequencersByRollapp>,
    /// this line is used by starport scaffolding # genesis/proto/state
    #[prost(message, repeated, tag = "4")]
    pub scheduler_list: ::prost::alloc::vec::Vec<Scheduler>,
}
/// SequencerInfo holds information for user query.
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
#[proto_message(type_url = "/dymensionxyz.dymension.sequencer.SequencerInfo")]
pub struct SequencerInfo {
    /// basic sequencer info
    #[prost(message, optional, tag = "1")]
    pub sequencer: ::core::option::Option<Sequencer>,
    /// sequencers' operating status
    #[prost(enumeration = "OperatingStatus", tag = "2")]
    #[serde(with = "OperatingStatus")]
    pub status: i32,
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
#[proto_message(type_url = "/dymensionxyz.dymension.sequencer.QueryParamsRequest")]
#[proto_query(
    path = "/dymensionxyz.dymension.sequencer.Query/Params",
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
#[proto_message(type_url = "/dymensionxyz.dymension.sequencer.QueryParamsResponse")]
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
#[proto_message(type_url = "/dymensionxyz.dymension.sequencer.QueryGetSequencerRequest")]
#[proto_query(
    path = "/dymensionxyz.dymension.sequencer.Query/Sequencer",
    response_type = QueryGetSequencerResponse
)]
pub struct QueryGetSequencerRequest {
    #[prost(string, tag = "1")]
    pub sequencer_address: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/dymensionxyz.dymension.sequencer.QueryGetSequencerResponse")]
pub struct QueryGetSequencerResponse {
    #[prost(message, optional, tag = "1")]
    pub sequencer_info: ::core::option::Option<SequencerInfo>,
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
#[proto_message(type_url = "/dymensionxyz.dymension.sequencer.QueryAllSequencerRequest")]
#[proto_query(
    path = "/dymensionxyz.dymension.sequencer.Query/SequencerAll",
    response_type = QueryAllSequencerResponse
)]
pub struct QueryAllSequencerRequest {
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
#[proto_message(type_url = "/dymensionxyz.dymension.sequencer.QueryAllSequencerResponse")]
pub struct QueryAllSequencerResponse {
    #[prost(message, repeated, tag = "1")]
    pub sequencer_info_list: ::prost::alloc::vec::Vec<SequencerInfo>,
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
#[proto_message(type_url = "/dymensionxyz.dymension.sequencer.QueryGetSequencersByRollappRequest")]
#[proto_query(
    path = "/dymensionxyz.dymension.sequencer.Query/SequencersByRollapp",
    response_type = QueryGetSequencersByRollappResponse
)]
pub struct QueryGetSequencersByRollappRequest {
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
#[proto_message(type_url = "/dymensionxyz.dymension.sequencer.QueryGetSequencersByRollappResponse")]
pub struct QueryGetSequencersByRollappResponse {
    #[prost(string, tag = "1")]
    #[serde(alias = "rollappID")]
    pub rollapp_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub sequencer_info_list: ::prost::alloc::vec::Vec<SequencerInfo>,
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
#[proto_message(type_url = "/dymensionxyz.dymension.sequencer.QueryAllSequencersByRollappRequest")]
#[proto_query(
    path = "/dymensionxyz.dymension.sequencer.Query/SequencersByRollappAll",
    response_type = QueryAllSequencersByRollappResponse
)]
pub struct QueryAllSequencersByRollappRequest {
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
#[proto_message(type_url = "/dymensionxyz.dymension.sequencer.QueryAllSequencersByRollappResponse")]
pub struct QueryAllSequencersByRollappResponse {
    #[prost(message, repeated, tag = "1")]
    pub sequencers_by_rollapp: ::prost::alloc::vec::Vec<QueryGetSequencersByRollappResponse>,
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
#[proto_message(type_url = "/dymensionxyz.dymension.sequencer.QueryGetSchedulerRequest")]
#[proto_query(
    path = "/dymensionxyz.dymension.sequencer.Query/Scheduler",
    response_type = QueryGetSchedulerResponse
)]
pub struct QueryGetSchedulerRequest {
    #[prost(string, tag = "1")]
    pub sequencer_address: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/dymensionxyz.dymension.sequencer.QueryGetSchedulerResponse")]
pub struct QueryGetSchedulerResponse {
    #[prost(message, optional, tag = "1")]
    pub scheduler: ::core::option::Option<Scheduler>,
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
#[proto_message(type_url = "/dymensionxyz.dymension.sequencer.QueryAllSchedulerRequest")]
#[proto_query(
    path = "/dymensionxyz.dymension.sequencer.Query/SchedulerAll",
    response_type = QueryAllSchedulerResponse
)]
pub struct QueryAllSchedulerRequest {
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
#[proto_message(type_url = "/dymensionxyz.dymension.sequencer.QueryAllSchedulerResponse")]
pub struct QueryAllSchedulerResponse {
    #[prost(message, repeated, tag = "1")]
    pub scheduler: ::prost::alloc::vec::Vec<Scheduler>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// MsgCreateSequencer defines a SDK message for creating a new sequencer.
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
#[proto_message(type_url = "/dymensionxyz.dymension.sequencer.MsgCreateSequencer")]
pub struct MsgCreateSequencer {
    /// creator is the bech32-encoded address of the sequencer account which is the account that the message was sent from.
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    /// pubkey is the public key of the sequencers' dymint client, as a Protobuf Any.
    #[prost(message, optional, tag = "2")]
    pub dymint_pub_key: ::core::option::Option<crate::shim::Any>,
    /// rollappId defines the rollapp to which the sequencer belongs.
    #[prost(string, tag = "3")]
    #[serde(alias = "rollappID")]
    pub rollapp_id: ::prost::alloc::string::String,
    /// description defines the descriptive terms for the sequencer.
    #[prost(message, optional, tag = "4")]
    pub description: ::core::option::Option<Description>,
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
#[proto_message(type_url = "/dymensionxyz.dymension.sequencer.MsgCreateSequencerResponse")]
pub struct MsgCreateSequencerResponse {}
pub struct SequencerQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> SequencerQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn params(&self) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
    pub fn sequencer(
        &self,
        sequencer_address: ::prost::alloc::string::String,
    ) -> Result<QueryGetSequencerResponse, cosmwasm_std::StdError> {
        QueryGetSequencerRequest { sequencer_address }.query(self.querier)
    }
    pub fn sequencer_all(
        &self,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryAllSequencerResponse, cosmwasm_std::StdError> {
        QueryAllSequencerRequest { pagination }.query(self.querier)
    }
    pub fn sequencers_by_rollapp(
        &self,
        rollapp_id: ::prost::alloc::string::String,
    ) -> Result<QueryGetSequencersByRollappResponse, cosmwasm_std::StdError> {
        QueryGetSequencersByRollappRequest { rollapp_id }.query(self.querier)
    }
    pub fn sequencers_by_rollapp_all(
        &self,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryAllSequencersByRollappResponse, cosmwasm_std::StdError> {
        QueryAllSequencersByRollappRequest { pagination }.query(self.querier)
    }
    pub fn scheduler(
        &self,
        sequencer_address: ::prost::alloc::string::String,
    ) -> Result<QueryGetSchedulerResponse, cosmwasm_std::StdError> {
        QueryGetSchedulerRequest { sequencer_address }.query(self.querier)
    }
    pub fn scheduler_all(
        &self,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryAllSchedulerResponse, cosmwasm_std::StdError> {
        QueryAllSchedulerRequest { pagination }.query(self.querier)
    }
}
impl OperatingStatus {
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
