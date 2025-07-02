//! Definições de mensagens MCP
//!
//! Tipos de mensagem padronizados do protocolo MCP.

use serde::{Deserialize, Serialize};
use crate::models::{McpError, UnixTimestamp};

/// Envelope para todas as mensagens MCP
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum McpMessage {
    /// Request - solicita uma ação do servidor
    Request(McpRequest),
    
    /// Response - resposta a um request
    Response(McpResponse),
    
    /// Notification - notificação unidirecional
    Notification(McpNotification),
}

/// Request MCP
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpRequest {
    /// ID único do request
    pub id: String,
    
    /// Método a ser executado
    pub method: String,
    
    /// Parâmetros do método
    pub params: Option<serde_json::Value>,
}

/// Response MCP
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpResponse {
    /// ID do request correspondente
    pub id: String,
    
    /// Resultado (se sucesso)
    pub result: Option<serde_json::Value>,
    
    /// Erro (se falha)
    pub error: Option<McpError>,
}

/// Notificação MCP
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpNotification {
    /// Método da notificação
    pub method: String,
    
    /// Parâmetros da notificação
    pub params: Option<serde_json::Value>,
    
    /// Timestamp da notificação
    pub timestamp: Option<UnixTimestamp>,
}

impl McpMessage {
    /// Cria um novo request
    pub fn new_request(id: String, method: String, params: Option<serde_json::Value>) -> Self {
        Self::Request(McpRequest { id, method, params })
    }
    
    /// Cria uma nova response de sucesso
    pub fn new_success_response(id: String, result: serde_json::Value) -> Self {
        Self::Response(McpResponse {
            id,
            result: Some(result),
            error: None,
        })
    }
    
    /// Cria uma nova response de erro
    pub fn new_error_response(id: String, error: McpError) -> Self {
        Self::Response(McpResponse {
            id,
            result: None,
            error: Some(error),
        })
    }
    
    /// Cria uma nova notificação
    pub fn new_notification(method: String, params: Option<serde_json::Value>) -> Self {
        Self::Notification(McpNotification {
            method,
            params,
            timestamp: Some(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
            ),
        })
    }
    
    /// Verifica se é um request
    pub fn is_request(&self) -> bool {
        matches!(self, Self::Request(_))
    }
    
    /// Verifica se é uma response
    pub fn is_response(&self) -> bool {
        matches!(self, Self::Response(_))
    }
    
    /// Verifica se é uma notificação
    pub fn is_notification(&self) -> bool {
        matches!(self, Self::Notification(_))
    }
    
    /// Obtém o ID se for request ou response
    pub fn id(&self) -> Option<&str> {
        match self {
            Self::Request(req) => Some(&req.id),
            Self::Response(resp) => Some(&resp.id),
            Self::Notification(_) => None,
        }
    }
    
    /// Obtém o método
    pub fn method(&self) -> Option<&str> {
        match self {
            Self::Request(req) => Some(&req.method),
            Self::Response(_) => None,
            Self::Notification(notif) => Some(&notif.method),
        }
    }
}

impl McpRequest {
    /// Cria um novo request
    pub fn new(id: String, method: String, params: Option<serde_json::Value>) -> Self {
        Self { id, method, params }
    }
    
    /// Cria um request ping
    pub fn ping(id: String) -> Self {
        Self::new(id, "ping".to_string(), None)
    }
    
    /// Cria um request de inicialização
    pub fn initialize(id: String, capabilities: serde_json::Value) -> Self {
        Self::new(id, "initialize".to_string(), Some(capabilities))
    }
    
    /// Cria um request para listar tools
    pub fn list_tools(id: String) -> Self {
        Self::new(id, "tools/list".to_string(), None)
    }
    
    /// Cria um request para executar tool
    pub fn call_tool(id: String, tool_name: String, arguments: serde_json::Value) -> Self {
        Self::new(
            id,
            "tools/call".to_string(),
            Some(serde_json::json!({
                "name": tool_name,
                "arguments": arguments
            }))
        )
    }
    
    /// Cria um request para listar resources
    pub fn list_resources(id: String) -> Self {
        Self::new(id, "resources/list".to_string(), None)
    }
    
