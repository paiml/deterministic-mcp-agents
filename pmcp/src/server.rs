use crate::{Request, Response, Result, ServerCapabilities, Tool};
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::RwLock;

#[async_trait]
pub trait ToolHandler: Send + Sync {
    async fn handle(&self, params: Option<serde_json::Value>) -> Result<serde_json::Value>;
}

#[derive(Clone)]
pub struct Server {
    capabilities: ServerCapabilities,
    handlers: Arc<RwLock<std::collections::HashMap<String, Box<dyn ToolHandler>>>>,
}

impl Server {
    pub fn new(capabilities: ServerCapabilities) -> Self {
        Self {
            capabilities,
            handlers: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }
    
    pub async fn register_tool(&self, tool: Tool, handler: Box<dyn ToolHandler>) {
        let mut handlers = self.handlers.write().await;
        handlers.insert(tool.name.clone(), handler);
    }
    
    pub async fn handle_request(&self, request: Request) -> Result<Response> {
        let handlers = self.handlers.read().await;
        
        if let Some(handler) = handlers.get(&request.method) {
            match handler.handle(request.params).await {
                Ok(result) => Ok(Response {
                    jsonrpc: "2.0".to_string(),
                    result: Some(result),
                    error: None,
                    id: request.id,
                }),
                Err(e) => Ok(Response {
                    jsonrpc: "2.0".to_string(),
                    result: None,
                    error: Some(crate::ErrorObject {
                        code: -32603,
                        message: e.to_string(),
                        data: None,
                    }),
                    id: request.id,
                }),
            }
        } else {
            Ok(Response {
                jsonrpc: "2.0".to_string(),
                result: None,
                error: Some(crate::ErrorObject {
                    code: -32601,
                    message: "Method not found".to_string(),
                    data: None,
                }),
                id: request.id,
            })
        }
    }
    
    pub fn capabilities(&self) -> &ServerCapabilities {
        &self.capabilities
    }
}

pub struct ServerBuilder {
    capabilities: ServerCapabilities,
}

impl ServerBuilder {
    pub fn new() -> Self {
        Self {
            capabilities: ServerCapabilities::default(),
        }
    }
    
    pub fn with_tool(mut self, tool: Tool) -> Self {
        self.capabilities.tools.push(tool);
        self
    }
    
    pub fn with_max_request_size(mut self, size: usize) -> Self {
        self.capabilities.max_request_size = size;
        self
    }
    
    pub fn build(self) -> Server {
        Server::new(self.capabilities)
    }
}

impl Default for ServerBuilder {
    fn default() -> Self {
        Self::new()
    }
}