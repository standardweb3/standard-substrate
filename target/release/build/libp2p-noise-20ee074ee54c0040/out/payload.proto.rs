// Payloads for Noise handshake messages.

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NoiseHandshakePayload {
    #[prost(bytes, tag="1")]
    pub identity_key: std::vec::Vec<u8>,
    #[prost(bytes, tag="2")]
    pub identity_sig: std::vec::Vec<u8>,
    #[prost(bytes, tag="3")]
    pub data: std::vec::Vec<u8>,
}
