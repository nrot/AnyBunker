// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Index {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndexRow {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub date: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub data: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndexListRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndexListResponse {
    #[prost(message, repeated, tag="1")]
    pub indexes: ::prost::alloc::vec::Vec<Index>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndexSearchRequest {
    #[prost(message, optional, tag="1")]
    pub index: ::core::option::Option<Index>,
    #[prost(string, tag="2")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, optional, tag="3")]
    pub to: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag="4")]
    pub take: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="5")]
    pub skip: ::core::option::Option<u64>,
    #[prost(string, optional, tag="6")]
    pub query: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndexSearchResult {
    #[prost(message, repeated, tag="1")]
    pub response: ::prost::alloc::vec::Vec<IndexRow>,
}
include!("index.serde.rs");
// @@protoc_insertion_point(module)
