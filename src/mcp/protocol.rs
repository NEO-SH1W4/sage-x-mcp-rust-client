//! Core MCP Protocol Implementation
//!
//! Implementa as funcionalidades principais do protocolo MCP.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use serde::{Deserialize, Serialize};
use tokio::sync::{RwLock, mpsc};
use uuid::Uuid;

use crate::error::{SageXError, SageXResult};
use super::messages::{McpMessage, McpRequest, McpResponse, McpNotification};
use super::transport::{Transport, TransportType};

/// Representação de uma conexão MCP
#[derive(Debug)]
pub struct McpConnection {
    /// ID único da conexão
    pub id: Uuid,
    
    /// Transporte usado pela conexão
    transport: Box<dyn Transport>,
    
    /// Capacidades negociadas
    capabilities: McpCapabilities,
    
    /// Estado da conexão
    state: Arc<RwLock<ConnectionState>>,
    
    /// Canal para mensagens recebidas
    message_sender: mpsc::UnboundedSender<McpMessage>,
    
    /// Canal para notificações
    notification_sender: mpsc::UnboundedSender<McpNotification>,
    
    /// Requests pendentes
    pending_requests: Arc<RwLock<HashMap<String, PendingRequest>>>,
}

/// Estado da conexão MCP
#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionState {
    /// Conectando
    Connecting,
    /// Conectado e operacional
    Connected,
    /// Desconectando
    Disconnecting,
    /// Desconectado
    Disconnected,
    /// Erro na conexão
    Error(String),
}

/// Capacidades do servidor/cliente MCP
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpCapabilities {
    /// Versão do protocolo
    pub protocol_version: String,
    
    /// Suporte a tools
    pub tools: bool,
    
    /// Suporte a resources
    pub resources: bool,
    
    /// Suporte a prompts
    pub prompts: bool,
    
    /// Suporte a notificações
    pub notifications: bool,
    
    /// Suporte a streaming
    pub streaming: bool,
    
    /// Suporte a logging
    pub logging: bool,
    
    /// Extensões customizadas
    pub extensions: HashMap<String, serde_json::Value>,
}

impl Default for McpCapabilities {
    fn default() -> Self {
        Self {
            protocol_version: super::MCP_VERSION.to_string(),
            tools: true,
            resources: true,
            prompts: true,
            notifications: true,
            streaming: false,
            logging: true,
            extensions: HashMap::new(),
        }
    }
}

/// Request pendente aguardando resposta
#[derive(Debug)]
struct PendingRequest {
    /// Timestamp do request
    timestamp: SystemTime,
    
    /// Sender para a resposta
    response_sender: tokio::sync::oneshot::Sender<McpResponse>,
    
    /// Timeout do request
    timeout: Duration,
}

impl McpConnection {
    /// Cria uma nova conexão MCP
    pub async fn new(
        transport: Box<dyn Transport>,
        capabilities: McpCapabilities,
    ) -> SageXResult<Self> {
        let id = Uuid::new_v4();
        let state = Arc::new(RwLock::new(ConnectionState::Connecting));
        
        let (message_sender, _message_receiver) = mpsc::unbounded_channel();
        let (notification_sender, _notification_receiver) = mpsc::unbounded_channel();
        
        let connection = Self {
            id,
            transport,
            capabilities,
            state,
            message_sender,
            notification_sender,
            pending_requests: Arc::new(RwLock::new(HashMap::new())),
        };
        
        Ok(connection)
    }
    
    /// Inicia a conexão
    pub async fn connect(&mut self) -> SageXResult<()> {
        {
            let mut state = self.state.write().await;
            *state = ConnectionState::Connecting;
        }
        
        // Inicializar transporte
        self.transport.initialize().await?;
        
        // Enviar handshake
        let handshake_request = McpRequest {
            id: Uuid::new_v4().to_string(),
            method: "initialize".to_string(),
            params: Some(serde_json::to_value(&self.capabilities)?),
        };
        
        let _response = self.send_request(handshake_request).await?;
        
        {
            let mut state = self.state.write().await;
            *state = ConnectionState::Connected;
        }
        
        Ok(())
    }
    
    /// Envia um request e aguarda resposta
    pub async fn send_request(&self, request: McpRequest) -> SageXResult<McpResponse> {
        let (response_sender, response_receiver) = tokio::sync::oneshot::channel();
        
        let pending_request = PendingRequest {
            timestamp: SystemTime::now(),
            response_sender,
            timeout: Duration::from_secs(30),
        };
        
        // Armazenar request pendente
        {
            let mut pending = self.pending_requests.write().await;
            pending.insert(request.id.clone(), pending_request);
        }
        
        // Enviar request através do transporte
        let message = McpMessage::Request(request.clone());
        self.transport.send_message(message).await?;
        
        // Aguardar resposta ou timeout
        let response = tokio::time::timeout(
            Duration::from_secs(30),
            response_receiver
        ).await;
        
        // Remover da lista de pendentes
        {
            let mut pending = self.pending_requests.write().await;
            pending.remove(&request.id);
        }
        
        match response {
            Ok(Ok(response)) => Ok(response),
            Ok(Err(_)) => Err(SageXError::mcp_protocol("Canal de resposta fechado")),
            Err(_) => Err(SageXError::timeout(30, "Request MCP")),
        }
    }
    
