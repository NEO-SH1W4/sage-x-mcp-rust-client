//! Sistema de erros para SAGE-X MCP Client
//!
//! Define tipos de erro específicos e utilitários para handling de erros
//! na integração com MCP e sistema de regras.

use std::fmt;

use thiserror::Error;

/// Tipo de resultado padrão para a biblioteca
pub type SageXResult<T> = Result<T, SageXError>;

/// Erros específicos do SAGE-X MCP Client
#[derive(Error, Debug)]
pub enum SageXError {
    /// Erro de autenticação ou autorização
    #[error("Erro de autenticação: {message}")]
    Authentication { message: String },

    /// Erro de conexão de rede
    #[error("Erro de conexão: {message}")]
    Connection { message: String },

    /// Erro ao processar regras
    #[error("Erro ao processar regra '{rule_id}': {message}")]
    RuleProcessing { rule_id: String, message: String },

    /// Erro de cache
    #[error("Erro de cache: {message}")]
    Cache { message: String },

    /// Erro de configuração
    #[error("Erro de configuração: {message}")]
    Configuration { message: String },

    /// Erro do protocolo MCP
    #[error("Erro de protocolo MCP: {message}")]
    McpProtocol { message: String },

    /// Erro de serialização/deserialização
    #[error("Erro de serialização: {message}")]
    Serialization { message: String },

    /// Erro de bridge Python
    #[cfg(feature = "python-bridge")]
    #[error("Erro no bridge Python: {message}")]
    PythonBridge { message: String },

    /// Erro de WASM
    #[cfg(feature = "wasm-support")]
    #[error("Erro WASM: {message}")]
    Wasm { message: String },

    /// Erro de timeout
    #[error("Timeout após {seconds}s: {operation}")]
    Timeout { seconds: u64, operation: String },

    /// Erro de validação
    #[error("Erro de validação: {field} - {message}")]
    Validation { field: String, message: String },

