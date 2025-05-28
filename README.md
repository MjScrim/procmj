# ⚙️ procmj

**Procmj** é uma ferramenta de linha de comando (CLI) escrita em Rust para monitoramento de processos do sistema de forma simples, rápida e extensível.

## 🔧 Funcionalidades

- Listagem de todos os processos ativos
- Filtro por nome de processo (`--filter`)
- Exportação da saída em formato JSON (`--output`)
- Arquitetura modular e assíncrona com `tokio`
- Compatível com Linux, macOS e Windows

## 🚀 Uso

### Executar diretamente com Cargo:

```bash
cargo run -- --filter firefox
cargo run -- --output processos.json

