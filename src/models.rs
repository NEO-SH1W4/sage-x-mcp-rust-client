//! Modelos de dados para SAGE-X MCP Client
//!
//! Define as estruturas de dados para comunicação MCP, regras SAGE-X
//! e integração com o ecossistema WARP_RULES.

use std::collections::HashMap;
use std::time::{Duration, SystemTime};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::SageXResult;

/// Identificador único para recursos SAGE-X
pub type SageXId = Uuid;

/// Timestamp Unix em segundos
pub type UnixTimestamp = u64;

/// Configuração principal do cliente MCP
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SageXConfig {
    /// URL base da API WARP_RULES
    pub api_base_url: String,
    
    /// Token de autenticação
    pub auth_token: String,
    
    /// Configurações de cache
    pub cache: CacheConfig,
    
    /// Configurações de rede
    pub network: NetworkConfig,
    
    /// Configurações de regras
    pub rules: RulesConfig,
    
    /// Features habilitadas
    pub features: FeatureFlags,
    
    /// Configurações específicas do MCP
    pub mcp: McpConfig,
    
    /// Configurações de telemetria
    pub telemetry: TelemetryConfig,
}

impl Default for SageXConfig {
    fn default() -> Self {
        Self {
            api_base_url: "https://api.sage-x.dev".to_string(),
            auth_token: String::new(),
            cache: CacheConfig::default(),
            network: NetworkConfig::default(),
            rules: RulesConfig::default(),
            features: FeatureFlags::default(),
            mcp: McpConfig::default(),
            telemetry: TelemetryConfig::default(),
        }
    }
}

/// Configurações de cache
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// Tamanho máximo do cache em MB
    pub max_size_mb: usize,
    
    /// TTL padrão para regras em segundos
    pub default_ttl: Duration,
    
    /// TTL para dados de sessão
    pub session_ttl: Duration,
    
    /// Habilitar cache persistente
    pub persistent: bool,
    
    /// Diretório para cache persistente
    pub cache_dir: Option<String>,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_size_mb: 100,
            default_ttl: Duration::from_secs(3600), // 1 hora
            session_ttl: Duration::from_secs(86400), // 24 horas
            persistent: true,
            cache_dir: None,
        }
    }
}

/// Configurações de rede
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// Timeout para conexões
    pub connect_timeout: Duration,
    
    /// Timeout para requisições
    pub request_timeout: Duration,
    
    /// Número máximo de tentativas
    pub max_retries: u32,
    
    /// Delay entre tentativas
    pub retry_delay: Duration,
    
    /// User agent personalizado
    pub user_agent: Option<String>,
    
    /// Headers customizados
    pub custom_headers: HashMap<String, String>,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            connect_timeout: Duration::from_secs(30),
            request_timeout: Duration::from_secs(60),
            max_retries: 3,
            retry_delay: Duration::from_secs(1),
            user_agent: Some("SAGE-X-MCP-Client/1.0".to_string()),
            custom_headers: HashMap::new(),
        }
    }
}

/// Configurações do sistema de regras
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RulesConfig {
    /// Habilitar aplicação automática de regras
    pub auto_apply: bool,
    
    /// Validar regras antes da aplicação
    pub validate_before_apply: bool,
    
    /// Modo de execução (strict, permissive)
    pub execution_mode: ExecutionMode,
    
    /// Filtros de regras ativas
    pub active_filters: Vec<String>,
    
    /// Configurações de prioridade
    pub priority_config: PriorityConfig,
}

impl Default for RulesConfig {
    fn default() -> Self {
        Self {
            auto_apply: true,
            validate_before_apply: true,
            execution_mode: ExecutionMode::Strict,
            active_filters: vec!["code_style".to_string(), "session_management".to_string()],
            priority_config: PriorityConfig::default(),
        }
    }
}

/// Modo de execução de regras
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExecutionMode {
    /// Modo estrito - falha em qualquer erro
    Strict,
    /// Modo permissivo - continua com warnings
    Permissive,
    /// Modo de dry-run - apenas simula
    DryRun,
}

/// Configurações de prioridade de regras
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriorityConfig {
    /// Prioridades por categoria
    pub category_priorities: HashMap<String, u32>,
    
    /// Prioridades por ID de regra
    pub rule_priorities: HashMap<String, u32>,
    
    /// Prioridade padrão
    pub default_priority: u32,
}

impl Default for PriorityConfig {
    fn default() -> Self {
        Self {
            category_priorities: HashMap::new(),
            rule_priorities: HashMap::new(),
            default_priority: 100,
        }
    }
}

/// Flags de features disponíveis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureFlags {
    /// Bridge Python habilitado
    pub python_bridge: bool,
    
    /// Suporte WASM habilitado
    pub wasm_support: bool,
    
    /// Telemetria habilitada
    pub telemetry_enabled: bool,
    
    /// Cache distribuído
    pub distributed_cache: bool,
    
    /// Modo experimental
    pub experimental_features: bool,
}

