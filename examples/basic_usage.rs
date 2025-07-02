//! Exemplo básico de uso do SAGE-X MCP Client
//!
//! Este exemplo demonstra como:
//! - Criar e configurar um cliente SAGE-X
//! - Iniciar uma sessão de desenvolvimento
//! - Carregar e aplicar regras
//! - Executar ferramentas MCP
//! - Coletar métricas

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use sage_x_mcp_client::{
    client::{SageXClient, SageXEvent},
    models::{
        SageXConfig, SessionContext, McpTool, McpResource,
        CacheConfig, NetworkConfig, RulesConfig, FeatureFlags,
        McpConfig, TelemetryConfig
    },
    error::SageXResult,
};

use serde_json::json;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> SageXResult<()> {
    println!("🦀 SAGE-X MCP Client - Exemplo Básico");
    println!("=====================================\n");

    // 1. Criar configuração personalizada
    let config = create_custom_config().await;
    println!("✅ Configuração criada");

    // 2. Criar cliente com configuração
    let client = Arc::new(SageXClient::with_config(config).await?);
    println!("✅ Cliente MCP criado");

    // 3. Iniciar processamento de eventos em background
    client.clone().start_event_processing().await;
    println!("✅ Processamento de eventos iniciado");

    // 4. Registrar ferramentas MCP
    register_sample_tools(&client).await?;
    println!("✅ Ferramentas MCP registradas");

    // 5. Registrar resources MCP
    register_sample_resources(&client).await?;
    println!("✅ Resources MCP registrados");

    // 6. Verificar saúde da conexão (vai falhar pois não há servidor real)
    match client.health_check().await {
        Ok(healthy) => println!("✅ Health check: {}", if healthy { "Saudável" } else { "Com problemas" }),
        Err(e) => println!("⚠️  Health check falhou (esperado): {}", e),
    }

    // 7. Criar contexto de sessão
    let session_context = create_session_context();
    println!("✅ Contexto de sessão criado");

    // 8. Iniciar sessão de desenvolvimento
    let session_id = client.start_session(session_context).await?;
    println!("✅ Sessão iniciada: {}", session_id);

    // 9. Listar ferramentas disponíveis
    let tools = client.list_tools().await;
    println!("\n📋 Ferramentas disponíveis:");
    for tool in &tools {
        println!("  - {}: {}", tool.name, tool.description);
    }

    // 10. Listar resources disponíveis
    let resources = client.list_resources().await;
    println!("\n📋 Resources disponíveis:");
    for resource in &resources {
        println!("  - {}: {}", resource.name, resource.description.as_deref().unwrap_or("N/A"));
    }

    // 11. Executar uma ferramenta MCP
    println!("\n🔧 Executando ferramenta 'code_formatter'...");
    let tool_result = client.execute_tool("code_formatter", json!({
        "language": "rust",
        "file_path": "src/main.rs",
        "format_options": {
            "line_width": 100,
            "tab_spaces": 4
        }
    })).await?;
    
    if let Some(result) = tool_result.result {
        println!("✅ Resultado da ferramenta: {}", result);
    }

    // 12. Simular carregamento de regras (vai falhar pois não há servidor)
    println!("\n📜 Tentando carregar regras...");
    match client.load_rules().await {
        Ok(rules) => {
            println!("✅ {} regras carregadas", rules.len());
            
            // Aplicar regras aplicáveis
            if !rules.is_empty() {
                println!("🔄 Aplicando regras aplicáveis...");
                let results = client.apply_applicable_rules().await?;
                println!("✅ {} regras aplicadas", results.len());
            }
        }
        Err(e) => println!("⚠️  Falha ao carregar regras (esperado): {}", e),
    }

    // 13. Coletar métricas
    println!("\n📊 Coletando métricas...");
    let metrics = client.collect_metrics().await?;
    println!("✅ Métricas coletadas:");
    for (key, value) in &metrics {
        println!("  - {}: {}", key, value);
    }

    // 14. Aguardar um pouco para ver eventos
    println!("\n⏳ Aguardando eventos...");
    sleep(Duration::from_secs(2)).await;

    // 15. Finalizar sessão
    if let Some(ended_session_id) = client.end_session().await? {
        println!("✅ Sessão finalizada: {}", ended_session_id);
    }

    // 16. Coletar métricas finais
    println!("\n📊 Métricas finais:");
    let final_metrics = client.collect_metrics().await?;
    for (key, value) in &final_metrics {
        println!("  - {}: {}", key, value);
    }

    println!("\n🎉 Exemplo concluído com sucesso!");
    Ok(())
}