    /// Cria um request para obter resource
    pub fn read_resource(id: String, uri: String) -> Self {
        Self::new(
            id,
            "resources/read".to_string(),
            Some(serde_json::json!({ "uri": uri }))
        )
    }
}

impl McpResponse {
    /// Cria uma response de sucesso
    pub fn success(id: String, result: serde_json::Value) -> Self {
        Self {
            id,
            result: Some(result),
            error: None,
        }
    }
    
    /// Cria uma response de erro
    pub fn error(id: String, code: i32, message: String, data: Option<serde_json::Value>) -> Self {
        Self {
            id,
            result: None,
            error: Some(McpError { code, message, data }),
        }
    }
    
    /// Verifica se é uma response de sucesso
    pub fn is_success(&self) -> bool {
        self.error.is_none()
    }
    
    /// Verifica se é uma response de erro
    pub fn is_error(&self) -> bool {
        self.error.is_some()
    }
}

impl McpNotification {
    /// Cria uma nova notificação
    pub fn new(method: String, params: Option<serde_json::Value>) -> Self {
        Self {
            method,
            params,
            timestamp: Some(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
            ),
        }
    }
    
    /// Cria notificação de progress
    pub fn progress(progress_token: String, work_done: u64, total_work: Option<u64>) -> Self {
        Self::new(
            "notifications/progress".to_string(),
            Some(serde_json::json!({
                "progressToken": progress_token,
                "value": {
                    "kind": "report",
                    "percentage": if let Some(total) = total_work {
                        (work_done as f64 / total as f64 * 100.0) as u8
                    } else {
                        0
                    }
                }
            }))
        )
    }
    
    /// Cria notificação de log
    pub fn log(level: LogLevel, message: String, data: Option<serde_json::Value>) -> Self {
        Self::new(
            "notifications/message".to_string(),
            Some(serde_json::json!({
                "level": level,
                "logger": "sage-x-mcp",
                "data": message,
                "extra": data
            }))
        )
    }
    
    /// Cria notificação de resource atualizado
    pub fn resource_updated(uri: String) -> Self {
        Self::new(
            "notifications/resources/updated".to_string(),
            Some(serde_json::json!({ "uri": uri }))
        )
    }
}

/// Nível de log para notificações
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    /// Debug
    Debug,
    /// Info
    Info,
    /// Warning
    Warning,
    /// Error
    Error,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_message_creation() {
        let request = McpMessage::new_request(
            "test-1".to_string(),
            "ping".to_string(),
            None
        );
        
        assert!(request.is_request());
        assert_eq!(request.id(), Some("test-1"));
        assert_eq!(request.method(), Some("ping"));
    }
    
    #[test]
    fn test_request_methods() {
        let ping = McpRequest::ping("ping-1".to_string());
        assert_eq!(ping.method, "ping");
        assert_eq!(ping.id, "ping-1");
        
        let list_tools = McpRequest::list_tools("tools-1".to_string());
        assert_eq!(list_tools.method, "tools/list");
    }
    
    #[test]
    fn test_response_creation() {
        let success = McpResponse::success(
            "test-1".to_string(),
            serde_json::json!({"status": "ok"})
        );
        assert!(success.is_success());
        assert!(!success.is_error());
        
        let error = McpResponse::error(
            "test-2".to_string(),
            -32600,
            "Invalid Request".to_string(),
            None
        );
        assert!(!error.is_success());
        assert!(error.is_error());
    }
    
    #[test]
    fn test_notification_creation() {
        let notification = McpNotification::new(
            "test/notification".to_string(),
            Some(serde_json::json!({"test": true}))
        );
        
        assert_eq!(notification.method, "test/notification");
        assert!(notification.timestamp.is_some());
        
        let log_notif = McpNotification::log(
            LogLevel::Info,
            "Test message".to_string(),
            None
        );
        assert_eq!(log_notif.method, "notifications/message");
    }
    
    #[test]
    fn test_serialization() {
        let request = McpMessage::new_request(
            "test-1".to_string(),
            "ping".to_string(),
            None
        );
        
        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: McpMessage = serde_json::from_str(&serialized).unwrap();
        
        assert!(deserialized.is_request());
        assert_eq!(deserialized.id(), Some("test-1"));
    }
}

