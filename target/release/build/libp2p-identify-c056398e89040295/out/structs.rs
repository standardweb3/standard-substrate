#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Identify {
    /// protocolVersion determines compatibility between peers
    ///
    /// e.g. ipfs/1.0.0
    #[prost(string, optional, tag="5")]
    pub protocol_version: ::std::option::Option<std::string::String>,
    /// agentVersion is like a UserAgent string in browsers, or client version in bittorrent
    /// includes the client name and client.
    ///
    /// e.g. go-ipfs/0.1.0
    #[prost(string, optional, tag="6")]
    pub agent_version: ::std::option::Option<std::string::String>,
    /// publicKey is this node's public key (which also gives its node.ID)
    /// - may not need to be sent, as secure channel implies it has been sent.
    /// - then again, if we change / disable secure channel, may still want it.
    #[prost(bytes, optional, tag="1")]
    pub public_key: ::std::option::Option<std::vec::Vec<u8>>,
    /// listenAddrs are the multiaddrs the sender node listens for open connections on
    #[prost(bytes, repeated, tag="2")]
    pub listen_addrs: ::std::vec::Vec<std::vec::Vec<u8>>,
    /// oservedAddr is the multiaddr of the remote endpoint that the sender node perceives
    /// this is useful information to convey to the other side, as it helps the remote endpoint
    /// determine whether its connection to the local peer goes through NAT.
    #[prost(bytes, optional, tag="4")]
    pub observed_addr: ::std::option::Option<std::vec::Vec<u8>>,
    #[prost(string, repeated, tag="3")]
    pub protocols: ::std::vec::Vec<std::string::String>,
}
