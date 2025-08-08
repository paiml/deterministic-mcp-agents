use serde::{Deserialize, Serialize};

pub const JSONRPC_VERSION: &str = "2.0";

pub const ERROR_PARSE: i32 = -32700;
pub const ERROR_INVALID_REQUEST: i32 = -32600;
pub const ERROR_METHOD_NOT_FOUND: i32 = -32601;
pub const ERROR_INVALID_PARAMS: i32 = -32602;
pub const ERROR_INTERNAL: i32 = -32603;

pub const ERROR_SERVER_MIN: i32 = -32099;
pub const ERROR_SERVER_MAX: i32 = -32000;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Message {
    Request(crate::Request),
    Response(crate::Response),
    Notification(crate::Notification),
}

#[must_use]
pub fn is_request(msg: &serde_json::Value) -> bool {
    msg.get("method").is_some() && msg.get("id").is_some()
}

#[must_use]
pub fn is_notification(msg: &serde_json::Value) -> bool {
    msg.get("method").is_some() && msg.get("id").is_none()
}

#[must_use]
pub fn is_response(msg: &serde_json::Value) -> bool {
    msg.get("result").is_some() || msg.get("error").is_some()
}
