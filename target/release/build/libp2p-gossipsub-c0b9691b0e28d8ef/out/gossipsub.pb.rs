#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Rpc {
    #[prost(message, repeated, tag="1")]
    pub subscriptions: ::std::vec::Vec<rpc::SubOpts>,
    #[prost(message, repeated, tag="2")]
    pub publish: ::std::vec::Vec<Message>,
    #[prost(message, optional, tag="3")]
    pub control: ::std::option::Option<ControlMessage>,
}
pub mod rpc {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SubOpts {
        /// subscribe or unsubscribe
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
    #[prost(bytes, optional, tag="5")]
    pub signature: ::std::option::Option<std::vec::Vec<u8>>,
    #[prost(bytes, optional, tag="6")]
    pub key: ::std::option::Option<std::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ControlMessage {
    #[prost(message, repeated, tag="1")]
    pub ihave: ::std::vec::Vec<ControlIHave>,
    #[prost(message, repeated, tag="2")]
    pub iwant: ::std::vec::Vec<ControlIWant>,
    #[prost(message, repeated, tag="3")]
    pub graft: ::std::vec::Vec<ControlGraft>,
    #[prost(message, repeated, tag="4")]
    pub prune: ::std::vec::Vec<ControlPrune>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ControlIHave {
    #[prost(string, optional, tag="1")]
    pub topic_id: ::std::option::Option<std::string::String>,
    #[prost(bytes, repeated, tag="2")]
    pub message_ids: ::std::vec::Vec<std::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ControlIWant {
    #[prost(bytes, repeated, tag="1")]
    pub message_ids: ::std::vec::Vec<std::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ControlGraft {
    #[prost(string, optional, tag="1")]
    pub topic_id: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ControlPrune {
    #[prost(string, optional, tag="1")]
    pub topic_id: ::std::option::Option<std::string::String>,
}
/// topicID = hash(topicDescriptor); (not the topic.name)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopicDescriptor {
    #[prost(string, optional, tag="1")]
    pub name: ::std::option::Option<std::string::String>,
    #[prost(message, optional, tag="2")]
    pub auth: ::std::option::Option<topic_descriptor::AuthOpts>,
    #[prost(message, optional, tag="3")]
    pub enc: ::std::option::Option<topic_descriptor::EncOpts>,
}
pub mod topic_descriptor {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AuthOpts {
        #[prost(enumeration="auth_opts::AuthMode", optional, tag="1")]
        pub mode: ::std::option::Option<i32>,
        /// root keys to trust
        #[prost(bytes, repeated, tag="2")]
        pub keys: ::std::vec::Vec<std::vec::Vec<u8>>,
    }
    pub mod auth_opts {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum AuthMode {
            /// no authentication, anyone can publish
            None = 0,
            /// only messages signed by keys in the topic descriptor are accepted
            Key = 1,
            /// web of trust, certificates can allow publisher set to grow
            Wot = 2,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EncOpts {
        #[prost(enumeration="enc_opts::EncMode", optional, tag="1")]
        pub mode: ::std::option::Option<i32>,
        /// the hashes of the shared keys used (salted)
        #[prost(bytes, repeated, tag="2")]
        pub key_hashes: ::std::vec::Vec<std::vec::Vec<u8>>,
    }
    pub mod enc_opts {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum EncMode {
            /// no encryption, anyone can read
            None = 0,
            /// messages are encrypted with shared key
            Sharedkey = 1,
            /// web of trust, certificates can allow publisher set to grow
            Wot = 2,
        }
    }
}
