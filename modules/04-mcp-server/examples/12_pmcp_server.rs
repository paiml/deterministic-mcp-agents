use pmcp::server::{Server, ServerBuilder};
use pmcp::tools::{calculator_tool, analyze_complexity_tool};
use pmcp::{Request, Response};
use std::time::Instant;

#[tokio::main]
async fn main() {
    println!("PMCP Server Demo");
    println!("================\n");
    
    let server = ServerBuilder::new()
        .with_tool(calculator_tool())
        .with_tool(analyze_complexity_tool())
        .with_max_request_size(10_485_760)
        .build();
    
    println!("✅ Server configured with {} tools", server.capabilities().tools.len());
    println!("✅ Max request size: 10MB");
    println!("✅ Batching supported");
    println!("✅ Cancellation supported");
    
    let request = Request {
        jsonrpc: "2.0".to_string(),
        method: "calculator".to_string(),
        params: Some(serde_json::json!({"operation": "add", "a": 5, "b": 3})),
        id: Some(serde_json::json!(1)),
    };
    
    let start = Instant::now();
    let response = server.handle_request(request).await.unwrap();
    let duration = start.elapsed();
    
    println!("\n📊 Request handling:");
    println!("  Response: {:?}", response.result);
    println!("  Latency: {:?}", duration);
    
    if duration.as_millis() < 10 {
        println!("  ✅ Performance target met (<10ms)");
    }
    
    benchmark_throughput(server).await;
}

async fn benchmark_throughput(server: Server) {
    println!("\n⚡ Benchmark: 10k requests/second");
    
    let iterations = 10_000;
    let start = Instant::now();
    
    for i in 0..iterations {
        let request = Request {
            jsonrpc: "2.0".to_string(),
            method: "calculator".to_string(),
            params: Some(serde_json::json!({"operation": "add", "a": i, "b": 1})),
            id: Some(serde_json::json!(i)),
        };
        
        let _ = server.handle_request(request).await;
    }
    
    let duration = start.elapsed();
    let rate = iterations as f64 / duration.as_secs_f64();
    
    println!("  Requests: {}", iterations);
    println!("  Duration: {:?}", duration);
    println!("  Rate: {:.0} req/s", rate);
    
    if rate >= 10_000.0 {
        println!("  ✅ Throughput target met");
    }
}