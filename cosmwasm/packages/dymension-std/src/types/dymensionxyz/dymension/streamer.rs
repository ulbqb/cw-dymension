use dymension_std_derive::CosmwasmExt;
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
#[proto_message(type_url = "/dymensionxyz.dymension.streamer.DistrInfo")]
pub struct DistrInfo {
    #[prost(string, tag = "1")]
    pub total_weight: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub records: ::prost::alloc::vec::Vec<DistrRecord>,
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
#[proto_message(type_url = "/dymensionxyz.dymension.streamer.DistrRecord")]
pub struct DistrRecord {
    #[prost(uint64, tag = "1")]
    #[serde(alias = "gaugeID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub gauge_id: u64,
    #[prost(string, tag = "2")]
    pub weight: ::prost::alloc::string::String,
}
/// Params holds parameters for the streamer module
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
#[proto_message(type_url = "/dymensionxyz.dymension.streamer.Params")]
pub struct Params {}
/// Stream is an object that stores and distributes yields to recipients who
/// satisfy certain conditions. Currently streams support conditions around the
/// duration for which a given denom is locked.
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
#[proto_message(type_url = "/dymensionxyz.dymension.streamer.Stream")]
pub struct Stream {
    /// id is the unique ID of a Stream
    #[prost(uint64, tag = "1")]
    #[serde(alias = "ID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub id: u64,
    /// distribute_to is the distr_info.
    #[prost(message, optional, tag = "2")]
    pub distribute_to: ::core::option::Option<DistrInfo>,
    /// coins is the total amount of coins that have been in the stream
    /// Can distribute multiple coin denoms
    #[prost(message, repeated, tag = "3")]
    pub coins: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// start_time is the distribution start time
    #[prost(message, optional, tag = "4")]
    pub start_time: ::core::option::Option<crate::shim::Timestamp>,
    /// distr_epoch_identifier is what epoch type di-stribution will be triggered by
    /// (day, week, etc.)
    #[prost(string, tag = "5")]
    #[serde(alias = "distr_epochIDentifier")]
    pub distr_epoch_identifier: ::prost::alloc::string::String,
    /// num_epochs_paid_over is the number of total epochs distribution will be
    /// completed over
    #[prost(uint64, tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub num_epochs_paid_over: u64,
    /// filled_epochs is the number of epochs distribution has been completed on
    /// already
    #[prost(uint64, tag = "7")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub filled_epochs: u64,
    /// distributed_coins are coins that have been distributed already
    #[prost(message, repeated, tag = "8")]
    pub distributed_coins:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// GenesisState defines the streamer module's various parameters when first
/// initialized
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
#[proto_message(type_url = "/dymensionxyz.dymension.streamer.GenesisState")]
pub struct GenesisState {
    /// params are all the parameters of the module
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// streams are all streams that should exist at genesis
    #[prost(message, repeated, tag = "2")]
    pub streams: ::prost::alloc::vec::Vec<Stream>,
    /// last_stream_id is what the stream number will increment from when creating
    /// the next stream after genesis
    #[prost(uint64, tag = "3")]
    #[serde(alias = "last_streamID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub last_stream_id: u64,
}
/// ReplaceStreamDistributionProposal is a gov Content type for updating a stream
/// If a ReplaceStreamDistributionProposal passes, the proposal’s records
/// override the existing DistrRecords set in the module. Each record has a
/// specified gauge id and weight, and the incentives are distributed to each
/// gauge according to weight/total_weight.
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
#[proto_message(type_url = "/dymensionxyz.dymension.streamer.ReplaceStreamDistributionProposal")]
pub struct ReplaceStreamDistributionProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    #[serde(alias = "streamID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub stream_id: u64,
    #[prost(message, repeated, tag = "4")]
    pub records: ::prost::alloc::vec::Vec<DistrRecord>,
}
/// For example: if the existing DistrRecords were:
/// [(Gauge 0, 5), (Gauge 1, 6), (Gauge 2, 6)]
/// An UpdatePoolIncentivesProposal includes
/// [(Gauge 1, 0), (Gauge 2, 4), (Gauge 3, 10)]
/// This would delete Gauge 1, Edit Gauge 2, and Add Gauge 3
/// The result DistrRecords in state would be:
/// [(Gauge 0, 5), (Gauge 2, 4), (Gauge 3, 10)]
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
#[proto_message(type_url = "/dymensionxyz.dymension.streamer.UpdateStreamDistributionProposal")]
pub struct UpdateStreamDistributionProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    #[serde(alias = "streamID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub stream_id: u64,
    #[prost(message, repeated, tag = "4")]
    pub records: ::prost::alloc::vec::Vec<DistrRecord>,
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
#[proto_message(type_url = "/dymensionxyz.dymension.streamer.CreateStreamProposal")]
pub struct CreateStreamProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub distribute_to_records: ::prost::alloc::vec::Vec<DistrRecord>,
    /// coins are coin(s) to be distributed by the stream
    #[prost(message, repeated, tag = "4")]
    pub coins: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// start_time is the distribution start time
    #[prost(message, optional, tag = "5")]
    pub start_time: ::core::option::Option<crate::shim::Timestamp>,
    #[prost(string, tag = "6")]
    #[serde(alias = "distr_epochIDentifier")]
    pub distr_epoch_identifier: ::prost::alloc::string::String,
    /// num_epochs_paid_over is the number of epochs distribution will be completed
    /// over
    #[prost(uint64, tag = "7")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub num_epochs_paid_over: u64,
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
#[proto_message(type_url = "/dymensionxyz.dymension.streamer.TerminateStreamProposal")]
pub struct TerminateStreamProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(uint64, tag = "4")]
    #[serde(alias = "streamID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub stream_id: u64,
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
#[proto_message(type_url = "/dymensionxyz.dymension.streamer.ModuleToDistributeCoinsRequest")]
#[proto_query(
    path = "/dymensionxyz.dymension.streamer.Query/ModuleToDistributeCoins",
    response_type = ModuleToDistributeCoinsResponse
)]
pub struct ModuleToDistributeCoinsRequest {}
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
#[proto_message(type_url = "/dymensionxyz.dymension.streamer.ModuleToDistributeCoinsResponse")]
pub struct ModuleToDistributeCoinsResponse {
    /// Coins that have yet to be distributed
    #[prost(message, repeated, tag = "1")]
    pub coins: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
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
#[proto_message(type_url = "/dymensionxyz.dymension.streamer.StreamByIDRequest")]
#[proto_query(
    path = "/dymensionxyz.dymension.streamer.Query/StreamByID",
    response_type = StreamByIdResponse
)]
pub struct StreamByIdRequest {
    /// Gague ID being queried
    #[prost(uint64, tag = "1")]
    #[serde(alias = "ID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub id: u64,
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
#[proto_message(type_url = "/dymensionxyz.dymension.streamer.StreamByIDResponse")]
pub struct StreamByIdResponse {
    /// Stream that corresponds to provided gague ID
    #[prost(message, optional, tag = "1")]
    pub stream: ::core::option::Option<Stream>,
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
#[proto_message(type_url = "/dymensionxyz.dymension.streamer.StreamsRequest")]
#[proto_query(
    path = "/dymensionxyz.dymension.streamer.Query/Streams",
    response_type = StreamsResponse
)]
pub struct StreamsRequest {
    /// Pagination defines pagination for the request
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
#[proto_message(type_url = "/dymensionxyz.dymension.streamer.StreamsResponse")]
pub struct StreamsResponse {
    /// Upcoming and active streams
    #[prost(message, repeated, tag = "1")]
    pub data: ::prost::alloc::vec::Vec<Stream>,
    /// Pagination defines pagination for the response
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
#[proto_message(type_url = "/dymensionxyz.dymension.streamer.ActiveStreamsRequest")]
#[proto_query(
    path = "/dymensionxyz.dymension.streamer.Query/ActiveStreams",
    response_type = ActiveStreamsResponse
)]
pub struct ActiveStreamsRequest {
    /// Pagination defines pagination for the request
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
#[proto_message(type_url = "/dymensionxyz.dymension.streamer.ActiveStreamsResponse")]
pub struct ActiveStreamsResponse {
    /// Active gagues only
    #[prost(message, repeated, tag = "1")]
    pub data: ::prost::alloc::vec::Vec<Stream>,
    /// Pagination defines pagination for the response
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
#[proto_message(type_url = "/dymensionxyz.dymension.streamer.UpcomingStreamsRequest")]
#[proto_query(
    path = "/dymensionxyz.dymension.streamer.Query/UpcomingStreams",
    response_type = UpcomingStreamsResponse
)]
pub struct UpcomingStreamsRequest {
    /// Pagination defines pagination for the request
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
#[proto_message(type_url = "/dymensionxyz.dymension.streamer.UpcomingStreamsResponse")]
pub struct UpcomingStreamsResponse {
    /// Streams whose distribution is upcoming
    #[prost(message, repeated, tag = "1")]
    pub data: ::prost::alloc::vec::Vec<Stream>,
    /// Pagination defines pagination for the response
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
pub struct StreamerQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> StreamerQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn module_to_distribute_coins(
        &self,
    ) -> Result<ModuleToDistributeCoinsResponse, cosmwasm_std::StdError> {
        ModuleToDistributeCoinsRequest {}.query(self.querier)
    }
    pub fn stream_by_id(&self, id: u64) -> Result<StreamByIdResponse, cosmwasm_std::StdError> {
        StreamByIdRequest { id }.query(self.querier)
    }
    pub fn streams(
        &self,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<StreamsResponse, cosmwasm_std::StdError> {
        StreamsRequest { pagination }.query(self.querier)
    }
    pub fn active_streams(
        &self,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<ActiveStreamsResponse, cosmwasm_std::StdError> {
        ActiveStreamsRequest { pagination }.query(self.querier)
    }
    pub fn upcoming_streams(
        &self,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<UpcomingStreamsResponse, cosmwasm_std::StdError> {
        UpcomingStreamsRequest { pagination }.query(self.querier)
    }
}
