use pmcp::{Request, Response, Notification, ErrorObject};
use pmcp::transport::{StdioTransport, Transport};
use pmcp::protocol::{JSONRPC_VERSION, ERROR_METHOD_NOT_FOUND};
use serde_json::json;
use std::time::Instant;
use std::collections::HashMap;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("MCP Protocol Implementation Demo");
    println!("================================\n");
    
    demonstrate_json_rpc_framing().await;
    demonstrate_request_response_correlation().await;
    demonstrate_notification_batching().await;
    demonstrate_error_handling();
    benchmark_latency().await;
}

async fn demonstrate_json_rpc_framing() {
    println!("📦 JSON-RPC 2.0 Message Framing:");
    
    let request = Request {
        jsonrpc: JSONRPC_VERSION.to_string(),
        method: "calculator.add".to_string(),
        params: Some(json!({ "a": 5, "b": 3 })),
        id: Some(json!(1)),
    };
    
    let json = serde_json::to_string_pretty(&request).unwrap();
    println!("  Request:\n{}", indent(&json, 4));
    
    let response = Response {
        jsonrpc: JSONRPC_VERSION.to_string(),
        result: Some(json!(8)),
        error: None,
        id: Some(json!(1)),
    };
    
    let json = serde_json::to_string_pretty(&response).unwrap();
    println!("\n  Response:\n{}", indent(&json, 4));
}

async fn demonstrate_request_response_correlation() {
    println!("\n🔗 Request/Response Correlation:");
    
    let mut pending_requests = HashMap::new();
    
    for i in 1..=3 {
        let request = Request {
            jsonrpc: JSONRPC_VERSION.to_string(),
            method: format!("method_{}", i),
            params: None,
            id: Some(json!(i)),
        };
        
        pending_requests.insert(i, request.method.clone());
        println!("  Sent request #{} for '{}'", i, pending_requests[&i]);
    }
    
    println!("\n  Processing responses (may arrive out of order):");
    let response_order = vec![2, 1, 3];
    
    for id in response_order {
        if let Some(method) = pending_requests.remove(&id) {
            println!("    Received response #{} for '{}'", id, method);
        }
    }
}

async fn demonstrate_notification_batching() {
    println!("\n📨 Notification Batching (10 messages/100ms):");
    
    let (tx, mut rx) = mpsc::channel(100);
    
    tokio::spawn(async move {
        let mut batch = Vec::new();
        let mut last_send = Instant::now();
        
        while let Some(notification) = rx.recv().await {
            batch.push(notification);
            
            if batch.len() >= 10 || last_send.elapsed() > Duration::from_millis(100) {
                println!("    Sending batch of {} notifications", batch.len());
                batch.clear();
                last_send = Instant::now();
            }
        }
    });
    
    for i in 1..=25 {
        let notification = Notification {
            jsonrpc: JSONRPC_VERSION.to_string(),
            method: "log".to_string(),
            params: Some(json!({ "message": format!("Event {}", i) })),
        };
        
        tx.send(notification).await.unwrap();
        
        if i % 5 == 0 {
            sleep(Duration::from_millis(30)).await;
        }
    }
    
    sleep(Duration::from_millis(200)).await;
}

fn demonstrate_error_handling() {
    println!("\n❌ Structured Error Handling:");
    
    let error_codes = vec![
        (-32700, "Parse error"),
        (-32600, "Invalid Request"),
        (-32601, "Method not found"),
        (-32602, "Invalid params"),
        (-32603, "Internal error"),
        (-32000, "Server error start"),
        (-32099, "Server error end"),
    ];
    
    for (code, message) in error_codes {
        let error = ErrorObject {
            code,
            message: message.to_string(),
            data: None,
        };
        
        println!("  Code {}: {}", error.code, error.message);
    }
}

async fn benchmark_latency() {
    println!("\n⏱️  Latency Benchmark:");
    
    let mut total_duration = Duration::ZERO;
    let iterations = 1000;
    
    for _ in 0..iterations {
        let start = Instant::now();
        
        let request = Request {
            jsonrpc: JSONRPC_VERSION.to_string(),
            method: "echo".to_string(),
            params: Some(json!("test")),
            id: Some(json!(1)),
        };
        
        let _ = serde_json::to_vec(&request).unwrap();
        let response = Response {
            jsonrpc: JSONRPC_VERSION.to_string(),
            result: Some(json!("test")),
            error: None,
            id: Some(json!(1)),
        };
        let _ = serde_json::to_vec(&response).unwrap();
        
        total_duration += start.elapsed();
    }
    
    let avg_latency = total_duration / iterations as u32;
    println!("  Average round-trip: {:?}", avg_latency);
    
    if avg_latency < Duration::from_millis(5) {
        println!("  ✅ Performance target met (<5ms)");
    } else {
        println!("  ⚠️  Latency: {:?}", avg_latency);
    }
}

fn indent(s: &str, spaces: usize) -> String {
    s.lines()
        .map(|line| format!("{}{}", " ".repeat(spaces), line))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_json_rpc_serialization() {
        let request = Request {
            jsonrpc: JSONRPC_VERSION.to_string(),
            method: "test".to_string(),
            params: None,
            id: Some(json!(1)),
        };
        
        let json = serde_json::to_string(&request).unwrap();
        let parsed: Request = serde_json::from_str(&json).unwrap();
        
        assert_eq!(parsed.method, "test");
        assert_eq!(parsed.jsonrpc, JSONRPC_VERSION);
    }
    
    #[tokio::test]
    async fn test_error_codes() {
        let error = ErrorObject {
            code: ERROR_METHOD_NOT_FOUND,
            message: "Method not found".to_string(),
            data: None,
        };
        
        assert_eq!(error.code, -32601);
    }
}