    /// Envia uma notificação
    pub async fn send_notification(&self, notification: McpNotification) -> SageXResult<()> {
        let message = McpMessage::Notification(notification);
        self.transport.send_message(message).await?;
        Ok(())
    }
    
    /// Envia uma resposta
    pub async fn send_response(&self, response: McpResponse) -> SageXResult<()> {
        let message = McpMessage::Response(response);
        self.transport.send_message(message).await?;
        Ok(())
    }
    
    /// Processa uma mensagem recebida
    pub async fn handle_message(&self, message: McpMessage) -> SageXResult<()> {
        match message {
            McpMessage::Request(request) => {
                // Encaminhar para handler de requests
                self.handle_request(request).await?;
            }
            
            McpMessage::Response(response) => {
                // Localizar request pendente correspondente
                let pending_request = {
                    let mut pending = self.pending_requests.write().await;
                    pending.remove(&response.id)
                };
                
                if let Some(pending) = pending_request {
                    let _ = pending.response_sender.send(response);
                }
            }
            
            McpMessage::Notification(notification) => {
                // Enviar através do canal de notificações
                let _ = self.notification_sender.send(notification);
            }
        }
        
        Ok(())
    }
    
    /// Manipula um request recebido
    async fn handle_request(&self, request: McpRequest) -> SageXResult<()> {
        // Implementação básica - em uma versão completa isso seria
        // despachado para handlers específicos por método
        
        let response = match request.method.as_str() {
            "ping" => McpResponse {
                id: request.id,
                result: Some(serde_json::json!({"pong": true})),
                error: None,
            },
            
            "capabilities" => McpResponse {
                id: request.id,
                result: Some(serde_json::to_value(&self.capabilities)?),
                error: None,
            },
            
            _ => McpResponse {
                id: request.id,
                result: None,
                error: Some(crate::models::McpError {
                    code: -32601,
                    message: "Method not found".to_string(),
                    data: None,
                }),
            }
        };
        
        self.send_response(response).await
    }
    
    /// Obtém o estado atual da conexão
    pub async fn state(&self) -> ConnectionState {
        self.state.read().await.clone()
    }
    
    /// Verifica se a conexão está ativa
    pub async fn is_connected(&self) -> bool {
        matches!(self.state().await, ConnectionState::Connected)
    }
    
    /// Obtém as capacidades negociadas
    pub fn capabilities(&self) -> &McpCapabilities {
        &self.capabilities
    }
    
    /// Obtém ID da conexão
    pub fn id(&self) -> Uuid {
        self.id
    }
    
    /// Fecha a conexão
    pub async fn disconnect(&mut self) -> SageXResult<()> {
        {
            let mut state = self.state.write().await;
            *state = ConnectionState::Disconnecting;
        }
        
        self.transport.close().await?;
        
        {
            let mut state = self.state.write().await;
            *state = ConnectionState::Disconnected;
        }
        
        Ok(())
    }
    
    /// Limpa requests pendentes que expiraram
    pub async fn cleanup_expired_requests(&self) {
        let now = SystemTime::now();
        let mut expired_ids = Vec::new();
        
        {
            let pending = self.pending_requests.read().await;
            for (id, request) in pending.iter() {
                if let Ok(elapsed) = now.duration_since(request.timestamp) {
                    if elapsed > request.timeout {
                        expired_ids.push(id.clone());
                    }
                }
            }
        }
        
        if !expired_ids.is_empty() {
            let mut pending = self.pending_requests.write().await;
            for id in expired_ids {
                pending.remove(&id);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mcp::transport::MockTransport;
    
    #[tokio::test]
    async fn test_connection_creation() {
        let transport = Box::new(MockTransport::new());
        let capabilities = McpCapabilities::default();
        
        let connection = McpConnection::new(transport, capabilities).await;
        assert!(connection.is_ok());
        
        let conn = connection.unwrap();
        assert!(matches!(conn.state().await, ConnectionState::Connecting));
    }
    
    #[tokio::test]
    async fn test_capabilities() {
        let transport = Box::new(MockTransport::new());
        let mut capabilities = McpCapabilities::default();
        capabilities.tools = true;
        capabilities.resources = false;
        
        let connection = McpConnection::new(transport, capabilities.clone()).await.unwrap();
        
        assert_eq!(connection.capabilities().tools, true);
        assert_eq!(connection.capabilities().resources, false);
        assert_eq!(connection.capabilities().protocol_version, super::super::MCP_VERSION);
    }
}

