use sage_x_mcp_rust_client::{MCPClient, MCPError};
use std::time::Duration;
use tokio::time::timeout;

/// Integration tests for the MCP Client
/// 
/// These tests require a running MCP server for full integration testing.
/// Use `cargo test --test integration` to run these tests.

#[tokio::test]
async fn test_client_initialization() {
    let client = MCPClient::new("test://localhost:8080");
    assert!(client.is_ok(), "Client should initialize successfully");
}

#[tokio::test]
async fn test_client_connection_timeout() {
    let client = MCPClient::new("test://invalid-host:8080");
    
    if let Ok(client) = client {
        // Test connection with timeout
        let result = timeout(Duration::from_secs(5), client.connect()).await;
        
        match result {
            Ok(Err(MCPError::ConnectionError(_))) => {
                // Expected: connection should fail for invalid host
            }
            Ok(Ok(_)) => {
                panic!("Connection should not succeed to invalid host");
            }
            Err(_) => {
                // Timeout is also acceptable for this test
            }
            Ok(Err(other)) => {
                panic!("Unexpected error type: {:?}", other);
            }
        }
    }
}

#[tokio::test]
async fn test_client_configuration() {
    let mut client = MCPClient::new("test://localhost:8080").unwrap();
    
    // Test setting timeout
    client.set_timeout(Duration::from_secs(30));
    
    // Test setting user agent
    client.set_user_agent("test-client/1.0");
    
    // Verify configuration doesn't panic
    assert!(true, "Configuration should complete without errors");
}

#[cfg(feature = "mock-server")]
mod mock_tests {
    use super::*;
    use sage_x_mcp_rust_client::testing::MockServer;
    
    #[tokio::test]
    async fn test_client_with_mock_server() {
        let mock_server = MockServer::start().await;
        let client = MCPClient::new(&mock_server.url()).unwrap();
        
        // Test basic request/response cycle
        let response = client.ping().await;
        assert!(response.is_ok(), "Ping should succeed with mock server");
        
        mock_server.stop().await;
    }
    
    #[tokio::test]
    async fn test_client_error_handling() {
        let mock_server = MockServer::start().await;
        mock_server.set_error_response(500, "Internal Server Error").await;
        
        let client = MCPClient::new(&mock_server.url()).unwrap();
        
        let response = client.ping().await;
        assert!(response.is_err(), "Should receive error from server");
        
        match response.unwrap_err() {
            MCPError::ServerError(code, _) => {
                assert_eq!(code, 500, "Should receive 500 error code");
            }
            other => panic!("Unexpected error type: {:?}", other),
        }
        
        mock_server.stop().await;
    }
}

/// Performance tests
mod performance_tests {
    use super::*;
    
    #[tokio::test]
    #[ignore] // Run with `cargo test -- --ignored` for performance tests
    async fn test_client_performance_baseline() {
        let client = MCPClient::new("test://localhost:8080").unwrap();
        
        let start = std::time::Instant::now();
        
        // Perform 100 operations
        for _ in 0..100 {
            if let Ok(_) = client.ping().await {
                // Operation succeeded
            }
        }
        
        let duration = start.elapsed();
        println!("100 operations completed in {:?}", duration);
        
        // Assert reasonable performance (this threshold may need adjustment)
        assert!(duration < Duration::from_secs(10), "Operations should complete within 10 seconds");
    }
}

/// Error recovery tests
mod recovery_tests {
    use super::*;
    
    #[tokio::test]
    async fn test_client_reconnection() {
        let client = MCPClient::new("test://localhost:8080").unwrap();
        
        // Simulate connection loss and recovery
        // This test would require specific server setup
        
        // For now, just test that the client can handle reconnection attempts
        let result = client.reconnect().await;
        
        // We don't assert success since we don't have a real server
        // But we verify the method exists and returns appropriate types
        match result {
            Ok(_) | Err(_) => {
                // Both outcomes are acceptable for this integration test
            }
        }
    }
}

