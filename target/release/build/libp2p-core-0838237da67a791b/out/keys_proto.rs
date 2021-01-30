#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublicKey {
    #[prost(enumeration="KeyType", required, tag="1")]
    pub r#type: i32,
    #[prost(bytes, required, tag="2")]
    pub data: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivateKey {
    #[prost(enumeration="KeyType", required, tag="1")]
    pub r#type: i32,
    #[prost(bytes, required, tag="2")]
    pub data: std::vec::Vec<u8>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum KeyType {
    Rsa = 0,
    Ed25519 = 1,
    Secp256k1 = 2,
}
