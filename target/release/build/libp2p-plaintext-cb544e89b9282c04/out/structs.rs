#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Exchange {
    #[prost(bytes, optional, tag="1")]
    pub id: ::std::option::Option<std::vec::Vec<u8>>,
    #[prost(bytes, optional, tag="2")]
    pub pubkey: ::std::option::Option<std::vec::Vec<u8>>,
}
