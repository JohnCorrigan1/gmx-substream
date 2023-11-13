// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Things {
    #[prost(message, repeated, tag="1")]
    pub things: ::prost::alloc::vec::Vec<Thing>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Thing {
    #[prost(string, tag="1")]
    pub thing1: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub thing2: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub thing3: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionIncreases {
    #[prost(message, repeated, tag="1")]
    pub position_increases: ::prost::alloc::vec::Vec<PositionIncrease>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionIncrease {
    #[prost(string, tag="1")]
    pub trx: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub market: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub size_usd: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub size_tokens: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub collateral_amount: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub collateral_usd: ::prost::alloc::string::String,
    #[prost(bool, tag="7")]
    pub is_long: bool,
    #[prost(string, tag="8")]
    pub leverage: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
