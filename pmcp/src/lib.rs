#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

pub mod server;
pub mod transport;
pub mod protocol;
pub mod tools;

#[derive(Error, Debug)]
pub enum PmcpError {
    #[error("Transport error: {0}")]
    Transport(String),
    
    #[error("Protocol error: {0}")]
    Protocol(String),
    
    #[error("Tool error: {0}")]
    Tool(String),
    
    #[error("Server error: {0}")]
    Server(String),
    
    #[error("JSON-RPC error: {code}: {message}")]
    JsonRpc { code: i32, message: String },
}

pub type Result<T> = std::result::Result<T, PmcpError>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    pub jsonrpc: String,
    pub method: String,
    pub params: Option<serde_json::Value>,
    pub id: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub jsonrpc: String,
    pub result: Option<serde_json::Value>,
    pub error: Option<ErrorObject>,
    pub id: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorObject {
    pub code: i32,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub jsonrpc: String,
    pub method: String,
    pub params: Option<serde_json::Value>,
}

#[derive(Debug, Clone)]
pub struct Tool {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
}

#[derive(Debug, Clone)]
pub struct ServerCapabilities {
    pub tools: Vec<Tool>,
    pub max_request_size: usize,
    pub supports_batching: bool,
    pub supports_cancellation: bool,
}

impl Default for ServerCapabilities {
    fn default() -> Self {
        Self {
            tools: Vec::new(),
            max_request_size: 10_485_760, // 10MB
            supports_batching: true,
            supports_cancellation: true,
        }
    }
}