/// Cria uma configuração personalizada para o cliente
async fn create_custom_config() -> SageXConfig {
    SageXConfig {
        api_base_url: "https://api.sage-x.local".to_string(),
        auth_token: "demo_token_12345".to_string(),
        
        cache: CacheConfig {
            max_size_mb: 50,
            default_ttl: Duration::from_secs(1800), // 30 minutos
            session_ttl: Duration::from_secs(43200), // 12 horas
            persistent: true,
            cache_dir: Some("./cache".to_string()),
        },
        
        network: NetworkConfig {
            connect_timeout: Duration::from_secs(10),
            request_timeout: Duration::from_secs(30),
            max_retries: 2,
            retry_delay: Duration::from_secs(2),
            user_agent: Some("SAGE-X-Demo/1.0".to_string()),
            custom_headers: {
                let mut headers = HashMap::new();
                headers.insert("X-Demo-Mode".to_string(), "true".to_string());
                headers
            },
        },
        
        rules: RulesConfig {
            auto_apply: true,
            validate_before_apply: true,
            execution_mode: sage_x_mcp_client::models::ExecutionMode::Permissive,
            active_filters: vec![
                "code_style".to_string(),
                "session_management".to_string(),
                "rust_specific".to_string(),
            ],
            priority_config: sage_x_mcp_client::models::PriorityConfig {
                category_priorities: {
                    let mut priorities = HashMap::new();
                    priorities.insert("code_style".to_string(), 100);
                    priorities.insert("security".to_string(), 200);
                    priorities.insert("performance".to_string(), 150);
                    priorities
                },
                rule_priorities: HashMap::new(),
                default_priority: 100,
            },
        },
        
        features: FeatureFlags {
            python_bridge: false,
            wasm_support: false,
            telemetry_enabled: true,
            distributed_cache: false,
            experimental_features: true,
        },
        
        mcp: McpConfig {
            protocol_version: "1.0".to_string(),
            server_name: "SAGE-X Demo Server".to_string(),
            server_description: "Servidor de demonstração SAGE-X MCP".to_string(),
            capabilities: sage_x_mcp_client::models::McpCapabilities {
                tools: true,
                resources: true,
                prompts: true,
                notifications: true,
                streaming: false,
            },
            transport: sage_x_mcp_client::models::TransportConfig {
                transport_type: sage_x_mcp_client::models::TransportType::Http,
                config: {
                    let mut config = HashMap::new();
                    config.insert("port".to_string(), json!(8080));
                    config.insert("host".to_string(), json!("localhost"));
                    config
                },
            },
        },
        
        telemetry: TelemetryConfig {
            metrics_enabled: true,
            tracing_enabled: false,
            endpoint: Some("https://telemetry.sage-x.local/metrics".to_string()),
            collection_interval: Duration::from_secs(30),
            retention_days: 3,
        },
    }
}

/// Cria contexto de sessão de exemplo
fn create_session_context() -> SessionContext {
    SessionContext {
        working_directory: std::env::current_dir()
            .unwrap()
            .to_string_lossy()
            .to_string(),
        project_name: Some("sage-x-mcp-rust-client".to_string()),
        git_branch: Some("main".to_string()),
        technologies: vec![
            "rust".to_string(),
            "tokio".to_string(),
            "serde".to_string(),
            "mcp".to_string(),
        ],
        environment: {
            let mut env = HashMap::new();
            env.insert("RUST_ENV".to_string(), "development".to_string());
            env.insert("MCP_MODE".to_string(), "demo".to_string());
            env
        },
        editor_config: {
            let mut config = HashMap::new();
            config.insert("editor".to_string(), json!("vscode"));
            config.insert("format_on_save".to_string(), json!(true));
            config.insert("line_width".to_string(), json!(100));
            config
        },
    }
}