impl Default for FeatureFlags {
    fn default() -> Self {
        Self {
            python_bridge: false,
            wasm_support: false,
            telemetry_enabled: true,
            distributed_cache: false,
            experimental_features: false,
        }
    }
}

/// Configurações específicas do MCP
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpConfig {
    /// Versão do protocolo MCP
    pub protocol_version: String,
    
    /// Nome do servidor MCP
    pub server_name: String,
    
    /// Descrição do servidor
    pub server_description: String,
    
    /// Capacidades do servidor
    pub capabilities: McpCapabilities,
    
    /// Configurações de transporte
    pub transport: TransportConfig,
}

impl Default for McpConfig {
    fn default() -> Self {
        Self {
            protocol_version: "1.0".to_string(),
            server_name: "SAGE-X MCP Server".to_string(),
            server_description: "SAGE-X Rules Integration Server for MCP".to_string(),
            capabilities: McpCapabilities::default(),
            transport: TransportConfig::default(),
        }
    }
}

/// Capacidades do servidor MCP
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpCapabilities {
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
}

impl Default for McpCapabilities {
    fn default() -> Self {
        Self {
            tools: true,
            resources: true,
            prompts: true,
            notifications: true,
            streaming: false,
        }
    }
}

/// Configurações de transporte MCP
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransportConfig {
    /// Tipo de transporte (stdio, http, websocket)
    pub transport_type: TransportType,
    
    /// Configurações específicas do transporte
    pub config: HashMap<String, serde_json::Value>,
}

impl Default for TransportConfig {
    fn default() -> Self {
        Self {
            transport_type: TransportType::Stdio,
            config: HashMap::new(),
        }
    }
}

/// Tipo de transporte MCP
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransportType {
    /// Standard I/O
    Stdio,
    /// HTTP
    Http,
    /// WebSocket
    WebSocket,
}

/// Configurações de telemetria
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryConfig {
    /// Habilitar coleta de métricas
    pub metrics_enabled: bool,
    
    /// Habilitar tracing
    pub tracing_enabled: bool,
    
    /// Endpoint para envio de telemetria
    pub endpoint: Option<String>,
    
    /// Intervalo de coleta em segundos
    pub collection_interval: Duration,
    
    /// Retenção de dados locais
    pub retention_days: u32,
}

impl Default for TelemetryConfig {
    fn default() -> Self {
        Self {
            metrics_enabled: true,
            tracing_enabled: false,
            endpoint: None,
            collection_interval: Duration::from_secs(60),
            retention_days: 7,
        }
    }
}

/// Representação de uma regra SAGE-X
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SageXRule {
    /// ID único da regra
    pub id: SageXId,
    
    /// Nome da regra
    pub name: String,
    
    /// Descrição da regra
    pub description: String,
    
    /// Categoria da regra
    pub category: String,
    
    /// Prioridade (0-1000)
    pub priority: u32,
    
    /// Condições para aplicação
    pub conditions: RuleConditions,
    
    /// Ações a serem executadas
    pub actions: Vec<RuleAction>,
    
    /// Metadados da regra
    pub metadata: RuleMetadata,
    
    /// Estado atual da regra
    pub state: RuleState,
    
    /// Configurações específicas
    pub config: HashMap<String, serde_json::Value>,
}

/// Condições para aplicação de regra
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleConditions {
    /// Contextos onde a regra se aplica
    pub contexts: Vec<String>,
    
    /// Padrões de arquivo
    pub file_patterns: Vec<String>,
    
    /// Condições de projeto
    pub project_conditions: Vec<ProjectCondition>,
    
    /// Condições temporais
    pub temporal_conditions: Option<TemporalCondition>,
    
    /// Condições customizadas
    pub custom_conditions: HashMap<String, serde_json::Value>,
}

/// Condição específica de projeto
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectCondition {
    /// Tipo da condição
    pub condition_type: String,
    
    /// Operador (equals, contains, matches, etc.)
    pub operator: String,
    
    /// Valor para comparação
    pub value: serde_json::Value,
    
    /// Inversão da condição
    pub negate: bool,
}

/// Condição temporal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalCondition {
    /// Horários de aplicação
    pub time_ranges: Vec<TimeRange>,
    
    /// Dias da semana
    pub weekdays: Vec<u8>,
    
    /// Fuso horário
    pub timezone: Option<String>,
}

/// Intervalo de tempo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRange {
    /// Hora de início (HH:MM)
    pub start: String,
    
    /// Hora de fim (HH:MM)
    pub end: String,
}

