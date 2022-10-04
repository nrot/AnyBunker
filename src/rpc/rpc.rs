// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Message {
    #[prost(string, tag="1")]
    pub index: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub password: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub data: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResultMessage {
    #[prost(bool, tag="1")]
    pub ok: bool,
}
include!("rpc.tonic.rs");
include!("rpc.serde.rs");
// @@protoc_insertion_point(module)
