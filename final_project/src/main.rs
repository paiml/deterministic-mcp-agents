#![warn(clippy::all, clippy::pedantic)]

use clap::Parser;
use pmcp::server::{Server, ServerBuilder};
use pmcp::tools::{
    analyze_complexity_tool, calculator_tool, deep_analysis_tool, extract_files_tool,
};
use pmcp::transport::{StdioTransport, Transport};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value = "info")]
    log_level: String,

    #[clap(short, long)]
    stdio: bool,

    #[clap(short, long, default_value = "8080")]
    port: u16,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let level = match args.log_level.as_str() {
        "debug" => Level::DEBUG,
        "info" => Level::INFO,
        "warn" => Level::WARN,
        "error" => Level::ERROR,
        _ => Level::INFO,
    };

    let subscriber = FmtSubscriber::builder().with_max_level(level).finish();

    tracing::subscriber::set_global_default(subscriber)?;

    info!("Starting Production MCP Server");
    info!("Version: {}", env!("CARGO_PKG_VERSION"));

    let server = ServerBuilder::new()
        .with_tool(calculator_tool())
        .with_tool(analyze_complexity_tool())
        .with_tool(extract_files_tool())
        .with_tool(deep_analysis_tool())
        .with_max_request_size(10_485_760)
        .build();

    info!(
        "Server configured with {} tools",
        server.capabilities().tools.len()
    );

    if args.stdio {
        info!("Running in stdio mode");
        run_stdio_server(server).await?;
    } else {
        info!("Running on port {}", args.port);
        run_tcp_server(server, args.port).await?;
    }

    Ok(())
}

async fn run_stdio_server(server: Server) -> anyhow::Result<()> {
    let mut transport = StdioTransport::new();

    info!("Stdio server ready");

    loop {
        match transport.receive().await {
            Ok(request) => {
                let response = server
                    .handle_request(request)
                    .await
                    .map_err(|e| anyhow::anyhow!("Request handling error: {}", e))?;
                transport
                    .send(response)
                    .await
                    .map_err(|e| anyhow::anyhow!("Transport send error: {}", e))?;
            }
            Err(e) => {
                tracing::error!("Transport error: {}", e);
                break;
            }
        }
    }

    Ok(())
}

async fn run_tcp_server(_server: Server, port: u16) -> anyhow::Result<()> {
    use tokio::net::TcpListener;

    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).await?;

    info!("TCP server listening on {}", addr);

    loop {
        let (_socket, addr) = listener.accept().await?;
        info!("New connection from {}", addr);

        tokio::spawn(async move {
            info!("Handling connection from {}", addr);
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_parsing() {
        let args = Args::parse_from(&["program", "--stdio"]);
        assert!(args.stdio);
        assert_eq!(args.port, 8080);
    }
}
