use dymension_std_derive::CosmwasmExt;
/// BaseAccount defines a base account type. It contains all the necessary fields
/// for basic account functionality. Any custom account type should extend this
/// type for additional functionality (e.g. vesting).
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
#[proto_message(type_url = "/cosmos.auth.v1beta1.BaseAccount")]
pub struct BaseAccount {
    #[prost(string, tag = "1")]
    #[serde(default)]
    pub address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    #[serde(default)]
    pub pub_key: ::core::option::Option<crate::shim::Any>,
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    #[serde(default)]
    pub account_number: u64,
    #[prost(uint64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    #[serde(default)]
    pub sequence: u64,
}
/// ModuleAccount defines an account for modules that holds coins on a pool.
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
#[proto_message(type_url = "/cosmos.auth.v1beta1.ModuleAccount")]
pub struct ModuleAccount {
    #[prost(message, optional, tag = "1")]
    #[serde(default)]
    pub base_account: ::core::option::Option<BaseAccount>,
    #[prost(string, tag = "2")]
    #[serde(default)]
    pub name: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    #[serde(default)]
    pub permissions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Params defines the parameters for the auth module.
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
#[proto_message(type_url = "/cosmos.auth.v1beta1.Params")]
pub struct Params {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    #[serde(default)]
    pub max_memo_characters: u64,
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    #[serde(default)]
    pub tx_sig_limit: u64,
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    #[serde(default)]
    pub tx_size_cost_per_byte: u64,
    #[prost(uint64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    #[serde(default)]
    pub sig_verify_cost_ed25519: u64,
    #[prost(uint64, tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    #[serde(default)]
    pub sig_verify_cost_secp256k1: u64,
}
/// GenesisState defines the auth module's genesis state.
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
#[proto_message(type_url = "/cosmos.auth.v1beta1.GenesisState")]
pub struct GenesisState {
    /// params defines all the paramaters of the module.
    #[prost(message, optional, tag = "1")]
    #[serde(default)]
    pub params: ::core::option::Option<Params>,
    /// accounts are the accounts present at genesis.
    #[prost(message, repeated, tag = "2")]
    #[serde(default)]
    pub accounts: ::prost::alloc::vec::Vec<crate::shim::Any>,
}
/// QueryAccountsRequest is the request type for the Query/Accounts RPC method.
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
#[proto_message(type_url = "/cosmos.auth.v1beta1.QueryAccountsRequest")]
#[proto_query(
    path = "/cosmos.auth.v1beta1.Query/Accounts",
    response_type = QueryAccountsResponse
)]
pub struct QueryAccountsRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "1")]
    #[serde(default)]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QueryAccountsResponse is the response type for the Query/Accounts RPC method.
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
#[proto_message(type_url = "/cosmos.auth.v1beta1.QueryAccountsResponse")]
pub struct QueryAccountsResponse {
    /// accounts are the existing accounts
    #[prost(message, repeated, tag = "1")]
    #[serde(default)]
    pub accounts: ::prost::alloc::vec::Vec<crate::shim::Any>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    #[serde(default)]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// QueryAccountRequest is the request type for the Query/Account RPC method.
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
#[proto_message(type_url = "/cosmos.auth.v1beta1.QueryAccountRequest")]
#[proto_query(
    path = "/cosmos.auth.v1beta1.Query/Account",
    response_type = QueryAccountResponse
)]
pub struct QueryAccountRequest {
    /// address defines the address to query for.
    #[prost(string, tag = "1")]
    #[serde(default)]
    pub address: ::prost::alloc::string::String,
}
/// QueryAccountResponse is the response type for the Query/Account RPC method.
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
#[proto_message(type_url = "/cosmos.auth.v1beta1.QueryAccountResponse")]
pub struct QueryAccountResponse {
    /// account defines the account of the corresponding address.
    #[prost(message, optional, tag = "1")]
    #[serde(default)]
    pub account: ::core::option::Option<crate::shim::Any>,
}
/// QueryParamsRequest is the request type for the Query/Params RPC method.
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
#[proto_message(type_url = "/cosmos.auth.v1beta1.QueryParamsRequest")]
#[proto_query(
    path = "/cosmos.auth.v1beta1.Query/Params",
    response_type = QueryParamsResponse
)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is the response type for the Query/Params RPC method.
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
#[proto_message(type_url = "/cosmos.auth.v1beta1.QueryParamsResponse")]
pub struct QueryParamsResponse {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag = "1")]
    #[serde(default)]
    pub params: ::core::option::Option<Params>,
}
/// QueryModuleAccountsRequest is the request type for the Query/ModuleAccounts RPC method.
///
/// Since: cosmos-sdk 0.46
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
#[proto_message(type_url = "/cosmos.auth.v1beta1.QueryModuleAccountsRequest")]
#[proto_query(
    path = "/cosmos.auth.v1beta1.Query/ModuleAccounts",
    response_type = QueryModuleAccountsResponse
)]
pub struct QueryModuleAccountsRequest {}
/// QueryModuleAccountsResponse is the response type for the Query/ModuleAccounts RPC method.
///
/// Since: cosmos-sdk 0.46
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
#[proto_message(type_url = "/cosmos.auth.v1beta1.QueryModuleAccountsResponse")]
pub struct QueryModuleAccountsResponse {
    #[prost(message, repeated, tag = "1")]
    #[serde(default)]
    pub accounts: ::prost::alloc::vec::Vec<crate::shim::Any>,
}
/// QueryModuleAccountByNameRequest is the request type for the Query/ModuleAccountByName RPC method.
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
#[proto_message(type_url = "/cosmos.auth.v1beta1.QueryModuleAccountByNameRequest")]
#[proto_query(
    path = "/cosmos.auth.v1beta1.Query/ModuleAccountByName",
    response_type = QueryModuleAccountByNameResponse
)]
pub struct QueryModuleAccountByNameRequest {
    #[prost(string, tag = "1")]
    #[serde(default)]
    pub name: ::prost::alloc::string::String,
}
/// QueryModuleAccountByNameResponse is the response type for the Query/ModuleAccountByName RPC method.
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
#[proto_message(type_url = "/cosmos.auth.v1beta1.QueryModuleAccountByNameResponse")]
pub struct QueryModuleAccountByNameResponse {
    #[prost(message, optional, tag = "1")]
    #[serde(default)]
    pub account: ::core::option::Option<crate::shim::Any>,
}
/// Bech32PrefixRequest is the request type for Bech32Prefix rpc method.
///
/// Since: cosmos-sdk 0.46
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
#[proto_message(type_url = "/cosmos.auth.v1beta1.Bech32PrefixRequest")]
#[proto_query(
    path = "/cosmos.auth.v1beta1.Query/Bech32Prefix",
    response_type = Bech32PrefixResponse
)]
pub struct Bech32PrefixRequest {}
/// Bech32PrefixResponse is the response type for Bech32Prefix rpc method.
///
/// Since: cosmos-sdk 0.46
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
#[proto_message(type_url = "/cosmos.auth.v1beta1.Bech32PrefixResponse")]
pub struct Bech32PrefixResponse {
    #[prost(string, tag = "1")]
    #[serde(default)]
    pub bech32_prefix: ::prost::alloc::string::String,
}
/// AddressBytesToStringRequest is the request type for AddressString rpc method.
///
/// Since: cosmos-sdk 0.46
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
#[proto_message(type_url = "/cosmos.auth.v1beta1.AddressBytesToStringRequest")]
#[proto_query(
    path = "/cosmos.auth.v1beta1.Query/AddressBytesToString",
    response_type = AddressBytesToStringResponse
)]
pub struct AddressBytesToStringRequest {
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    #[serde(default)]
    pub address_bytes: ::prost::alloc::vec::Vec<u8>,
}
/// AddressBytesToStringResponse is the response type for AddressString rpc method.
///
/// Since: cosmos-sdk 0.46
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
#[proto_message(type_url = "/cosmos.auth.v1beta1.AddressBytesToStringResponse")]
pub struct AddressBytesToStringResponse {
    #[prost(string, tag = "1")]
    #[serde(default)]
    pub address_string: ::prost::alloc::string::String,
}
/// AddressStringToBytesRequest is the request type for AccountBytes rpc method.
///
/// Since: cosmos-sdk 0.46
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
#[proto_message(type_url = "/cosmos.auth.v1beta1.AddressStringToBytesRequest")]
#[proto_query(
    path = "/cosmos.auth.v1beta1.Query/AddressStringToBytes",
    response_type = AddressStringToBytesResponse
)]
pub struct AddressStringToBytesRequest {
    #[prost(string, tag = "1")]
    #[serde(default)]
    pub address_string: ::prost::alloc::string::String,
}
/// AddressStringToBytesResponse is the response type for AddressBytes rpc method.
///
/// Since: cosmos-sdk 0.46
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
#[proto_message(type_url = "/cosmos.auth.v1beta1.AddressStringToBytesResponse")]
pub struct AddressStringToBytesResponse {
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    #[serde(default)]
    pub address_bytes: ::prost::alloc::vec::Vec<u8>,
}
/// QueryAccountAddressByIDRequest is the request type for AccountAddressByID rpc method
///
/// Since: cosmos-sdk 0.46.2
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
#[proto_message(type_url = "/cosmos.auth.v1beta1.QueryAccountAddressByIDRequest")]
#[proto_query(
    path = "/cosmos.auth.v1beta1.Query/AccountAddressByID",
    response_type = QueryAccountAddressByIdResponse
)]
pub struct QueryAccountAddressByIdRequest {
    /// id is the account number of the address to be queried. This field
    /// should have been an uint64 (like all account numbers), and will be
    /// updated to uint64 in a future version of the auth query.
    #[prost(int64, tag = "1")]
    #[serde(alias = "ID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    #[serde(default)]
    pub id: i64,
}
/// QueryAccountAddressByIDResponse is the response type for AccountAddressByID rpc method
///
/// Since: cosmos-sdk 0.46.2
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
#[proto_message(type_url = "/cosmos.auth.v1beta1.QueryAccountAddressByIDResponse")]
pub struct QueryAccountAddressByIdResponse {
    #[prost(string, tag = "1")]
    #[serde(default)]
    pub account_address: ::prost::alloc::string::String,
}
pub struct AuthQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> AuthQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn accounts(
        &self,
        pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
    ) -> Result<QueryAccountsResponse, cosmwasm_std::StdError> {
        QueryAccountsRequest { pagination }.query(self.querier)
    }
    pub fn account(
        &self,
        address: ::prost::alloc::string::String,
    ) -> Result<QueryAccountResponse, cosmwasm_std::StdError> {
        QueryAccountRequest { address }.query(self.querier)
    }
    pub fn account_address_by_id(
        &self,
        id: i64,
    ) -> Result<QueryAccountAddressByIdResponse, cosmwasm_std::StdError> {
        QueryAccountAddressByIdRequest { id }.query(self.querier)
    }
    pub fn params(&self) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
    pub fn module_accounts(&self) -> Result<QueryModuleAccountsResponse, cosmwasm_std::StdError> {
        QueryModuleAccountsRequest {}.query(self.querier)
    }
    pub fn module_account_by_name(
        &self,
        name: ::prost::alloc::string::String,
    ) -> Result<QueryModuleAccountByNameResponse, cosmwasm_std::StdError> {
        QueryModuleAccountByNameRequest { name }.query(self.querier)
    }
    pub fn bech32_prefix(&self) -> Result<Bech32PrefixResponse, cosmwasm_std::StdError> {
        Bech32PrefixRequest {}.query(self.querier)
    }
    pub fn address_bytes_to_string(
        &self,
        address_bytes: ::prost::alloc::vec::Vec<u8>,
    ) -> Result<AddressBytesToStringResponse, cosmwasm_std::StdError> {
        AddressBytesToStringRequest { address_bytes }.query(self.querier)
    }
    pub fn address_string_to_bytes(
        &self,
        address_string: ::prost::alloc::string::String,
    ) -> Result<AddressStringToBytesResponse, cosmwasm_std::StdError> {
        AddressStringToBytesRequest { address_string }.query(self.querier)
    }
}
