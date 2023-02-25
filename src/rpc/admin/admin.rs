// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdminAuth {
    #[prost(string, tag="1")]
    pub token: ::prost::alloc::string::String,
}
include!("admin.tonic.rs");
include!("admin.serde.rs");
// @@protoc_insertion_point(module)
