//! # SAGE-X MCP Rust Client
//!
//! 🦀 Cliente Rust moderno para integração com capacidades MCP (Model Context Protocol) avançadas,
//! sistema de regras adaptativos e bridge simbiótico Python-Rust.
//!
//! ## Características
//!
//! - **MCP Enhanced**: Integração completa com Model Context Protocol
//! - **Rules Engine**: Sistema de regras adaptativos e contextuais
//! - **Bridge Simbiótico**: Conectividade Python-Rust de alta performance
//! - **Event Streaming**: Server-Sent Events para atualizações em tempo real
//! - **Cache Inteligente**: Sistema de cache com ETag e versionamento
//!
//! ## Quick Start
//!
//! ```rust
//! use sage_x_mcp_client::{SageXMcpClient, ClientConfig, Credentials};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Configuração do cliente
//! let config = ClientConfig::builder()
//!     .api_url("http://localhost:8001")
//!     .credentials(Credentials::new("sage_x_agent", "agent_secret"))
//!     .use_sse(true)
//!     .cache_enabled(true)
//!     .build()?;
//!
//! // Inicialização do cliente
//! let mut client = SageXMcpClient::new(config);
//! client.init().await?;
//!
//! // Criar contexto do agente
//! let mut context = client.create_agent_context("sage_x_001", "SAGE-X Agent")?;
//!
//! // Buscar e aplicar regras
//! let rules = client.fetch_rules().await?;
//! let results = client.apply_rules(&mut context).await?;
//!
//! // Enviar resultados de volta
//! client.send_results(&context).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Features
//!
//! - `mcp-enhanced`: Capacidades MCP avançadas (padrão)
//! - `rules-engine`: Motor de regras adaptativos (padrão)
//! - `python-bridge`: Bridge Python-Rust via PyO3
//! - `wasm-support`: Compilação para WebAssembly
//! - `dev-tools`: Ferramentas de desenvolvimento

#![warn(missing_docs)]
#![warn(clippy::all)]
#![allow(clippy::module_inception)]

pub mod client;
pub mod error;
pub mod mcp;
pub mod models;
pub mod rules;
pub mod sync;

// Re-exportações públicas principais
pub use client::{ClientConfig, SageXMcpClient};
pub use error::{SageXError, SageXResult};
pub use models::{
    AgentContext, Credentials, McpMessage, McpRequest, McpResponse, Rule, RuleResult, Token,
};

// Re-exportações condicionais por features
#[cfg(feature = "python-bridge")]
pub use mcp::bridge::PythonBridge;

#[cfg(feature = "wasm-support")]
pub use mcp::wasm::WasmBridge;

/// Versão da biblioteca
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Nome da biblioteca
pub const LIB_NAME: &str = env!("CARGO_PKG_NAME");

/// Descrição da biblioteca
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

/// User-Agent padrão para requisições HTTP
pub const USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " (Rust SAGE-X MCP Client)"
);

/// Configuração padrão para desenvolvimento
pub fn default_dev_config() -> ClientConfig {
    ClientConfig::builder()
        .api_url("http://localhost:8001")
        .use_sse(true)
        .cache_enabled(true)
        .timeout_seconds(30)
        .retry_attempts(3)
        .build()
        .expect("Configuração padrão deve ser válida")
}

/// Configuração padrão para produção
pub fn default_prod_config() -> ClientConfig {
    ClientConfig::builder()
        .api_url("https://api.sage-x.ai")
        .use_sse(true)
        .cache_enabled(true)
        .timeout_seconds(60)
        .retry_attempts(5)
        .connection_pool_size(10)
        .build()
        .expect("Configuração padrão deve ser válida")
}

/// Utilitário para logging configurado
pub fn init_logging(level: log::LevelFilter) {
    env_logger::Builder::from_default_env()
        .filter_level(level)
        .format_timestamp_secs()
        .init();
}

/// Macro para criar um contexto de agente rapidamente
#[macro_export]
macro_rules! agent_context {
    ($id:expr, $name:expr) => {
        $crate::AgentContext::new($id.to_string(), $name.to_string())
    };
    ($id:expr, $name:expr, $($key:expr => $value:expr),+ $(,)?) => {{
        let mut context = $crate::AgentContext::new($id.to_string(), $name.to_string());
        $(
            context.set_state($key, $value);
        )+
        context
    }};
}

/// Macro para configuração rápida do cliente
#[macro_export]
macro_rules! sage_client {
    ($api_url:expr) => {
        $crate::SageXMcpClient::new(
            $crate::ClientConfig::builder()
                .api_url($api_url)
                .build()
                .expect("Configuração deve ser válida")
        )
    };
    ($api_url:expr, $client_id:expr, $client_secret:expr) => {
        $crate::SageXMcpClient::new(
            $crate::ClientConfig::builder()
                .api_url($api_url)
                .credentials($crate::Credentials::new($client_id, $client_secret))
                .build()
                .expect("Configuração deve ser válida")
        )
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_info() {
        assert!(!VERSION.is_empty());
        assert!(!LIB_NAME.is_empty());
        assert!(!DESCRIPTION.is_empty());
        assert!(USER_AGENT.contains(VERSION));
    }

    #[test]
    fn test_default_configs() {
        let dev_config = default_dev_config();
        assert_eq!(dev_config.api_url(), "http://localhost:8001");
        assert!(dev_config.use_sse());

        let prod_config = default_prod_config();
        assert_eq!(prod_config.api_url(), "https://api.sage-x.ai");
        assert!(prod_config.use_sse());
    }

    #[test]
    fn test_agent_context_macro() {
        let context = agent_context!("test_id", "Test Agent");
        assert_eq!(context.agent_id(), "test_id");
        assert_eq!(context.agent_name(), "Test Agent");

        let context_with_state = agent_context!(
            "test_id", 
            "Test Agent",
            "key1" => serde_json::json!("value1"),
            "key2" => serde_json::json!(42)
        );
        assert_eq!(context_with_state.agent_id(), "test_id");
        assert!(context_with_state.get_state("key1").is_some());
        assert!(context_with_state.get_state("key2").is_some());
    }

    #[test]
    fn test_sage_client_macro() {
        let client = sage_client!("http://localhost:8001");
        assert_eq!(client.config().api_url(), "http://localhost:8001");

        let client_with_creds = sage_client!(
            "http://localhost:8001", 
            "test_client", 
            "test_secret"
        );
        assert!(client_with_creds.config().credentials().is_some());
    }
}

