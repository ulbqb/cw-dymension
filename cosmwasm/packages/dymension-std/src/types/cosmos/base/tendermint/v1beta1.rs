use dymension_std_derive::CosmwasmExt;
/// Block is tendermint type Block, with the Header proposer address
/// field converted to bech32 string.
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
#[proto_message(type_url = "/cosmos.base.tendermint.v1beta1.Block")]
pub struct Block {
    #[prost(message, optional, tag = "1")]
    #[serde(default)]
    pub header: ::core::option::Option<Header>,
    #[prost(message, optional, tag = "2")]
    #[serde(default)]
    pub data: ::core::option::Option<super::super::super::super::tendermint::types::Data>,
    #[prost(message, optional, tag = "3")]
    #[serde(default)]
    pub evidence:
        ::core::option::Option<super::super::super::super::tendermint::types::EvidenceList>,
    #[prost(message, optional, tag = "4")]
    #[serde(default)]
    pub last_commit: ::core::option::Option<super::super::super::super::tendermint::types::Commit>,
}
/// Header defines the structure of a Tendermint block header.
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
#[proto_message(type_url = "/cosmos.base.tendermint.v1beta1.Header")]
pub struct Header {
    /// basic block info
    #[prost(message, optional, tag = "1")]
    #[serde(default)]
    pub version: ::core::option::Option<super::super::super::super::tendermint::version::Consensus>,
    #[prost(string, tag = "2")]
    #[serde(alias = "chainID")]
    #[serde(default)]
    pub chain_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    #[serde(default)]
    pub height: i64,
    #[prost(message, optional, tag = "4")]
    #[serde(default)]
    pub time: ::core::option::Option<crate::shim::Timestamp>,
    /// prev block info
    #[prost(message, optional, tag = "5")]
    #[serde(alias = "last_blockID")]
    #[serde(default)]
    pub last_block_id:
        ::core::option::Option<super::super::super::super::tendermint::types::BlockId>,
    /// hashes of block data
    ///
    /// commit from validators from the last block
    #[prost(bytes = "vec", tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    #[serde(default)]
    pub last_commit_hash: ::prost::alloc::vec::Vec<u8>,
    /// transactions
    #[prost(bytes = "vec", tag = "7")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    #[serde(default)]
    pub data_hash: ::prost::alloc::vec::Vec<u8>,
    /// hashes from the app output from the prev block
    ///
    /// validators for the current block
    #[prost(bytes = "vec", tag = "8")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    #[serde(default)]
    pub validators_hash: ::prost::alloc::vec::Vec<u8>,
    /// validators for the next block
    #[prost(bytes = "vec", tag = "9")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    #[serde(default)]
    pub next_validators_hash: ::prost::alloc::vec::Vec<u8>,
    /// consensus params for current block
    #[prost(bytes = "vec", tag = "10")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    #[serde(default)]
    pub consensus_hash: ::prost::alloc::vec::Vec<u8>,
    /// state after txs from the previous block
    #[prost(bytes = "vec", tag = "11")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    #[serde(default)]
    pub app_hash: ::prost::alloc::vec::Vec<u8>,
    /// root hash of all results from the txs from the previous block
    #[prost(bytes = "vec", tag = "12")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    #[serde(default)]
    pub last_results_hash: ::prost::alloc::vec::Vec<u8>,
    /// consensus info
    ///
    /// evidence included in the block
    #[prost(bytes = "vec", tag = "13")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    #[serde(default)]
    pub evidence_hash: ::prost::alloc::vec::Vec<u8>,
    /// proposer_address is the original block proposer address, formatted as a Bech32 string.
    /// In Tendermint, this type is `bytes`, but in the SDK, we convert it to a Bech32 string
    /// for better UX.
    ///
    /// original proposer of the block
    #[prost(string, tag = "14")]
    #[serde(default)]
    pub proposer_address: ::prost::alloc::string::String,
}
/// GetValidatorSetByHeightRequest is the request type for the
/// Query/GetValidatorSetByHeight RPC method.
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
#[proto_message(type_url = "/cosmos.base.tendermint.v1beta1.GetValidatorSetByHeightRequest")]
pub struct GetValidatorSetByHeightRequest {
    #[prost(int64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    #[serde(default)]
    pub height: i64,
    /// pagination defines an pagination for the request.
    #[prost(message, optional, tag = "2")]
    #[serde(default)]
    pub pagination: ::core::option::Option<super::super::query::v1beta1::PageRequest>,
}
/// GetValidatorSetByHeightResponse is the response type for the
/// Query/GetValidatorSetByHeight RPC method.
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
#[proto_message(type_url = "/cosmos.base.tendermint.v1beta1.GetValidatorSetByHeightResponse")]
pub struct GetValidatorSetByHeightResponse {
    #[prost(int64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    #[serde(default)]
    pub block_height: i64,
    #[prost(message, repeated, tag = "2")]
    #[serde(default)]
    pub validators: ::prost::alloc::vec::Vec<Validator>,
    /// pagination defines an pagination for the response.
    #[prost(message, optional, tag = "3")]
    #[serde(default)]
    pub pagination: ::core::option::Option<super::super::query::v1beta1::PageResponse>,
}
/// GetLatestValidatorSetRequest is the request type for the
/// Query/GetValidatorSetByHeight RPC method.
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
#[proto_message(type_url = "/cosmos.base.tendermint.v1beta1.GetLatestValidatorSetRequest")]
pub struct GetLatestValidatorSetRequest {
    /// pagination defines an pagination for the request.
    #[prost(message, optional, tag = "1")]
    #[serde(default)]
    pub pagination: ::core::option::Option<super::super::query::v1beta1::PageRequest>,
}
/// GetLatestValidatorSetResponse is the response type for the
/// Query/GetValidatorSetByHeight RPC method.
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
#[proto_message(type_url = "/cosmos.base.tendermint.v1beta1.GetLatestValidatorSetResponse")]
pub struct GetLatestValidatorSetResponse {
    #[prost(int64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    #[serde(default)]
    pub block_height: i64,
    #[prost(message, repeated, tag = "2")]
    #[serde(default)]
    pub validators: ::prost::alloc::vec::Vec<Validator>,
    /// pagination defines an pagination for the response.
    #[prost(message, optional, tag = "3")]
    #[serde(default)]
    pub pagination: ::core::option::Option<super::super::query::v1beta1::PageResponse>,
}
/// Validator is the type for the validator-set.
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
#[proto_message(type_url = "/cosmos.base.tendermint.v1beta1.Validator")]
pub struct Validator {
    #[prost(string, tag = "1")]
    #[serde(default)]
    pub address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    #[serde(default)]
    pub pub_key: ::core::option::Option<crate::shim::Any>,
    #[prost(int64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    #[serde(default)]
    pub voting_power: i64,
    #[prost(int64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    #[serde(default)]
    pub proposer_priority: i64,
}
/// GetBlockByHeightRequest is the request type for the Query/GetBlockByHeight
/// RPC method.
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
#[proto_message(type_url = "/cosmos.base.tendermint.v1beta1.GetBlockByHeightRequest")]
pub struct GetBlockByHeightRequest {
    #[prost(int64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    #[serde(default)]
    pub height: i64,
}
/// GetBlockByHeightResponse is the response type for the Query/GetBlockByHeight
/// RPC method.
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
#[proto_message(type_url = "/cosmos.base.tendermint.v1beta1.GetBlockByHeightResponse")]
pub struct GetBlockByHeightResponse {
    #[prost(message, optional, tag = "1")]
    #[serde(alias = "blockID")]
    #[serde(default)]
    pub block_id: ::core::option::Option<super::super::super::super::tendermint::types::BlockId>,
    /// Deprecated: please use `sdk_block` instead
    #[prost(message, optional, tag = "2")]
    #[serde(default)]
    pub block: ::core::option::Option<super::super::super::super::tendermint::types::Block>,
    /// Since: cosmos-sdk 0.47
    #[prost(message, optional, tag = "3")]
    #[serde(default)]
    pub sdk_block: ::core::option::Option<Block>,
}
/// GetLatestBlockRequest is the request type for the Query/GetLatestBlock RPC
/// method.
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
#[proto_message(type_url = "/cosmos.base.tendermint.v1beta1.GetLatestBlockRequest")]
pub struct GetLatestBlockRequest {}
/// GetLatestBlockResponse is the response type for the Query/GetLatestBlock RPC
/// method.
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
#[proto_message(type_url = "/cosmos.base.tendermint.v1beta1.GetLatestBlockResponse")]
pub struct GetLatestBlockResponse {
    #[prost(message, optional, tag = "1")]
    #[serde(alias = "blockID")]
    #[serde(default)]
    pub block_id: ::core::option::Option<super::super::super::super::tendermint::types::BlockId>,
    /// Deprecated: please use `sdk_block` instead
    #[prost(message, optional, tag = "2")]
    #[serde(default)]
    pub block: ::core::option::Option<super::super::super::super::tendermint::types::Block>,
    /// Since: cosmos-sdk 0.47
    #[prost(message, optional, tag = "3")]
    #[serde(default)]
    pub sdk_block: ::core::option::Option<Block>,
}
/// GetSyncingRequest is the request type for the Query/GetSyncing RPC method.
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
#[proto_message(type_url = "/cosmos.base.tendermint.v1beta1.GetSyncingRequest")]
pub struct GetSyncingRequest {}
/// GetSyncingResponse is the response type for the Query/GetSyncing RPC method.
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
#[proto_message(type_url = "/cosmos.base.tendermint.v1beta1.GetSyncingResponse")]
pub struct GetSyncingResponse {
    #[prost(bool, tag = "1")]
    #[serde(default)]
    pub syncing: bool,
}
/// GetNodeInfoRequest is the request type for the Query/GetNodeInfo RPC method.
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
#[proto_message(type_url = "/cosmos.base.tendermint.v1beta1.GetNodeInfoRequest")]
pub struct GetNodeInfoRequest {}
/// GetNodeInfoResponse is the response type for the Query/GetNodeInfo RPC
/// method.
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
#[proto_message(type_url = "/cosmos.base.tendermint.v1beta1.GetNodeInfoResponse")]
pub struct GetNodeInfoResponse {
    #[prost(message, optional, tag = "1")]
    #[serde(default)]
    pub default_node_info:
        ::core::option::Option<super::super::super::super::tendermint::p2p::DefaultNodeInfo>,
    #[prost(message, optional, tag = "2")]
    #[serde(default)]
    pub application_version: ::core::option::Option<VersionInfo>,
}
/// VersionInfo is the type for the GetNodeInfoResponse message.
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
#[proto_message(type_url = "/cosmos.base.tendermint.v1beta1.VersionInfo")]
pub struct VersionInfo {
    #[prost(string, tag = "1")]
    #[serde(default)]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    #[serde(default)]
    pub app_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    #[serde(default)]
    pub version: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    #[serde(default)]
    pub git_commit: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    #[serde(default)]
    pub build_tags: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    #[serde(default)]
    pub go_version: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "7")]
    #[serde(default)]
    pub build_deps: ::prost::alloc::vec::Vec<Module>,
    /// Since: cosmos-sdk 0.43
    #[prost(string, tag = "8")]
    #[serde(default)]
    pub cosmos_sdk_version: ::prost::alloc::string::String,
}
/// Module is the type for VersionInfo
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
#[proto_message(type_url = "/cosmos.base.tendermint.v1beta1.Module")]
pub struct Module {
    /// module path
    #[prost(string, tag = "1")]
    #[serde(default)]
    pub path: ::prost::alloc::string::String,
    /// module version
    #[prost(string, tag = "2")]
    #[serde(default)]
    pub version: ::prost::alloc::string::String,
    /// checksum
    #[prost(string, tag = "3")]
    #[serde(default)]
    pub sum: ::prost::alloc::string::String,
}
/// ABCIQueryRequest defines the request structure for the ABCIQuery gRPC query.
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
#[proto_message(type_url = "/cosmos.base.tendermint.v1beta1.ABCIQueryRequest")]
pub struct AbciQueryRequest {
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    #[serde(default)]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "2")]
    #[serde(default)]
    pub path: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    #[serde(default)]
    pub height: i64,
    #[prost(bool, tag = "4")]
    #[serde(default)]
    pub prove: bool,
}
/// ABCIQueryResponse defines the response structure for the ABCIQuery gRPC
/// query.
///
/// Note: This type is a duplicate of the ResponseQuery proto type defined in
/// Tendermint.
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
#[proto_message(type_url = "/cosmos.base.tendermint.v1beta1.ABCIQueryResponse")]
pub struct AbciQueryResponse {
    #[prost(uint32, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    #[serde(default)]
    pub code: u32,
    /// nondeterministic
    #[prost(string, tag = "3")]
    #[serde(default)]
    pub log: ::prost::alloc::string::String,
    /// nondeterministic
    #[prost(string, tag = "4")]
    #[serde(default)]
    pub info: ::prost::alloc::string::String,
    #[prost(int64, tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    #[serde(default)]
    pub index: i64,
    #[prost(bytes = "vec", tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    #[serde(default)]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "7")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    #[serde(default)]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "8")]
    #[serde(default)]
    pub proof_ops: ::core::option::Option<ProofOps>,
    #[prost(int64, tag = "9")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    #[serde(default)]
    pub height: i64,
    #[prost(string, tag = "10")]
    #[serde(default)]
    pub codespace: ::prost::alloc::string::String,
}
/// ProofOp defines an operation used for calculating Merkle root. The data could
/// be arbitrary format, providing nessecary data for example neighbouring node
/// hash.
///
/// Note: This type is a duplicate of the ProofOp proto type defined in
/// Tendermint.
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
#[proto_message(type_url = "/cosmos.base.tendermint.v1beta1.ProofOp")]
pub struct ProofOp {
    #[prost(string, tag = "1")]
    #[serde(default)]
    pub r#type: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    #[serde(default)]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    #[serde(default)]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// ProofOps is Merkle proof defined by the list of ProofOps.
///
/// Note: This type is a duplicate of the ProofOps proto type defined in
/// Tendermint.
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
#[proto_message(type_url = "/cosmos.base.tendermint.v1beta1.ProofOps")]
pub struct ProofOps {
    #[prost(message, repeated, tag = "1")]
    #[serde(default)]
    pub ops: ::prost::alloc::vec::Vec<ProofOp>,
}
