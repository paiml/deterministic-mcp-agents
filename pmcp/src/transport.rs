use crate::{Request, Response, Result};
use async_trait::async_trait;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::mpsc;

#[async_trait]
pub trait Transport: Send + Sync {
    async fn send(&mut self, response: Response) -> Result<()>;
    async fn receive(&mut self) -> Result<Request>;
}

pub struct StdioTransport {
    stdin: BufReader<tokio::io::Stdin>,
    stdout: tokio::io::Stdout,
}

impl StdioTransport {
    #[must_use]
    pub fn new() -> Self {
        Self {
            stdin: BufReader::new(tokio::io::stdin()),
            stdout: tokio::io::stdout(),
        }
    }
}

impl Default for StdioTransport {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Transport for StdioTransport {
    async fn send(&mut self, response: Response) -> Result<()> {
        let json = serde_json::to_string(&response)
            .map_err(|e| crate::PmcpError::Protocol(e.to_string()))?;

        self.stdout
            .write_all(json.as_bytes())
            .await
            .map_err(|e| crate::PmcpError::Transport(e.to_string()))?;

        self.stdout
            .write_all(b"\n")
            .await
            .map_err(|e| crate::PmcpError::Transport(e.to_string()))?;

        self.stdout
            .flush()
            .await
            .map_err(|e| crate::PmcpError::Transport(e.to_string()))?;

        Ok(())
    }

    async fn receive(&mut self) -> Result<Request> {
        let mut line = String::new();
        self.stdin
            .read_line(&mut line)
            .await
            .map_err(|e| crate::PmcpError::Transport(e.to_string()))?;

        serde_json::from_str(&line).map_err(|e| crate::PmcpError::Protocol(e.to_string()))
    }
}

pub struct WebSocketTransport {
    tx: mpsc::Sender<Response>,
    rx: mpsc::Receiver<Request>,
}

impl WebSocketTransport {
    #[must_use]
    pub fn new(tx: mpsc::Sender<Response>, rx: mpsc::Receiver<Request>) -> Self {
        Self { tx, rx }
    }
}

#[async_trait]
impl Transport for WebSocketTransport {
    async fn send(&mut self, response: Response) -> Result<()> {
        self.tx
            .send(response)
            .await
            .map_err(|e| crate::PmcpError::Transport(e.to_string()))
    }

    async fn receive(&mut self) -> Result<Request> {
        self.rx
            .recv()
            .await
            .ok_or_else(|| crate::PmcpError::Transport("Channel closed".to_string()))
    }
}
