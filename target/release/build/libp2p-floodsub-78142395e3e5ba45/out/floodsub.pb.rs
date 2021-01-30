#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Rpc {
    #[prost(message, repeated, tag="1")]
    pub subscriptions: ::std::vec::Vec<rpc::SubOpts>,
    #[prost(message, repeated, tag="2")]
    pub publish: ::std::vec::Vec<Message>,
}
pub mod rpc {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SubOpts {
        /// subscribe or unsubcribe
        #[prost(bool, optional, tag="1")]
        pub subscribe: ::std::option::Option<bool>,
        #[prost(string, optional, tag="2")]
        pub topic_id: ::std::option::Option<std::string::String>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Message {
    #[prost(bytes, optional, tag="1")]
    pub from: ::std::option::Option<std::vec::Vec<u8>>,
    #[prost(bytes, optional, tag="2")]
    pub data: ::std::option::Option<std::vec::Vec<u8>>,
    #[prost(bytes, optional, tag="3")]
    pub seqno: ::std::option::Option<std::vec::Vec<u8>>,
    #[prost(string, repeated, tag="4")]
    pub topic_ids: ::std::vec::Vec<std::string::String>,
}