/// Registra ferramentas MCP de exemplo
async fn register_sample_tools(client: &SageXClient) -> SageXResult<()> {
    // Ferramenta de formatação de código
    let code_formatter = McpTool {
        name: "code_formatter".to_string(),
        description: "Formata código Rust usando rustfmt".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "language": {
                    "type": "string",
                    "enum": ["rust", "javascript", "python"]
                },
                "file_path": {
                    "type": "string",
                    "description": "Caminho do arquivo a ser formatado"
                },
                "format_options": {
                    "type": "object",
                    "properties": {
                        "line_width": {"type": "integer", "default": 100},
                        "tab_spaces": {"type": "integer", "default": 4}
                    }
                }
            },
            "required": ["language", "file_path"]
        }),
        metadata: Some({
            let mut meta = HashMap::new();
            meta.insert("category".to_string(), json!("code_quality"));
            meta.insert("version".to_string(), json!("1.0.0"));
            meta
        }),
    };

    // Ferramenta de linting
    let linter = McpTool {
        name: "linter".to_string(),
        description: "Executa análise estática de código".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "language": {
                    "type": "string",
                    "enum": ["rust", "javascript", "python"]
                },
                "target": {
                    "type": "string",
                    "description": "Arquivo ou diretório para analisar"
                },
                "severity": {
                    "type": "string",
                    "enum": ["error", "warning", "info"],
                    "default": "warning"
                }
            },
            "required": ["language", "target"]
        }),
        metadata: Some({
            let mut meta = HashMap::new();
            meta.insert("category".to_string(), json!("code_quality"));
            meta.insert("version".to_string(), json!("2.1.0"));
            meta
        }),
    };

    // Ferramenta de teste
    let test_runner = McpTool {
        name: "test_runner".to_string(),
        description: "Executa testes automatizados".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "test_type": {
                    "type": "string",
                    "enum": ["unit", "integration", "all"],
                    "default": "unit"
                },
                "pattern": {
                    "type": "string",
                    "description": "Padrão para filtrar testes"
                },
                "coverage": {
                    "type": "boolean",
                    "default": false,
                    "description": "Gerar relatório de cobertura"
                }
            }
        }),
        metadata: Some({
            let mut meta = HashMap::new();
            meta.insert("category".to_string(), json!("testing"));
            meta.insert("version".to_string(), json!("1.5.0"));
            meta
        }),
    };

    client.register_tool(code_formatter).await?;
    client.register_tool(linter).await?;
    client.register_tool(test_runner).await?;

    Ok(())
}

/// Registra resources MCP de exemplo
async fn register_sample_resources(client: &SageXClient) -> SageXResult<()> {
    // Resource de configuração do projeto
    let project_config = McpResource {
        uri: "sage-x://project/config".to_string(),
        name: "Project Configuration".to_string(),
        description: Some("Configuração atual do projeto SAGE-X".to_string()),
        mime_type: Some("application/json".to_string()),
        metadata: Some({
            let mut meta = HashMap::new();
            meta.insert("category".to_string(), json!("configuration"));
            meta.insert("read_only".to_string(), json!(false));
            meta
        }),
    };

    // Resource de regras ativas
    let active_rules = McpResource {
        uri: "sage-x://rules/active".to_string(),
        name: "Active Rules".to_string(),
        description: Some("Lista de regras ativas no contexto atual".to_string()),
        mime_type: Some("application/json".to_string()),
        metadata: Some({
            let mut meta = HashMap::new();
            meta.insert("category".to_string(), json!("rules"));
            meta.insert("read_only".to_string(), json!(true));
            meta
        }),
    };

    // Resource de métricas
    let metrics_data = McpResource {
        uri: "sage-x://metrics/current".to_string(),
        name: "Current Metrics".to_string(),
        description: Some("Métricas atuais da sessão de desenvolvimento".to_string()),
        mime_type: Some("application/json".to_string()),
        metadata: Some({
            let mut meta = HashMap::new();
            meta.insert("category".to_string(), json!("telemetry"));
            meta.insert("read_only".to_string(), json!(true));
            meta.insert("refresh_interval".to_string(), json!(30));
            meta
        }),
    };

    client.register_resource(project_config).await?;
    client.register_resource(active_rules).await?;
    client.register_resource(metrics_data).await?;

    Ok(())
}

