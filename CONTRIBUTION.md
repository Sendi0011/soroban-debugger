# Contributing to Soroban Debugger

Thanks for your interest in contributing to the Soroban Debugger project!

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/yourusername/soroban-debugger.git`
3. Create a new branch: `git checkout -b feature/your-feature-name`
4. Make your changes
5. Run tests: `cargo test`
6. Run formatting: `cargo fmt`
7. Run linter: `cargo clippy`
8. Commit your changes: `git commit -am 'Add some feature'`
9. Push to the branch: `git push origin feature/your-feature-name`
10. Create a Pull Request

## Development Setup

### Prerequisites

- Rust 1.75 or later
- Soroban CLI (for testing)

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Running the CLI

```bash
cargo run -- run --contract path/to/contract.wasm --function function_name
```

## Project Structure

- `src/cli/` - Command-line interface
- `src/debugger/` - Core debugging engine
- `src/runtime/` - WASM execution environment
- `src/inspector/` - State inspection tools
- `src/ui/` - Terminal user interface
- `src/utils/` - Utility functions
- `tests/` - Integration tests
- `examples/` - Example usage

## Code Style

This project follows standard Rust conventions:

- Run `cargo fmt` before committing
- Run `cargo clippy` and fix any warnings
- Write tests for new functionality
- Update documentation as needed

## Commit Messages

- Use clear, descriptive commit messages
- Start with a verb in present tense (Add, Fix, Update, etc.)
- Keep the first line under 72 characters
- Add detailed description in the body if needed

## Pull Request Process

1. Ensure all tests pass
2. Update README.md if you've added features
3. Update CHANGELOG.md with your changes
4. Request review from maintainers

## Issue Guidelines

### Reporting Bugs

Include:
- Steps to reproduce
- Expected behavior
- Actual behavior
- Contract WASM file (if relevant)
- Error messages
- Environment details

### Suggesting Features

Include:
- Clear description of the feature
- Use cases
- Expected behavior
- Any relevant examples

## Areas for Contribution

### Phase 1 (Current)
- Basic CLI improvements
- Better error messages
- Storage inspection enhancements
- Budget tracking improvements

### Phase 2 (Upcoming)
- Breakpoint management
- Enhanced terminal UI
- Call stack visualization
- Execution replay

### Phase 3 (Future)
- WASM instrumentation
- Source map support
- Memory profiling
- Performance analysis

## Questions?

Feel free to open an issue or reach out to the maintainers.

## Code of Conduct

Be respectful and constructive in all interactions.