use self::target::HttpInfoProtocol;
use abi_stable::StableAbi;

pub mod args;
pub mod command;
pub mod response;
pub mod target;

pub trait Push<T> {
    fn push(&mut self, t: T);
}

// openC2的跟踪标识
pub trait TraceIdent {
    fn trace_id(&self) -> &str;
}

#[repr(u8)]
#[derive(Debug, Clone, Serialize, Eq, StableAbi, Deserialize, PartialEq)]
pub enum OpenC2MsgType {
    #[serde(rename = "request")]
    Request,

    #[serde(rename = "response")]
    Response,
}

impl Default for OpenC2MsgType {
    fn default() -> Self {
        Self::Response
    }
}

impl Default for HttpInfoProtocol {
    fn default() -> Self {
        Self::Http
    }
}
