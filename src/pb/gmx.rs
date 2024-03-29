// @generated
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
    pub event_name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub trx: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub account: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub market: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub market_address: ::prost::alloc::string::String,
    #[prost(double, tag="6")]
    pub execution_price: f64,
    #[prost(double, tag="7")]
    pub size_usd: f64,
    #[prost(double, tag="8")]
    pub size_tokens: f64,
    #[prost(double, tag="9")]
    pub collateral_amount: f64,
    #[prost(bool, tag="10")]
    pub is_long: bool,
    #[prost(double, tag="11")]
    pub leverage: f64,
    #[prost(int32, tag="12")]
    pub order_type: i32,
    #[prost(string, tag="13")]
    pub order_key: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub position_key: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub timestamp: ::prost::alloc::string::String,
    #[prost(uint64, tag="16")]
    pub block_number: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionDecreases {
    #[prost(message, repeated, tag="1")]
    pub position_decreases: ::prost::alloc::vec::Vec<PositionDecrease>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionDecrease {
    #[prost(string, tag="1")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub trx: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub account: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub market: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub market_address: ::prost::alloc::string::String,
    #[prost(double, tag="6")]
    pub execution_price: f64,
    #[prost(double, tag="7")]
    pub size_usd: f64,
    #[prost(double, tag="8")]
    pub size_tokens: f64,
    #[prost(double, tag="9")]
    pub collateral_amount: f64,
    #[prost(bool, tag="10")]
    pub is_long: bool,
    #[prost(double, tag="11")]
    pub base_pnl: f64,
    #[prost(double, tag="12")]
    pub leverage: f64,
    #[prost(int32, tag="13")]
    pub order_type: i32,
    #[prost(string, tag="14")]
    pub order_key: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub position_key: ::prost::alloc::string::String,
    #[prost(string, tag="16")]
    pub timestamp: ::prost::alloc::string::String,
    #[prost(uint64, tag="17")]
    pub block_number: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionChange {
    #[prost(string, tag="1")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub trx: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub account: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub market: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub market_address: ::prost::alloc::string::String,
    #[prost(double, tag="6")]
    pub execution_price: f64,
    #[prost(double, tag="7")]
    pub size_usd: f64,
    #[prost(double, tag="8")]
    pub size_tokens: f64,
    #[prost(double, tag="9")]
    pub collateral_amount: f64,
    #[prost(bool, tag="10")]
    pub is_long: bool,
    #[prost(double, tag="11")]
    pub base_pnl: f64,
    #[prost(double, tag="12")]
    pub leverage: f64,
    #[prost(int32, tag="13")]
    pub order_type: i32,
    #[prost(string, tag="14")]
    pub order_key: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub position_key: ::prost::alloc::string::String,
    #[prost(string, tag="16")]
    pub timestamp: ::prost::alloc::string::String,
    #[prost(uint64, tag="17")]
    pub block_number: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionChanges {
    #[prost(message, repeated, tag="1")]
    pub position_changes: ::prost::alloc::vec::Vec<PositionChange>,
}
// @@protoc_insertion_point(module)