/// Ação executada por uma regra
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleAction {
    /// Tipo da ação
    pub action_type: ActionType,
    
    /// Parâmetros da ação
    pub parameters: HashMap<String, serde_json::Value>,
    
    /// Ordem de execução
    pub execution_order: u32,
    
    /// Condições específicas da ação
    pub conditions: Option<HashMap<String, serde_json::Value>>,
}

/// Tipos de ação disponíveis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionType {
    /// Executar comando
    ExecuteCommand,
    
    /// Modificar arquivo
    ModifyFile,
    
    /// Criar arquivo
    CreateFile,
    
    /// Aplicar formatação
    ApplyFormat,
    
    /// Executar lint
    RunLint,
    
    /// Notificar usuário
    Notify,
    
    /// Registrar log
    Log,
    
    /// Executar hook
    ExecuteHook,
    
    /// Aplicar template
    ApplyTemplate,
    
    /// Ação customizada
    Custom(String),
}

/// Metadados da regra
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleMetadata {
    /// Autor da regra
    pub author: String,
    
    /// Versão da regra
    pub version: String,
    
    /// Data de criação
    pub created_at: UnixTimestamp,
    
    /// Data de última modificação
    pub updated_at: UnixTimestamp,
    
    /// Tags para categorização
    pub tags: Vec<String>,
    
    /// Dependências de outras regras
    pub dependencies: Vec<SageXId>,
    
    /// Conflitos com outras regras
    pub conflicts: Vec<SageXId>,
    
    /// Documentação adicional
    pub documentation: Option<String>,
}

/// Estado atual da regra
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleState {
    /// Se a regra está ativa
    pub enabled: bool,
    
    /// Última execução
    pub last_execution: Option<UnixTimestamp>,
    
    /// Resultado da última execução
    pub last_result: Option<ExecutionResult>,
    
    /// Contadores de execução
    pub execution_stats: ExecutionStats,
    
    /// Erros recentes
    pub recent_errors: Vec<String>,
}

/// Resultado de execução de regra
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    /// Se foi bem-sucedida
    pub success: bool,
    
    /// Mensagem de resultado
    pub message: String,
    
    /// Duração da execução
    pub duration_ms: u64,
    
    /// Dados adicionais
    pub data: HashMap<String, serde_json::Value>,
}

/// Estatísticas de execução
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionStats {
    /// Total de execuções
    pub total_executions: u64,
    
    /// Execuções bem-sucedidas
    pub successful_executions: u64,
    
    /// Execuções falhadas
    pub failed_executions: u64,
    
    /// Duração média
    pub average_duration_ms: f64,
    
    /// Última atualização das estatísticas
    pub last_updated: UnixTimestamp,
}

