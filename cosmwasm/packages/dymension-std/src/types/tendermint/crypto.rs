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
#[proto_message(type_url = "/tendermint.crypto.Proof")]
pub struct Proof {
    #[prost(int64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    #[serde(default)]
    pub total: i64,
    #[prost(int64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    #[serde(default)]
    pub index: i64,
    #[prost(bytes = "vec", tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    #[serde(default)]
    pub leaf_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", repeated, tag = "4")]
    #[serde(default)]
    pub aunts: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
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
#[proto_message(type_url = "/tendermint.crypto.ValueOp")]
pub struct ValueOp {
    /// Encoded in ProofOp.Key.
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    #[serde(default)]
    pub key: ::prost::alloc::vec::Vec<u8>,
    /// To encode in ProofOp.Data
    #[prost(message, optional, tag = "2")]
    #[serde(default)]
    pub proof: ::core::option::Option<Proof>,
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
#[proto_message(type_url = "/tendermint.crypto.DominoOp")]
pub struct DominoOp {
    #[prost(string, tag = "1")]
    #[serde(default)]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    #[serde(default)]
    pub input: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    #[serde(default)]
    pub output: ::prost::alloc::string::String,
}
/// ProofOp defines an operation used for calculating Merkle root
/// The data could be arbitrary format, providing nessecary data
/// for example neighbouring node hash
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
#[proto_message(type_url = "/tendermint.crypto.ProofOp")]
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
/// ProofOps is Merkle proof defined by the list of ProofOps
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
#[proto_message(type_url = "/tendermint.crypto.ProofOps")]
pub struct ProofOps {
    #[prost(message, repeated, tag = "1")]
    #[serde(default)]
    pub ops: ::prost::alloc::vec::Vec<ProofOp>,
}
/// PublicKey defines the keys available for use with Validators
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
#[proto_message(type_url = "/tendermint.crypto.PublicKey")]
pub struct PublicKey {
    #[prost(oneof = "public_key::Sum", tags = "1, 2")]
    #[serde(default)]
    pub sum: ::core::option::Option<public_key::Sum>,
}
/// Nested message and enum types in `PublicKey`.
pub mod public_key {
    use dymension_std_derive::CosmwasmExt;
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(
        Clone,
        PartialEq,
        Eq,
        ::prost::Oneof,
        ::serde::Serialize,
        ::serde::Deserialize,
        ::schemars::JsonSchema,
    )]
    pub enum Sum {
        #[prost(bytes, tag = "1")]
        Ed25519(::prost::alloc::vec::Vec<u8>),
        #[prost(bytes, tag = "2")]
        Secp256k1(::prost::alloc::vec::Vec<u8>),
    }
}
