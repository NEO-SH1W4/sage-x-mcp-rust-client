//! Sistema de transporte para protocolo MCP
//!
//! Implementa diferentes tipos de transporte para comunicação MCP.

use std::fmt::Debug;
use std::sync::Arc;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, RwLock};

use crate::error::{SageXError, SageXResult};
use super::messages::McpMessage;

/// Trait para implementações de transporte MCP
#[async_trait]
pub trait Transport: Send + Sync + Debug {
    /// Inicializa o transporte
    async fn initialize(&mut self) -> SageXResult<()>;
    
    /// Envia uma mensagem
    async fn send_message(&self, message: McpMessage) -> SageXResult<()>;
    
    /// Recebe uma mensagem (não-bloqueante)
    async fn receive_message(&self) -> SageXResult<Option<McpMessage>>;
    
    /// Fecha o transporte
    async fn close(&mut self) -> SageXResult<()>;
    
    /// Verifica se está conectado
    async fn is_connected(&self) -> bool;
    
    /// Obtém o tipo de transporte
    fn transport_type(&self) -> TransportType;
}

/// Tipos de transporte disponíveis
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransportType {
    /// Standard I/O (stdin/stdout)
    Stdio,
    /// HTTP
    Http,
    /// WebSocket
    WebSocket,
    /// Mock (para testes)
    Mock,
}

/// Transporte HTTP para MCP
#[derive(Debug)]
pub struct HttpTransport {
    /// URL base do servidor
    base_url: String,
    
    /// Cliente HTTP
    client: reqwest::Client,
    
    /// Canal para mensagens recebidas
    incoming_messages: Arc<RwLock<mpsc::UnboundedReceiver<McpMessage>>>,
    
    /// Sender para mensagens recebidas
    message_sender: mpsc::UnboundedSender<McpMessage>,
    
    /// Estado da conexão
    connected: Arc<RwLock<bool>>,
}

impl HttpTransport {
    /// Cria um novo transporte HTTP
    pub fn new(base_url: String) -> Self {
        let (message_sender, message_receiver) = mpsc::unbounded_channel();
        
        Self {
            base_url,
            client: reqwest::Client::new(),
            incoming_messages: Arc::new(RwLock::new(message_receiver)),
            message_sender,
            connected: Arc::new(RwLock::new(false)),
        }
    }
    
    /// Constrói URL completa para endpoint
    fn build_url(&self, endpoint: &str) -> String {
        format!("{}/{}", self.base_url.trim_end_matches('/'), endpoint.trim_start_matches('/'))
    }
}

#[async_trait]
impl Transport for HttpTransport {
    async fn initialize(&mut self) -> SageXResult<()> {
        // Testar conectividade com endpoint de health
        let health_url = self.build_url("health");
        
        let response = self.client
            .get(&health_url)
            .timeout(std::time::Duration::from_secs(10))
            .send()
            .await
            .map_err(|e| SageXError::connection(format!("Falha ao conectar com {}: {}", health_url, e)))?;
        
        if response.status().is_success() {
            let mut connected = self.connected.write().await;
            *connected = true;
            Ok(())
        } else {
            Err(SageXError::connection(format!(
                "Servidor retornou status: {}",
                response.status()
            )))
        }
    }
    
    async fn send_message(&self, message: McpMessage) -> SageXResult<()> {
        if !self.is_connected().await {
            return Err(SageXError::connection("Transporte não conectado"));
        }
        
        let endpoint = match &message {
            McpMessage::Request(req) => format!("mcp/request/{}", req.method),
            McpMessage::Response(resp) => format!("mcp/response/{}", resp.id),
            McpMessage::Notification(notif) => format!("mcp/notification/{}", notif.method),
        };
        
        let url = self.build_url(&endpoint);
        
        let response = self.client
            .post(&url)
            .json(&message)
            .timeout(std::time::Duration::from_secs(30))
            .send()
            .await
            .map_err(|e| SageXError::connection(format!("Falha ao enviar mensagem: {}", e)))?;
        
        if !response.status().is_success() {
            return Err(SageXError::Http(format!(
                "Erro HTTP {}: {}",
                response.status(),
                response.text().await.unwrap_or_default()
            )));
        }
        
        Ok(())
    }
    
    async fn receive_message(&self) -> SageXResult<Option<McpMessage>> {
        let mut incoming = self.incoming_messages.write().await;
        Ok(incoming.try_recv().ok())
    }
    
