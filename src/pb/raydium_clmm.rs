// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwapEvents {
    #[prost(message, repeated, tag="1")]
    pub events: ::prost::alloc::vec::Vec<SwapEvent>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwapEvent {
    #[prost(string, tag="1")]
    pub pool_state: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub token_account_0: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub token_account_1: ::prost::alloc::string::String,
    #[prost(uint64, tag="5")]
    pub amount_0: u64,
    #[prost(uint64, tag="6")]
    pub transfer_fee_0: u64,
    #[prost(uint64, tag="7")]
    pub amount_1: u64,
    #[prost(uint64, tag="8")]
    pub transfer_fee_1: u64,
    #[prost(bool, tag="9")]
    pub zero_for_one: bool,
    #[prost(uint64, tag="10")]
    pub sqrt_price_x64: u64,
    #[prost(uint64, tag="11")]
    pub liquidity: u64,
    #[prost(int32, tag="12")]
    pub tick: i32,
}
// @@protoc_insertion_point(module)
