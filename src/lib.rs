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
// pub mod mcp;
// pub mod rules;
// pub mod sync;
pub mod models;

// Re-exportações públicas principais
pub use client::{SageXClient, SageXClientBuilder, SageXEvent};
pub use error::{SageXError, SageXResult};
pub use models::{
    SageXConfig, SessionContext, McpRequest, McpResponse, McpTool, McpResource,
    SageXRule, DevSession, ExecutionResult,
};

// Re-exportações condicionais por features
// #[cfg(feature = "python-bridge")]
// pub use mcp::bridge::PythonBridge;

// #[cfg(feature = "wasm-support")]
// pub use mcp::wasm::WasmBridge;

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
pub fn default_dev_config() -> SageXConfig {
    let mut config = SageXConfig::default();
    config.api_base_url = "http://localhost:8001".to_string();
    config
}

/// Configuração padrão para produção
pub fn default_prod_config() -> SageXConfig {
    let mut config = SageXConfig::default();
    config.api_base_url = "https://api.sage-x.ai".to_string();
    config
}

/// Utilitário para logging configurado
pub fn init_logging(level: log::LevelFilter) {
    env_logger::Builder::from_default_env()
        .filter_level(level)
        .format_timestamp_secs()
        .init();
}

/// Macro para criar configuração de cliente rapidamente
#[macro_export]
macro_rules! sage_config {
    ($api_url:expr) => {
        {
            let mut config = $crate::SageXConfig::default();
            config.api_base_url = $api_url.to_string();
            config
        }
    };
    ($api_url:expr, $token:expr) => {
        {
            let mut config = $crate::SageXConfig::default();
            config.api_base_url = $api_url.to_string();
            config.auth_token = $token.to_string();
            config
        }
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
        assert_eq!(dev_config.api_base_url, "http://localhost:8001");

        let prod_config = default_prod_config();
        assert_eq!(prod_config.api_base_url, "https://api.sage-x.ai");
    }

    #[test]
    fn test_sage_config_macro() {
        let config = sage_config!("http://localhost:8001");
        assert_eq!(config.api_base_url, "http://localhost:8001");

        let config_with_token = sage_config!(
            "http://localhost:8001", 
            "test_token"
        );
        assert_eq!(config_with_token.api_base_url, "http://localhost:8001");
        assert_eq!(config_with_token.auth_token, "test_token");
    }
}