    /// Erro de I/O
    #[error("Erro de I/O: {0}")]
    Io(#[from] std::io::Error),

    /// Erro HTTP
    #[error("Erro HTTP: {0}")]
    Http(#[from] reqwest::Error),

    /// Erro JSON
    #[error("Erro JSON: {0}")]
    Json(#[from] serde_json::Error),

    /// Erro JWT
    #[error("Erro JWT: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    /// Erro genérico
    #[error("Erro interno: {0}")]
    Internal(#[from] anyhow::Error),

    /// Erro desconhecido
    #[error("Erro desconhecido: {message}")]
    Unknown { message: String },
}

impl SageXError {
    /// Cria um erro de autenticação
    pub fn authentication<S: Into<String>>(message: S) -> Self {
        Self::Authentication {
            message: message.into(),
        }
    }

    /// Cria um erro de conexão
    pub fn connection<S: Into<String>>(message: S) -> Self {
        Self::Connection {
            message: message.into(),
        }
    }

    /// Cria um erro de processamento de regra
    pub fn rule_processing<S1: Into<String>, S2: Into<String>>(rule_id: S1, message: S2) -> Self {
        Self::RuleProcessing {
            rule_id: rule_id.into(),
            message: message.into(),
        }
    }

    /// Cria um erro de cache
    pub fn cache<S: Into<String>>(message: S) -> Self {
        Self::Cache {
            message: message.into(),
        }
    }

    /// Cria um erro de configuração
    pub fn configuration<S: Into<String>>(message: S) -> Self {
        Self::Configuration {
            message: message.into(),
        }
    }

    /// Cria um erro de protocolo MCP
    pub fn mcp_protocol<S: Into<String>>(message: S) -> Self {
        Self::McpProtocol {
            message: message.into(),
        }
    }

    /// Cria um erro de serialização
    pub fn serialization<S: Into<String>>(message: S) -> Self {
        Self::Serialization {
            message: message.into(),
        }
    }

    /// Cria um erro de timeout
    pub fn timeout<S: Into<String>>(seconds: u64, operation: S) -> Self {
        Self::Timeout {
            seconds,
            operation: operation.into(),
        }
    }

    /// Cria um erro de validação
    pub fn validation<S1: Into<String>, S2: Into<String>>(field: S1, message: S2) -> Self {
        Self::Validation {
            field: field.into(),
            message: message.into(),
        }
    }

    /// Cria um erro desconhecido
    pub fn unknown<S: Into<String>>(message: S) -> Self {
        Self::Unknown {
            message: message.into(),
        }
    }

    /// Cria um erro de bridge Python
    #[cfg(feature = "python-bridge")]
    pub fn python_bridge<S: Into<String>>(message: S) -> Self {
        Self::PythonBridge {
            message: message.into(),
        }
    }

    /// Cria um erro WASM
    #[cfg(feature = "wasm-support")]
    pub fn wasm<S: Into<String>>(message: S) -> Self {
        Self::Wasm {
            message: message.into(),
        }
    }

    /// Verifica se o erro é recuperável
    pub fn is_recoverable(&self) -> bool {
        match self {
            Self::Connection { .. }
            | Self::Timeout { .. }
            | Self::Http(_)
            | Self::Cache { .. } => true,
            Self::Authentication { .. }
            | Self::Configuration { .. }
            | Self::Validation { .. }
            | Self::Serialization { .. } => false,
            Self::RuleProcessing { .. } | Self::McpProtocol { .. } => true,
            Self::Io(_) | Self::Json(_) | Self::Jwt(_) => false,
            Self::Internal(_) | Self::Unknown { .. } => false,
            #[cfg(feature = "python-bridge")]
            Self::PythonBridge { .. } => true,
            #[cfg(feature = "wasm-support")]
            Self::Wasm { .. } => true,
        }
    }

    /// Retorna a categoria do erro
    pub fn category(&self) -> ErrorCategory {
        match self {
            Self::Authentication { .. } => ErrorCategory::Authentication,
            Self::Connection { .. } | Self::Http(_) => ErrorCategory::Network,
            Self::RuleProcessing { .. } => ErrorCategory::Rules,
            Self::Cache { .. } => ErrorCategory::Cache,
            Self::Configuration { .. } | Self::Validation { .. } => ErrorCategory::Configuration,
            Self::McpProtocol { .. } => ErrorCategory::Protocol,
            Self::Serialization { .. } | Self::Json(_) => ErrorCategory::Serialization,
            Self::Timeout { .. } => ErrorCategory::Timeout,
            Self::Io(_) => ErrorCategory::Io,
            Self::Jwt(_) => ErrorCategory::Security,
            Self::Internal(_) | Self::Unknown { .. } => ErrorCategory::Internal,
            #[cfg(feature = "python-bridge")]
            Self::PythonBridge { .. } => ErrorCategory::Bridge,
            #[cfg(feature = "wasm-support")]
            Self::Wasm { .. } => ErrorCategory::Bridge,
        }
    }

    /// Retorna código de erro para logging/telemetria
    pub fn error_code(&self) -> &'static str {
        match self {
            Self::Authentication { .. } => "AUTH_001",
            Self::Connection { .. } => "CONN_001",
            Self::RuleProcessing { .. } => "RULE_001",
            Self::Cache { .. } => "CACHE_001",
            Self::Configuration { .. } => "CONFIG_001",
            Self::McpProtocol { .. } => "MCP_001",
            Self::Serialization { .. } => "SERIAL_001",
            Self::Timeout { .. } => "TIMEOUT_001",
            Self::Validation { .. } => "VALID_001",
            Self::Io(_) => "IO_001",
            Self::Http(_) => "HTTP_001",
            Self::Json(_) => "JSON_001",
            Self::Jwt(_) => "JWT_001",
            Self::Internal(_) => "INTERNAL_001",
            Self::Unknown { .. } => "UNKNOWN_001",
            #[cfg(feature = "python-bridge")]
            Self::PythonBridge { .. } => "PYTHON_001",
            #[cfg(feature = "wasm-support")]
            Self::Wasm { .. } => "WASM_001",
        }
    }
}

/// Categorias de erro para telemetria e debugging
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCategory {
    /// Erros de autenticação e autorização
    Authentication,
    /// Erros de rede e conectividade
    Network,
    /// Erros de processamento de regras
    Rules,
    /// Erros de cache
    Cache,
    /// Erros de configuração
    Configuration,
    /// Erros de protocolo MCP
    Protocol,
    /// Erros de serialização
    Serialization,
    /// Erros de timeout
    Timeout,
    /// Erros de I/O
    Io,
    /// Erros de segurança
    Security,
    /// Erros de bridge (Python/WASM)
    Bridge,
    /// Erros internos
    Internal,
}

impl fmt::Display for ErrorCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Authentication => write!(f, "Authentication"),
            Self::Network => write!(f, "Network"),
            Self::Rules => write!(f, "Rules"),
            Self::Cache => write!(f, "Cache"),
            Self::Configuration => write!(f, "Configuration"),
            Self::Protocol => write!(f, "Protocol"),
            Self::Serialization => write!(f, "Serialization"),
            Self::Timeout => write!(f, "Timeout"),
            Self::Io => write!(f, "IO"),
            Self::Security => write!(f, "Security"),
            Self::Bridge => write!(f, "Bridge"),
            Self::Internal => write!(f, "Internal"),
        }
    }
}

