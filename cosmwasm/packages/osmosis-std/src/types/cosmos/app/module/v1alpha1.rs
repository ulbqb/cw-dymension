use osmosis_std_derive::CosmwasmExt;
/// Module is the module config object for the cosmos.app v1 app module.
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
#[proto_message(type_url = "/cosmos.app.module.v1alpha1.Module")]
pub struct Module {}
