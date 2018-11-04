use std::default::Default;
use std::time::Duration;


pub struct Config {
    /// How frequently the server should ping a client to check for liveness.
    ///
    /// Default: 30,000 milliseconds
    pub ping_interval: Duration,

    /// The number of times to try pinging before disconnecting the client.
    ///
    /// Default: 1
    pub ping_max_tries: u8,

    /// The number of milliseconds to wait for a response. If no response is received
    /// after `ping_max_wait_ms` we will try again according to `ping_max_tries` policy.
    /// This number should not exceed `ping_interval`.
    ///
    /// Default: 30,000 milliseconds
    pub ping_max_wait: Duration,

    /// An ordered list of encodings supported by the server. The Loqui server does
    /// not handle the actual encoding/decoding of payloads. This is for the purpose
    /// of negotiation only.
    ///
    /// Default: vec![]
    pub supported_encodings: Vec<String>,

    /// An ordered list of compressions supported by the server. The Loqui server does
    /// not handle the actual compressing/decompressing of payloads. This is for the purpose
    /// of negotiation only.
    /// Default: vec![]
    pub supported_compressions: Vec<String>,

    /// The maximum size allowed from a payload.
    ///
    /// Default: ::std::u32::MAX
    pub max_payload_bytes: u32,

    /// The string representing a `host[:port]`. If the port is omit an ephemeral
    /// port will be chosen.
    ///
    /// Default: 127.0.0.1
    pub address: String,
}

impl Default for Config {
    fn default() -> Config {
        Config{
            ping_interval: Duration::from_millis(30_000),
            ping_max_tries: 1,
            ping_max_wait: Duration::from_millis(1_000),
            supported_encodings: Vec::new(),
            supported_compressions: Vec::new(),
            max_payload_bytes: ::std::u32::MAX,
            address: String::from("127.0.0.1:4001"),
        }
    }
}
