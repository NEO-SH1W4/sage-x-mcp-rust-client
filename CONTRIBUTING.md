# Contributing to MCP Rust Client

Thank you for considering contributing to this MCP Rust Client! This document outlines the process for contributing to this project.

## Code of Conduct

By participating in this project, you are expected to uphold our Code of Conduct. Please report unacceptable behavior to [maintainer email].

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check the issue tracker to avoid duplicates. When you create a bug report, include as many details as possible:

- Use a clear and descriptive title
- Describe the exact steps to reproduce the problem
- Describe the behavior you observed and what you expected to see
- Include details about your environment (OS, Rust version, etc.)

### Suggesting Enhancements

Enhancement suggestions are also welcome. Please include:

- A clear and descriptive title
- A detailed description of the proposed functionality
- Explain why this enhancement would be useful to most users

### Pull Requests

1. Fork the repository
2. Create a new branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests (`cargo test`)
5. Commit your changes using [Conventional Commits](https://www.conventionalcommits.org/)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

## Styleguides

### Rust Style Guide

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` before committing
- Run `cargo clippy` and address all warnings
- Write documentation for public APIs
- Add tests for new functionality

### Git Commit Messages

- Use [Conventional Commits](https://www.conventionalcommits.org/)
- Reference issues and pull requests when relevant
- Consider starting the commit message with an applicable emoji:
  - ✨ `:sparkles:` for new features
  - 🐛 `:bug:` for bug fixes
  - 📝 `:memo:` for documentation
  - ♻️ `:recycle:` for refactoring
  - 🧪 `:test_tube:` for adding tests

## Development Workflow

1. **Setup**: Clone the repository and run `cargo build`
2. **Development**: Make your changes, following the style guides
3. **Testing**: Run `cargo test` to ensure all tests pass
4. **Documentation**: Update documentation as needed
5. **Pull Request**: Submit your changes for review

## Additional Notes

### Issue and Pull Request Labels

- `bug`: Bug reports
- `enhancement`: Feature requests
- `documentation`: Documentation improvements
- `good first issue`: Good for newcomers
- `help wanted`: Extra attention is needed

Thank you for contributing to this MCP Rust Client!

