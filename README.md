# 🦀 SAGE-X MCP Rust Client

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![License](https://img.shields.io/badge/license-MIT-blue.svg?style=for-the-badge)
![Version](https://img.shields.io/badge/version-0.1.0-green.svg?style=for-the-badge)

**Cliente Rust moderno para integração com capacidades MCP (Model Context Protocol) avançadas, sistema de regras adaptativos e bridge simbiótico Python-Rust.**

## ✨ Características

### 🔥 Core Features
- **MCP Enhanced**: Integração completa com Model Context Protocol
- **Rules Engine**: Sistema de regras adaptativos e contextuais  
- **Bridge Simbiótico**: Conectividade Python-Rust de alta performance
- **Event Streaming**: Server-Sent Events para atualizações em tempo real
- **Cache Inteligente**: Sistema de cache com ETag e versionamento

### 🌐 Capacidades Modernas
- **Async/Await**: Runtime Tokio com suporte completo assíncrono
- **WASM Ready**: Compilação para WebAssembly para integração web
- **Security First**: JWT, SHA-256, autenticação OAuth2
- **Configuration**: Sistema de configuração flexível (TOML, ENV, CLI)
- **Error Handling**: Tratamento de erros com `thiserror` e `anyhow`

### 🎯 Integração WARP_RULES
- Busca automática de regras da API WARP
- Aplicação dinâmica de regras no contexto do agente
- Sincronização bidirecional de resultados
- Monitoramento contínuo via SSE

## 🚀 Quick Start

### Instalação

```bash
# Clone o repositório
git clone https://github.com/NEO-SH1W4/sage-x-mcp-rust-client.git
cd sage-x-mcp-rust-client

# Build básico
cargo build

# Build com todas as features
cargo build --features full

# Executar demo
cargo run --bin sage-x-mcp-demo
```

### Uso Básico

```rust
use sage_x_mcp_client::{SageXMcpClient, ClientConfig, Credentials};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configuração do cliente
    let config = ClientConfig::builder()
        .api_url("http://localhost:8001")
        .credentials(Credentials::new("sage_x_agent", "agent_secret"))
        .use_sse(true)
        .cache_enabled(true)
        .build()?;

    // Inicialização do cliente
    let mut client = SageXMcpClient::new(config);
    client.init().await?;

    // Criar contexto do agente
    let mut context = client.create_agent_context("sage_x_001", "SAGE-X Agent")?;

    // Buscar e aplicar regras
    let rules = client.fetch_rules().await?;
    let results = client.apply_rules(&mut context).await?;

    // Enviar resultados de volta
    client.send_results(&context).await?;

    // Iniciar monitoramento contínuo
    client.start_monitoring().await?;

    Ok(())
}
```

## 🏗️ Arquitetura

```
sage-x-mcp-rust-client/
├── src/
│   ├── lib.rs              # API pública principal
│   ├── client/             # Cliente MCP core
│   │   ├── mod.rs
│   │   ├── config.rs       # Configuração
│   │   ├── auth.rs         # Autenticação JWT
│   │   └── http.rs         # Cliente HTTP
│   ├── rules/              # Engine de regras
│   │   ├── mod.rs
│   │   ├── engine.rs       # Motor de regras
│   │   ├── parser.rs       # Parser de regras
│   │   └── context.rs      # Contexto do agente
│   ├── mcp/                # Protocolo MCP
│   │   ├── mod.rs
│   │   ├── protocol.rs     # Protocolo base
│   │   ├── messages.rs     # Mensagens MCP
│   │   └── bridge.rs       # Bridge Python-Rust
│   ├── sync/               # Sincronização
│   │   ├── mod.rs
│   │   ├── sse.rs          # Server-Sent Events
│   │   └── cache.rs        # Sistema de cache
│   ├── error.rs            # Definições de erro
│   ├── models.rs           # Modelos de dados
│   └── bin/
│       └── demo.rs         # Aplicação demo
├── examples/               # Exemplos de uso
├── benches/               # Benchmarks
├── tests/                 # Testes
└── docs/                  # Documentação
```

## 🔧 Features

### Default Features
```toml
default = ["mcp-enhanced", "rules-engine"]
```

### Todas as Features
```toml
full = ["mcp-enhanced", "rules-engine", "python-bridge", "wasm-support"]
```

### Features Individuais
- `mcp-enhanced`: Capacidades MCP avançadas
- `rules-engine`: Motor de regras adaptativos  
- `python-bridge`: Bridge Python-Rust via PyO3
- `wasm-support`: Compilação para WebAssembly
- `dev-tools`: Ferramentas de desenvolvimento

## 🌐 Capacidades MCP

### Protocolo MCP Nativo
- Mensagens JSON-RPC 2.0
- Transporte HTTP/WebSocket
- Autenticação e autorização
- Metadados e versionamento

### Integração WARP_RULES
- Endpoint: `/api/rules/v1/rules`
- Autenticação: JWT OAuth2
- Cache: ETag + timestamp
- Streaming: Server-Sent Events

### Bridge Simbiótico
```rust
// Chamada Python desde Rust
let result: PyResult<String> = client
    .python_bridge()
    .call_function("process_rules", (rules, context))?;

// Callback Rust desde Python
client.register_python_callback("on_rule_applied", |rule_id, result| {
    println!("Regra {} aplicada: {:?}", rule_id, result);
})?;
```

## 🔥 Performance

### Benchmarks
```bash
# Executar benchmarks
cargo bench

# Benchmark específico
cargo bench mcp_client_bench
```

### Otimizações
- **Zero-copy**: Serialização/deserialização otimizada
- **Connection pooling**: Reutilização de conexões HTTP
- **Async streams**: Processamento reativo de eventos
- **Memory management**: Controle fino de alocação

## 🧪 Desenvolvimento

### Testes
```bash
# Testes unitários
cargo test

# Testes de integração
cargo test --features full

# Testes com mock
cargo test --features mockito
```

### Linting
```bash
# Clippy
cargo clippy --all-features

# Formatação
cargo fmt

# Auditoria de segurança
cargo audit
```

## 📖 Exemplos

### Exemplo Básico
```bash
cargo run --example basic_client
```

### Exemplo com Python Bridge
```bash
cargo run --example python_bridge --features python-bridge
```

### Exemplo WASM
```bash
wasm-pack build --target web --features wasm-support
```

## 🤝 Contribuição

1. Fork o projeto
2. Criar feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit suas mudanças (`git commit -m 'Add: AmazingFeature'`)
4. Push para a branch (`git push origin feature/AmazingFeature`)
5. Abrir Pull Request

### Padrões de Commit
```
feat: nova funcionalidade
fix: correção de bug
docs: documentação
style: formatação
refactor: refatoração
test: testes
chore: manutenção
```

## 🔗 Ecossistema SAGE-X

Este cliente faz parte do ecossistema SAGE-X:

- **[VIREON](https://github.com/NEO-SH1W4/VIREON)** - Consciência artificial avançada
- **[GUARDRIVE](https://github.com/NEO-SH1W4/GUARDRIVE)** - Sistema de armazenamento simbiótico  
- **[ARQUIMAX](https://github.com/NEO-SH1W4/ARQUIMAX)** - Arquitetura de IA coletiva
- **[MCP_ECOSYSTEM](https://github.com/NEO-SH1W4/MCP_ECOSYSTEM)** - Ferramentas MCP

## 📄 Licença

Este projeto está licenciado sob a Licença MIT - veja o arquivo [LICENSE](LICENSE) para detalhes.

## 🙏 Agradecimentos

- Comunidade Rust por ferramentas excepcionais
- Protocolo MCP por padrões modernos  
- PyO3 por bridge Python-Rust seamless
- Tokio por runtime assíncrono de alta performance

---

**Desenvolvido com 🦀 e ❤️ por [NEO-SH1W4](https://github.com/NEO-SH1W4)**

