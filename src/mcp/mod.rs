//! Protocolo MCP (Model Context Protocol) para SAGE-X
//!
//! Implementa o protocolo MCP com extensões específicas para o sistema SAGE-X.

pub mod protocol;
pub mod messages;
pub mod transport;

// Re-exportações principais
pub use protocol::{McpConnection, McpCapabilities};
pub use messages::{McpMessage, McpRequest, McpResponse, McpNotification};
pub use transport::{Transport, TransportType, HttpTransport, StdioTransport};

/// Versão do protocolo MCP suportada
pub const MCP_VERSION: &str = "1.0.0";

/// Namespace para extensões SAGE-X
pub const SAGE_X_NAMESPACE: &str = "sage-x";

