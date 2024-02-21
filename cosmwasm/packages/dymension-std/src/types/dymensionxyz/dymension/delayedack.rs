use dymension_std_derive::CosmwasmExt;
/// GenesisState defines the delayedack module's genesis state.
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
#[proto_message(type_url = "/dymensionxyz.dymension.delayedack.GenesisState")]
pub struct GenesisState {}
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
#[proto_message(type_url = "/dymensionxyz.dymension.delayedack.RollappPacket")]
pub struct RollappPacket {
    #[prost(message, optional, tag = "1")]
    pub packet: ::core::option::Option<super::super::super::ibc::core::channel::v1::Packet>,
    #[prost(enumeration = "rollapp_packet::Status", tag = "2")]
    #[serde(with = "rollapp_packet::Status")]
    pub status: i32,
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub proof_height: u64,
    #[prost(string, tag = "4")]
    pub error: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub relayer: ::prost::alloc::vec::Vec<u8>,
}
/// Nested message and enum types in `RollappPacket`.
pub mod rollapp_packet {
    use dymension_std_derive::CosmwasmExt;
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    #[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
    pub enum Status {
        Pending = 0,
        Accepted = 1,
        Rejected = 2,
    }
    impl Status {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Status::Pending => "PENDING",
                Status::Accepted => "ACCEPTED",
                Status::Rejected => "REJECTED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PENDING" => Some(Self::Pending),
                "ACCEPTED" => Some(Self::Accepted),
                "REJECTED" => Some(Self::Rejected),
                _ => None,
            }
        }
    }
    impl Status {
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
}