    async fn close(&mut self) -> SageXResult<()> {
        let mut connected = self.connected.write().await;
        *connected = false;
        Ok(())
    }
    
    async fn is_connected(&self) -> bool {
        *self.connected.read().await
    }
    
    fn transport_type(&self) -> TransportType {
        TransportType::Http
    }
}

/// Transporte Standard I/O para MCP
#[derive(Debug)]
pub struct StdioTransport {
    /// Canal para mensagens recebidas
    incoming_messages: Arc<RwLock<mpsc::UnboundedReceiver<McpMessage>>>,
    
    /// Sender para mensagens recebidas
    message_sender: mpsc::UnboundedSender<McpMessage>,
    
    /// Estado da conexão
    connected: Arc<RwLock<bool>>,
}

impl StdioTransport {
    /// Cria um novo transporte Stdio
    pub fn new() -> Self {
        let (message_sender, message_receiver) = mpsc::unbounded_channel();
        
        Self {
            incoming_messages: Arc::new(RwLock::new(message_receiver)),
            message_sender,
            connected: Arc::new(RwLock::new(false)),
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
    async fn initialize(&mut self) -> SageXResult<()> {
        // Para stdio, apenas marcar como conectado
        let mut connected = self.connected.write().await;
        *connected = true;
        
        // TODO: Iniciar task para ler de stdin em background
        Ok(())
    }
    
    async fn send_message(&self, message: McpMessage) -> SageXResult<()> {
        if !self.is_connected().await {
            return Err(SageXError::connection("Transporte não conectado"));
        }
        
        // Serializar e enviar para stdout
        let json = serde_json::to_string(&message)
            .map_err(|e| SageXError::serialization(format!("Falha ao serializar mensagem: {}", e)))?;
        
        println!("{}", json);
        Ok(())
    }
    
    async fn receive_message(&self) -> SageXResult<Option<McpMessage>> {
        let mut incoming = self.incoming_messages.write().await;
        Ok(incoming.try_recv().ok())
    }
    
    async fn close(&mut self) -> SageXResult<()> {
        let mut connected = self.connected.write().await;
        *connected = false;
        Ok(())
    }
    
    async fn is_connected(&self) -> bool {
        *self.connected.read().await
    }
    
    fn transport_type(&self) -> TransportType {
        TransportType::Stdio
    }
}

/// Transporte Mock para testes
#[derive(Debug)]
pub struct MockTransport {
    /// Mensagens enviadas (para verificação em testes)
    sent_messages: Arc<RwLock<Vec<McpMessage>>>,
    
    /// Mensagens a serem recebidas (simuladas)
    mock_incoming: Arc<RwLock<Vec<McpMessage>>>,
    
    /// Estado da conexão
    connected: Arc<RwLock<bool>>,
    
    /// Simular falha na inicialização
    fail_init: bool,
    
    /// Simular falha no envio
    fail_send: bool,
}

impl MockTransport {
    /// Cria um novo transporte mock
    pub fn new() -> Self {
        Self {
            sent_messages: Arc::new(RwLock::new(Vec::new())),
            mock_incoming: Arc::new(RwLock::new(Vec::new())),
            connected: Arc::new(RwLock::new(false)),
            fail_init: false,
            fail_send: false,
        }
    }
    
    /// Configura para falhar na inicialização
    pub fn with_init_failure(mut self) -> Self {
        self.fail_init = true;
        self
    }
    
    /// Configura para falhar no envio
    pub fn with_send_failure(mut self) -> Self {
        self.fail_send = true;
        self
    }
    
    /// Adiciona uma mensagem para ser "recebida"
    pub async fn add_incoming_message(&self, message: McpMessage) {
        let mut incoming = self.mock_incoming.write().await;
        incoming.push(message);
    }
    
    /// Obtém todas as mensagens enviadas
    pub async fn sent_messages(&self) -> Vec<McpMessage> {
        self.sent_messages.read().await.clone()
    }
    
    /// Limpa mensagens enviadas
    pub async fn clear_sent_messages(&self) {
        let mut sent = self.sent_messages.write().await;
        sent.clear();
    }
}

impl Default for MockTransport {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Transport for MockTransport {
    async fn initialize(&mut self) -> SageXResult<()> {
        if self.fail_init {
            return Err(SageXError::connection("Mock init failure"));
        }
        
        let mut connected = self.connected.write().await;
        *connected = true;
        Ok(())
    }
    
    async fn send_message(&self, message: McpMessage) -> SageXResult<()> {
        if self.fail_send {
            return Err(SageXError::connection("Mock send failure"));
        }
        
        if !self.is_connected().await {
            return Err(SageXError::connection("Transporte não conectado"));
        }
        
        let mut sent = self.sent_messages.write().await;
        sent.push(message);
        Ok(())
    }
    
    async fn receive_message(&self) -> SageXResult<Option<McpMessage>> {
        let mut incoming = self.mock_incoming.write().await;
        Ok(incoming.pop())
    }
    
    async fn close(&mut self) -> SageXResult<()> {
        let mut connected = self.connected.write().await;
        *connected = false;
        Ok(())
    }
    
    async fn is_connected(&self) -> bool {
        *self.connected.read().await
    }
    
    fn transport_type(&self) -> TransportType {
        TransportType::Mock
    }
}

/// Factory para criar transportes
pub struct TransportFactory;

impl TransportFactory {
    /// Cria um transporte baseado no tipo
    pub fn create(transport_type: TransportType, config: Option<serde_json::Value>) -> SageXResult<Box<dyn Transport>> {
        match transport_type {
            TransportType::Http => {
                let base_url = if let Some(config) = config {
                    config.get("base_url")
                        .and_then(|v| v.as_str())
                        .unwrap_or("http://localhost:8080")
                        .to_string()
                } else {
                    "http://localhost:8080".to_string()
                };
                
                Ok(Box::new(HttpTransport::new(base_url)))
            }
            
            TransportType::Stdio => {
                Ok(Box::new(StdioTransport::new()))
            }
            
            TransportType::WebSocket => {
                // TODO: Implementar WebSocket transport
                Err(SageXError::configuration("WebSocket transport não implementado ainda"))
            }
            
            TransportType::Mock => {
                Ok(Box::new(MockTransport::new()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mcp::messages::McpRequest;
    
    #[tokio::test]
    async fn test_mock_transport() {
        let mut transport = MockTransport::new();
        
        // Inicializar
        transport.initialize().await.unwrap();
        assert!(transport.is_connected().await);
        
        // Enviar mensagem
        let request = McpMessage::Request(McpRequest::ping("test-1".to_string()));
        transport.send_message(request.clone()).await.unwrap();
        
        // Verificar que foi enviada
        let sent = transport.sent_messages().await;
        assert_eq!(sent.len(), 1);
        assert!(sent[0].is_request());
        
        // Adicionar mensagem incoming
        let response = McpMessage::new_success_response(
            "test-1".to_string(),
            serde_json::json!({"pong": true})
        );
        transport.add_incoming_message(response).await;
        
        // Receber mensagem
        let received = transport.receive_message().await.unwrap();
        assert!(received.is_some());
        assert!(received.unwrap().is_response());
        
        // Fechar
        transport.close().await.unwrap();
        assert!(!transport.is_connected().await);
    }
    
    #[tokio::test]
    async fn test_mock_transport_failures() {
        let mut transport = MockTransport::new().with_init_failure();
        
        // Falha na inicialização
        let result = transport.initialize().await;
        assert!(result.is_err());
        
        let mut transport = MockTransport::new().with_send_failure();
        transport.initialize().await.unwrap();
        
        // Falha no envio
        let request = McpMessage::Request(McpRequest::ping("test-1".to_string()));
        let result = transport.send_message(request).await;
        assert!(result.is_err());
    }
    
    #[tokio::test]
    async fn test_stdio_transport() {
        let mut transport = StdioTransport::new();
        
        // Inicializar
        transport.initialize().await.unwrap();
        assert!(transport.is_connected().await);
        assert_eq!(transport.transport_type(), TransportType::Stdio);
        
        // Fechar
        transport.close().await.unwrap();
        assert!(!transport.is_connected().await);
    }
    
    #[test]
    fn test_transport_factory() {
        let http_transport = TransportFactory::create(
            TransportType::Http,
            Some(serde_json::json!({"base_url": "http://test.com"}))
        ).unwrap();
        assert_eq!(http_transport.transport_type(), TransportType::Http);
        
        let stdio_transport = TransportFactory::create(TransportType::Stdio, None).unwrap();
        assert_eq!(stdio_transport.transport_type(), TransportType::Stdio);
        
        let mock_transport = TransportFactory::create(TransportType::Mock, None).unwrap();
        assert_eq!(mock_transport.transport_type(), TransportType::Mock);
        
        let websocket_result = TransportFactory::create(TransportType::WebSocket, None);
        assert!(websocket_result.is_err());
    }
}

