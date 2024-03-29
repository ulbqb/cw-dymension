use dymension_std_derive::CosmwasmExt;
/// CommitInfo defines commit information used by the multi-store when committing
/// a version/height.
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
#[proto_message(type_url = "/cosmos.base.store.v1beta1.CommitInfo")]
pub struct CommitInfo {
    #[prost(int64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    #[serde(default)]
    pub version: i64,
    #[prost(message, repeated, tag = "2")]
    #[serde(default)]
    pub store_infos: ::prost::alloc::vec::Vec<StoreInfo>,
}
/// StoreInfo defines store-specific commit information. It contains a reference
/// between a store name and the commit ID.
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
#[proto_message(type_url = "/cosmos.base.store.v1beta1.StoreInfo")]
pub struct StoreInfo {
    #[prost(string, tag = "1")]
    #[serde(default)]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    #[serde(alias = "commitID")]
    #[serde(default)]
    pub commit_id: ::core::option::Option<CommitId>,
}
/// CommitID defines the committment information when a specific store is
/// committed.
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
#[proto_message(type_url = "/cosmos.base.store.v1beta1.CommitID")]
pub struct CommitId {
    #[prost(int64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    #[serde(default)]
    pub version: i64,
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    #[serde(default)]
    pub hash: ::prost::alloc::vec::Vec<u8>,
}
/// StoreKVPair is a KVStore KVPair used for listening to state changes (Sets and Deletes)
/// It optionally includes the StoreKey for the originating KVStore and a Boolean flag to distinguish between Sets and
/// Deletes
///
/// Since: cosmos-sdk 0.43
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
#[proto_message(type_url = "/cosmos.base.store.v1beta1.StoreKVPair")]
pub struct StoreKvPair {
    /// the store key for the KVStore this pair originates from
    #[prost(string, tag = "1")]
    #[serde(default)]
    pub store_key: ::prost::alloc::string::String,
    /// true indicates a delete operation, false indicates a set operation
    #[prost(bool, tag = "2")]
    #[serde(default)]
    pub delete: bool,
    #[prost(bytes = "vec", tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    #[serde(default)]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    #[serde(default)]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
/// BlockMetadata contains all the abci event data of a block
/// the file streamer dump them into files together with the state changes.
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
#[proto_message(type_url = "/cosmos.base.store.v1beta1.BlockMetadata")]
pub struct BlockMetadata {
    #[prost(message, optional, tag = "1")]
    #[serde(default)]
    pub request_begin_block:
        ::core::option::Option<super::super::super::super::tendermint::abci::RequestBeginBlock>,
    #[prost(message, optional, tag = "2")]
    #[serde(default)]
    pub response_begin_block:
        ::core::option::Option<super::super::super::super::tendermint::abci::ResponseBeginBlock>,
    #[prost(message, repeated, tag = "3")]
    #[serde(default)]
    pub deliver_txs: ::prost::alloc::vec::Vec<block_metadata::DeliverTx>,
    #[prost(message, optional, tag = "4")]
    #[serde(default)]
    pub request_end_block:
        ::core::option::Option<super::super::super::super::tendermint::abci::RequestEndBlock>,
    #[prost(message, optional, tag = "5")]
    #[serde(default)]
    pub response_end_block:
        ::core::option::Option<super::super::super::super::tendermint::abci::ResponseEndBlock>,
    #[prost(message, optional, tag = "6")]
    #[serde(default)]
    pub response_commit:
        ::core::option::Option<super::super::super::super::tendermint::abci::ResponseCommit>,
}
/// Nested message and enum types in `BlockMetadata`.
pub mod block_metadata {
    use dymension_std_derive::CosmwasmExt;
    /// DeliverTx encapulate deliver tx request and response.
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
    #[proto_message(type_url = "/cosmos.base.store.v1beta1.BlockMetadata.DeliverTx")]
    pub struct DeliverTx {
        #[prost(message, optional, tag = "1")]
        #[serde(default)]
        pub request: ::core::option::Option<
            super::super::super::super::super::tendermint::abci::RequestDeliverTx,
        >,
        #[prost(message, optional, tag = "2")]
        #[serde(default)]
        pub response: ::core::option::Option<
            super::super::super::super::super::tendermint::abci::ResponseDeliverTx,
        >,
    }
}