/// Trait auxiliar para conversão fácil de erros
pub trait IntoSageXError<T> {
    /// Converte Result em SageXResult
    fn into_sage_error(self) -> SageXResult<T>;
}

impl<T, E> IntoSageXError<T> for Result<T, E>
where
    E: Into<SageXError>,
{
    fn into_sage_error(self) -> SageXResult<T> {
        self.map_err(Into::into)
    }
}

/// Macro para criar erros contextualizados rapidamente
#[macro_export]
macro_rules! sage_error {
    (auth: $msg:expr) => {
        $crate::SageXError::authentication($msg)
    };
    (conn: $msg:expr) => {
        $crate::SageXError::connection($msg)
    };
    (rule: $rule_id:expr, $msg:expr) => {
        $crate::SageXError::rule_processing($rule_id, $msg)
    };
    (cache: $msg:expr) => {
        $crate::SageXError::cache($msg)
    };
    (config: $msg:expr) => {
        $crate::SageXError::configuration($msg)
    };
    (mcp: $msg:expr) => {
        $crate::SageXError::mcp_protocol($msg)
    };
    (timeout: $seconds:expr, $op:expr) => {
        $crate::SageXError::timeout($seconds, $op)
    };
    (valid: $field:expr, $msg:expr) => {
        $crate::SageXError::validation($field, $msg)
    };
    ($msg:expr) => {
        $crate::SageXError::unknown($msg)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let auth_error = SageXError::authentication("Token inválido");
        assert!(matches!(auth_error, SageXError::Authentication { .. }));
        assert_eq!(auth_error.category(), ErrorCategory::Authentication);
        assert_eq!(auth_error.error_code(), "AUTH_001");
        assert!(!auth_error.is_recoverable());

        let conn_error = SageXError::connection("Falha na conexão");
        assert!(matches!(conn_error, SageXError::Connection { .. }));
        assert_eq!(conn_error.category(), ErrorCategory::Network);
        assert!(conn_error.is_recoverable());
    }

    #[test]
    fn test_error_macro() {
        let auth_error = sage_error!(auth: "Token expirado");
        assert!(matches!(auth_error, SageXError::Authentication { .. }));

        let rule_error = sage_error!(rule: "rule_123", "Falha na aplicação");
        assert!(matches!(rule_error, SageXError::RuleProcessing { .. }));

        let timeout_error = sage_error!(timeout: 30, "Busca de regras");
        assert!(matches!(timeout_error, SageXError::Timeout { .. }));
    }

    #[test]
    fn test_error_categories() {
        use ErrorCategory::*;

        let categories = [
            Authentication, Network, Rules, Cache, Configuration,
            Protocol, Serialization, Timeout, Io, Security, Bridge, Internal,
        ];

        for category in categories {
            assert!(!category.to_string().is_empty());
        }
    }

    #[test]
    fn test_recoverable_errors() {
        let recoverable = SageXError::connection("test");
        assert!(recoverable.is_recoverable());

        let non_recoverable = SageXError::authentication("test");
        assert!(!non_recoverable.is_recoverable());
    }
}

