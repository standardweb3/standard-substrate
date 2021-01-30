/// A pair of arbitrary bytes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pair {
    /// The first element of the pair.
    #[prost(bytes, tag="1")]
    pub fst: std::vec::Vec<u8>,
    /// The second element of the pair.
    #[prost(bytes, tag="2")]
    pub snd: std::vec::Vec<u8>,
}
/// Enumerate all possible light client request messages.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Request {
    #[prost(oneof="request::Request", tags="1, 2, 3, 4, 5")]
    pub request: ::std::option::Option<request::Request>,
}
pub mod request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Request {
        #[prost(message, tag="1")]
        RemoteCallRequest(super::RemoteCallRequest),
        #[prost(message, tag="2")]
        RemoteReadRequest(super::RemoteReadRequest),
        #[prost(message, tag="3")]
        RemoteHeaderRequest(super::RemoteHeaderRequest),
        #[prost(message, tag="4")]
        RemoteReadChildRequest(super::RemoteReadChildRequest),
        #[prost(message, tag="5")]
        RemoteChangesRequest(super::RemoteChangesRequest),
    }
}
/// Enumerate all possible light client response messages.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    #[prost(oneof="response::Response", tags="1, 2, 3, 4")]
    pub response: ::std::option::Option<response::Response>,
}
pub mod response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag="1")]
        RemoteCallResponse(super::RemoteCallResponse),
        #[prost(message, tag="2")]
        RemoteReadResponse(super::RemoteReadResponse),
        #[prost(message, tag="3")]
        RemoteHeaderResponse(super::RemoteHeaderResponse),
        #[prost(message, tag="4")]
        RemoteChangesResponse(super::RemoteChangesResponse),
    }
}
/// Remote call request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoteCallRequest {
    /// Block at which to perform call.
    #[prost(bytes, tag="2")]
    pub block: std::vec::Vec<u8>,
    /// Method name.
    #[prost(string, tag="3")]
    pub method: std::string::String,
    /// Call data.
    #[prost(bytes, tag="4")]
    pub data: std::vec::Vec<u8>,
}
/// Remote call response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoteCallResponse {
    /// Execution proof.
    #[prost(bytes, tag="2")]
    pub proof: std::vec::Vec<u8>,
}
/// Remote storage read request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoteReadRequest {
    /// Block at which to perform call.
    #[prost(bytes, tag="2")]
    pub block: std::vec::Vec<u8>,
    /// Storage keys.
    #[prost(bytes, repeated, tag="3")]
    pub keys: ::std::vec::Vec<std::vec::Vec<u8>>,
}
/// Remote read response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoteReadResponse {
    /// Read proof.
    #[prost(bytes, tag="2")]
    pub proof: std::vec::Vec<u8>,
}
/// Remote storage read child request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoteReadChildRequest {
    /// Block at which to perform call.
    #[prost(bytes, tag="2")]
    pub block: std::vec::Vec<u8>,
    /// Child Storage key, this is relative
    /// to the child type storage location.
    #[prost(bytes, tag="3")]
    pub storage_key: std::vec::Vec<u8>,
    /// Storage keys.
    #[prost(bytes, repeated, tag="6")]
    pub keys: ::std::vec::Vec<std::vec::Vec<u8>>,
}
/// Remote header request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoteHeaderRequest {
    /// Block number to request header for.
    #[prost(bytes, tag="2")]
    pub block: std::vec::Vec<u8>,
}
/// Remote header response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoteHeaderResponse {
    /// Header. None if proof generation has failed (e.g. header is unknown).
    ///
    /// optional
    #[prost(bytes, tag="2")]
    pub header: std::vec::Vec<u8>,
    /// Header proof.
    #[prost(bytes, tag="3")]
    pub proof: std::vec::Vec<u8>,
}
//// Remote changes request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoteChangesRequest {
    /// Hash of the first block of the range (including first) where changes are requested.
    #[prost(bytes, tag="2")]
    pub first: std::vec::Vec<u8>,
    /// Hash of the last block of the range (including last) where changes are requested.
    #[prost(bytes, tag="3")]
    pub last: std::vec::Vec<u8>,
    /// Hash of the first block for which the requester has the changes trie root. All other
    /// affected roots must be proved.
    #[prost(bytes, tag="4")]
    pub min: std::vec::Vec<u8>,
    /// Hash of the last block that we can use when querying changes.
    #[prost(bytes, tag="5")]
    pub max: std::vec::Vec<u8>,
    /// Storage child node key which changes are requested.
    ///
    /// optional
    #[prost(bytes, tag="6")]
    pub storage_key: std::vec::Vec<u8>,
    /// Storage key which changes are requested.
    #[prost(bytes, tag="7")]
    pub key: std::vec::Vec<u8>,
}
/// Remote changes response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoteChangesResponse {
    /// Proof has been generated using block with this number as a max block. Should be
    /// less than or equal to the RemoteChangesRequest::max block number.
    #[prost(bytes, tag="2")]
    pub max: std::vec::Vec<u8>,
    /// Changes proof.
    #[prost(bytes, repeated, tag="3")]
    pub proof: ::std::vec::Vec<std::vec::Vec<u8>>,
    /// Changes tries roots missing on the requester' node.
    #[prost(message, repeated, tag="4")]
    pub roots: ::std::vec::Vec<Pair>,
    /// Missing changes tries roots proof.
    #[prost(bytes, tag="5")]
    pub roots_proof: std::vec::Vec<u8>,
}
