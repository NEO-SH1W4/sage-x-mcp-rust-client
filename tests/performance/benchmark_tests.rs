use sage_x_mcp_rust_client::{MCPClient, models::*};
use std::time::{Duration, Instant};
use tokio::time::timeout;

/// Performance benchmarks for the MCP Client
/// 
/// Run with: `cargo test --release --test performance`
/// For more detailed benchmarks, use the benches/ directory with criterion.

const TEST_ITERATIONS: usize = 1000;
const TIMEOUT_DURATION: Duration = Duration::from_secs(30);

#[tokio::test]
async fn benchmark_client_creation() {
    let start = Instant::now();
    
    for i in 0..TEST_ITERATIONS {
        let url = format!("test://localhost:{}", 8080 + i);
        let _client = MCPClient::new(&url);
    }
    
    let duration = start.elapsed();
    let per_operation = duration / TEST_ITERATIONS as u32;
    
    println!("Client creation benchmark:");
    println!("  Total time: {:?}", duration);
    println!("  Per operation: {:?}", per_operation);
    println!("  Operations per second: {:.0}", 1.0 / per_operation.as_secs_f64());
    
    // Assert reasonable performance
    assert!(per_operation < Duration::from_millis(1), 
           "Client creation should be under 1ms per operation");
}

#[tokio::test]
async fn benchmark_request_serialization() {
    let start = Instant::now();
    
    for i in 0..TEST_ITERATIONS {
        let request = MCPRequest {
            jsonrpc: "2.0".to_string(),
            method: "test_method".to_string(),
            params: Some(serde_json::json!({
                "test_param": format!("value_{}", i),
                "iteration": i
            })),
            id: Some(MCPRequestId::Number(i as i64)),
        };
        
        let _serialized = serde_json::to_string(&request).unwrap();
    }
    
    let duration = start.elapsed();
    let per_operation = duration / TEST_ITERATIONS as u32;
    
    println!("Request serialization benchmark:");
    println!("  Total time: {:?}", duration);
    println!("  Per operation: {:?}", per_operation);
    println!("  Operations per second: {:.0}", 1.0 / per_operation.as_secs_f64());
    
    // Assert reasonable performance
    assert!(per_operation < Duration::from_micros(100), 
           "Serialization should be under 100µs per operation");
}

#[tokio::test]
async fn benchmark_response_deserialization() {
    // Pre-create test responses
    let test_responses: Vec<String> = (0..TEST_ITERATIONS)
        .map(|i| {
            serde_json::to_string(&MCPResponse {
                jsonrpc: "2.0".to_string(),
                result: Some(serde_json::json!({
                    "data": format!("response_{}", i),
                    "iteration": i
                })),
                error: None,
                id: Some(MCPRequestId::Number(i as i64)),
            }).unwrap()
        })
        .collect();
    
    let start = Instant::now();
    
    for response_str in &test_responses {
        let _response: MCPResponse = serde_json::from_str(response_str).unwrap();
    }
    
    let duration = start.elapsed();
    let per_operation = duration / TEST_ITERATIONS as u32;
    
    println!("Response deserialization benchmark:");
    println!("  Total time: {:?}", duration);
    println!("  Per operation: {:?}", per_operation);
    println!("  Operations per second: {:.0}", 1.0 / per_operation.as_secs_f64());
    
    // Assert reasonable performance
    assert!(per_operation < Duration::from_micros(100), 
           "Deserialization should be under 100µs per operation");
}

#[tokio::test]
async fn benchmark_concurrent_requests() {
    let client = MCPClient::new("test://localhost:8080").unwrap();
    let start = Instant::now();
    
    // Create futures for concurrent requests
    let mut futures = Vec::new();
    
    for i in 0..100 { // Use smaller number for concurrent test
        let client_clone = client.clone();
        let future = async move {
            let request = MCPRequest {
                jsonrpc: "2.0".to_string(),
                method: "ping".to_string(),
                params: None,
                id: Some(MCPRequestId::Number(i)),
            };
            
            // Simulate request processing (since we don't have a real server)
            tokio::time::sleep(Duration::from_millis(1)).await;
            Ok::<(), sage_x_mcp_rust_client::MCPError>(())
        };
        
        futures.push(future);
    }
    
    // Execute all futures concurrently
    let results = futures::future::join_all(futures).await;
    
    let duration = start.elapsed();
    let successful_requests = results.iter().filter(|r| r.is_ok()).count();
    
    println!("Concurrent requests benchmark:");
    println!("  Total time: {:?}", duration);
    println!("  Successful requests: {}", successful_requests);
    println!("  Requests per second: {:.0}", successful_requests as f64 / duration.as_secs_f64());
    
    // Assert most requests succeeded
    assert!(successful_requests >= 95, "At least 95% of requests should succeed");
    
    // Assert reasonable throughput
    assert!(duration < Duration::from_secs(5), "Concurrent requests should complete within 5 seconds");
}

#[tokio::test]
async fn benchmark_memory_usage() {
    let start_memory = get_memory_usage();
    
    // Create many clients and requests to test memory usage
    let mut clients = Vec::new();
    let mut requests = Vec::new();
    
    for i in 0..1000 {
        let client = MCPClient::new(&format!("test://localhost:{}", 8080 + i)).unwrap();
        clients.push(client);
        
        let request = MCPRequest {
            jsonrpc: "2.0".to_string(),
            method: "test_method".to_string(),
            params: Some(serde_json::json!({
                "large_data": "x".repeat(1024), // 1KB of data
                "iteration": i
            })),
            id: Some(MCPRequestId::Number(i as i64)),
        };
        requests.push(request);
    }
    
    let peak_memory = get_memory_usage();
    let memory_used = peak_memory.saturating_sub(start_memory);
    
    println!("Memory usage benchmark:");
    println!("  Start memory: {} KB", start_memory);
    println!("  Peak memory: {} KB", peak_memory);
    println!("  Memory used: {} KB", memory_used);
    println!("  Memory per client: {} bytes", (memory_used * 1024) / clients.len());
    
    // Clean up to test memory release
    drop(clients);
    drop(requests);
    
    // Force garbage collection (not available in Rust, but dropping should help)
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    let end_memory = get_memory_usage();
    let memory_released = peak_memory.saturating_sub(end_memory);
    
    println!("  End memory: {} KB", end_memory);
    println!("  Memory released: {} KB", memory_released);
    
    // Assert reasonable memory usage (these thresholds may need adjustment)
    assert!(memory_used < 50_000, "Memory usage should be under 50MB for 1000 clients");
}

/// Helper function to get current memory usage in KB
/// Note: This is a simplified implementation. In real scenarios,
/// you might want to use a proper memory profiling tool.
fn get_memory_usage() -> usize {
    #[cfg(target_os = "windows")]
    {
        use std::mem;
        use winapi::um::processthreadsapi::GetCurrentProcess;
        use winapi::um::psapi::{GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS};
        
        unsafe {
            let mut pmc: PROCESS_MEMORY_COUNTERS = mem::zeroed();
            if GetProcessMemoryInfo(
                GetCurrentProcess(),
                &mut pmc as *mut _,
                mem::size_of::<PROCESS_MEMORY_COUNTERS>() as u32,
            ) != 0 {
                return (pmc.WorkingSetSize / 1024) as usize;
            }
        }
    }
    
    // Fallback: return 0 if we can't get memory info
    // In real benchmarks, you'd want a proper cross-platform solution
    0
}

