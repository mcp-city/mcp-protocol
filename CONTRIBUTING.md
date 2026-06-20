# Contributing to mcp:// Protocol

Thank you for your interest in contributing to the mcp:// protocol!

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/your-username/mcp-protocol.git`
3. Create a branch: `git checkout -b feature/your-feature-name`

## Development Setup

### Python Client
```bash
cd python
pip install -e .
pip install pytest pytest-asyncio
```

### JavaScript Client
```bash
cd javascript
npm install
```

### Rust CLI
```bash
cd cli
cargo build
cargo test
```

## Running Tests

```bash
# Python tests
cd python && pytest tests/

# JavaScript tests
cd javascript && npm test

# Rust tests
cd cli && cargo test

# Integration tests
cargo test --test integration_test
```

## Code Style

- Python: Follow PEP 8
- JavaScript: Use ESLint
- Rust: Use `cargo fmt` and `cargo clippy`

## Submitting Changes

1. Commit your changes: `git commit -m "Description of your changes"`
2. Push to your fork: `git push origin feature/your-feature-name`
3. Create a Pull Request on GitHub

## Pull Request Guidelines

- Write a clear description of your changes
- Reference related issues
- Add tests for new features
- Update documentation as needed
- Ensure all tests pass

## Issue Labels

- `bug`: Bug reports
- `enhancement`: Feature requests
- `documentation`: Documentation improvements
- `python`: Python client issues
- `javascript`: JavaScript client issues
- `rust`: Rust CLI issues
- `testing`: Test-related issues
- `security`: Security vulnerabilities

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

## Powered by
[HYBRID IN.](https://hybridin.io/) x [GRN.cloud](https://grn.cloud/)
