#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenericMessage {
    #[prost(message, optional, tag = "1")]
    pub payload: ::core::option::Option<::prost_types::Any>,
}
