/// Request a finality proof from a peer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinalityProofRequest {
    /// SCALE-encoded hash of the block to request.
    #[prost(bytes, tag="1")]
    pub block_hash: std::vec::Vec<u8>,
    /// Opaque chain-specific additional request data.
    #[prost(bytes, tag="2")]
    pub request: std::vec::Vec<u8>,
}
/// Response to a finality proof request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinalityProofResponse {
    /// Opaque chain-specific finality proof. Empty if no such proof exists.
    ///
    /// optional
    #[prost(bytes, tag="1")]
    pub proof: std::vec::Vec<u8>,
}