impl Default for ExecutionStats {
    fn default() -> Self {
        Self {
            total_executions: 0,
            successful_executions: 0,
            failed_executions: 0,
            average_duration_ms: 0.0,
            last_updated: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}

/// Sessão de desenvolvimento
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevSession {
    /// ID da sessão
    pub id: SageXId,
    
    /// Timestamp de início
    pub started_at: UnixTimestamp,
    
    /// Timestamp de fim (se finalizada)
    pub ended_at: Option<UnixTimestamp>,
    
    /// Contexto da sessão
    pub context: SessionContext,
    
    /// Regras aplicadas na sessão
    pub applied_rules: Vec<SageXId>,
    
    /// Métricas da sessão
    pub metrics: SessionMetrics,
    
    /// Estado atual
    pub state: SessionState,
}

/// Contexto da sessão de desenvolvimento
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionContext {
    /// Diretório de trabalho
    pub working_directory: String,
    
    /// Projeto atual
    pub project_name: Option<String>,
    
    /// Branch Git ativa
    pub git_branch: Option<String>,
    
    /// Tecnologias detectadas
    pub technologies: Vec<String>,
    
    /// Variáveis de ambiente relevantes
    pub environment: HashMap<String, String>,
    
    /// Configurações de editor
    pub editor_config: HashMap<String, serde_json::Value>,
}

/// Métricas da sessão
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionMetrics {
    /// Número de regras aplicadas
    pub rules_applied: u32,
    
    /// Número de arquivos modificados
    pub files_modified: u32,
    
    /// Número de comandos executados
    pub commands_executed: u32,
    
    /// Tempo total ativo
    pub active_time_ms: u64,
    
    /// Erros encontrados
    pub errors_count: u32,
    
    /// Warnings gerados
    pub warnings_count: u32,
}

/// Estado da sessão
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionState {
    /// Sessão ativa
    Active,
    
    /// Sessão pausada
    Paused,
    
    /// Sessão finalizada normalmente
    Completed,
    
    /// Sessão finalizada com erro
    Failed,
    
    /// Sessão interrompida
    Interrupted,
}

/// Request para a API MCP
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpRequest {
    /// ID único da requisição
    pub id: String,
    
    /// Método solicitado
    pub method: String,
    
    /// Parâmetros da requisição
    pub params: Option<serde_json::Value>,
    
    /// Timestamp da requisição
    pub timestamp: UnixTimestamp,
}

/// Response da API MCP
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpResponse {
    /// ID da requisição correspondente
    pub id: String,
    
    /// Resultado (se sucesso)
    pub result: Option<serde_json::Value>,
    
    /// Erro (se falha)
    pub error: Option<McpError>,
    
    /// Timestamp da resposta
    pub timestamp: UnixTimestamp,
}

/// Erro MCP padronizado
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpError {
    /// Código do erro
    pub code: i32,
    
    /// Mensagem do erro
    pub message: String,
    
    /// Dados adicionais
    pub data: Option<serde_json::Value>,
}

/// Tool MCP disponível
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpTool {
    /// Nome da ferramenta
    pub name: String,
    
    /// Descrição da ferramenta
    pub description: String,
    
    /// Schema de input
    pub input_schema: serde_json::Value,
    
    /// Metadados adicionais
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

/// Resource MCP disponível
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpResource {
    /// URI do resource
    pub uri: String,
    
    /// Nome do resource
    pub name: String,
    
    /// Descrição do resource
    pub description: Option<String>,
    
    /// Tipo MIME
    pub mime_type: Option<String>,
    
    /// Metadados adicionais
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

impl SageXRule {
    /// Verifica se a regra pode ser aplicada no contexto atual
    pub fn can_apply(&self, context: &SessionContext) -> bool {
        // Implementação básica - pode ser expandida
        if !self.state.enabled {
            return false;
        }

        // Verificar contextos
        if !self.conditions.contexts.is_empty() {
            let has_matching_context = self.conditions.contexts.iter()
                .any(|ctx| context.technologies.contains(ctx));
            if !has_matching_context {
                return false;
            }
        }

        // Verificar padrões de arquivo
        if !self.conditions.file_patterns.is_empty() {
            // Implementação específica baseada no working_directory
            // Por enquanto, sempre verdadeiro se houver padrões
        }

        true
    }

    /// Aplica a regra no contexto fornecido
    pub async fn apply(&mut self, context: &SessionContext) -> SageXResult<ExecutionResult> {
        let start_time = SystemTime::now();
        
        // Simular aplicação da regra
        // A implementação real dependeria do tipo de ação
        
        let duration = start_time.elapsed().unwrap_or_default();
        
        let result = ExecutionResult {
            success: true,
            message: format!("Regra '{}' aplicada com sucesso", self.name),
            duration_ms: duration.as_millis() as u64,
            data: HashMap::new(),
        };

        // Atualizar estatísticas
        self.state.last_execution = Some(
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );
        self.state.last_result = Some(result.clone());
        self.state.execution_stats.total_executions += 1;
        if result.success {
            self.state.execution_stats.successful_executions += 1;
        } else {
            self.state.execution_stats.failed_executions += 1;
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sage_x_config_default() {
        let config = SageXConfig::default();
        assert_eq!(config.api_base_url, "https://api.sage-x.dev");
        assert!(config.auth_token.is_empty());
        assert!(config.features.telemetry_enabled);
    }

    #[test]
    fn test_rule_can_apply() {
        let mut rule = SageXRule {
            id: Uuid::new_v4(),
            name: "Test Rule".to_string(),
            description: "Test".to_string(),
            category: "test".to_string(),
            priority: 100,
            conditions: RuleConditions {
                contexts: vec!["rust".to_string()],
                file_patterns: vec![],
                project_conditions: vec![],
                temporal_conditions: None,
                custom_conditions: HashMap::new(),
            },
            actions: vec![],
            metadata: RuleMetadata {
                author: "test".to_string(),
                version: "1.0".to_string(),
                created_at: 0,
                updated_at: 0,
                tags: vec![],
                dependencies: vec![],
                conflicts: vec![],
                documentation: None,
            },
            state: RuleState {
                enabled: true,
                last_execution: None,
                last_result: None,
                execution_stats: ExecutionStats::default(),
                recent_errors: vec![],
            },
            config: HashMap::new(),
        };

        let context = SessionContext {
            working_directory: "/test".to_string(),
            project_name: Some("test".to_string()),
            git_branch: Some("main".to_string()),
            technologies: vec!["rust".to_string()],
            environment: HashMap::new(),
            editor_config: HashMap::new(),
        };

        assert!(rule.can_apply(&context));

        // Desabilitar regra
        rule.state.enabled = false;
        assert!(!rule.can_apply(&context));
    }

    #[test]
    fn test_execution_stats_default() {
        let stats = ExecutionStats::default();
        assert_eq!(stats.total_executions, 0);
        assert_eq!(stats.successful_executions, 0);
        assert_eq!(stats.failed_executions, 0);
        assert_eq!(stats.average_duration_ms, 0.0);
    }
}

