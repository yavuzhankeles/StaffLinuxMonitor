# Contributing to StaffLinuxMonitor

## Overview

Thank you for your interest in contributing to StaffLinuxMonitor! This document provides guidelines and information for contributors.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Coding Standards](#coding-standards)
- [Testing](#testing)
- [Pull Request Process](#pull-request-process)
- [Issue Reporting](#issue-reporting)
- [Documentation](#documentation)
- [Release Process](#release-process)

## Code of Conduct

### Our Pledge

We as members, contributors, and leaders pledge to make participation in our community a harassment-free experience for everyone, regardless of age, body size, visible or invisible disability, ethnicity, sex characteristics, gender identity and expression, level of experience, education, socio-economic status, nationality, personal appearance, race, religion, or sexual identity and orientation.

### Our Standards

Examples of behavior that contributes to a positive environment for our community include:

- Using welcoming and inclusive language
- Being respectful of differing opinions and viewpoints
- Gracefully accepting constructive criticism
- Focusing on what is best for the community
- Showing empathy towards other community members

Examples of unacceptable behavior include:

- The use of sexualized language or imagery, and sexual attention or advances
- Trolling, insulting or derogatory comments, and personal or political attacks
- Public or private harassment
- Publishing others' private information without explicit permission
- Other conduct which could reasonably be considered inappropriate in a professional setting

## Getting Started

### Prerequisites

- **Rust**: 1.70 or higher
- **Git**: Latest version
- **Linux Development Tools**: build-essential, pkg-config, libssl-dev
- **Docker**: For containerized development (optional)

### Fork and Clone

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/StaffLinuxMonitor.git
   cd StaffLinuxMonitor
   ```
3. **Add the upstream remote**:
   ```bash
   git remote add upstream https://github.com/forniya/StaffLinuxMonitor.git
   ```

### Development Setup

1. **Install Rust** (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```

2. **Install dependencies**:
   ```bash
   # Ubuntu/Debian
   sudo apt-get update
   sudo apt-get install -y build-essential pkg-config libssl-dev libcurl4-openssl-dev

   # CentOS/RHEL
   sudo yum groupinstall "Development Tools"
   sudo yum install pkgconfig openssl-devel libcurl-devel

   # Arch Linux
   sudo pacman -S base-devel pkg-config openssl curl
   ```

3. **Build the project**:
   ```bash
   cargo build
   ```

4. **Run tests**:
   ```bash
   cargo test
   ```

## Development Workflow

### Branch Strategy

We use a feature branch workflow:

- `main`: Production-ready code
- `develop`: Integration branch for features
- `feature/*`: Feature branches
- `bugfix/*`: Bug fix branches
- `hotfix/*`: Critical bug fixes

### Creating a Feature Branch

```bash
# Update your local main branch
git checkout main
git pull upstream main

# Create and switch to a new feature branch
git checkout -b feature/your-feature-name

# Make your changes
# ... edit files ...

# Commit your changes
git add .
git commit -m "feat: add your feature description"

# Push to your fork
git push origin feature/your-feature-name
```

### Commit Message Format

We follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

**Types:**
- `feat`: A new feature
- `fix`: A bug fix
- `docs`: Documentation only changes
- `style`: Changes that do not affect the meaning of the code
- `refactor`: A code change that neither fixes a bug nor adds a feature
- `perf`: A code change that improves performance
- `test`: Adding missing tests or correcting existing tests
- `chore`: Changes to the build process or auxiliary tools

**Examples:**
```bash
git commit -m "feat: add GPU monitoring support"
git commit -m "fix: resolve memory leak in API client"
git commit -m "docs: update API documentation"
git commit -m "test: add unit tests for configuration module"
```

## Coding Standards

### Rust Code Style

We follow the [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/style/naming/README.html) and use `rustfmt` for formatting.

1. **Install rustfmt**:
   ```bash
   rustup component add rustfmt
   ```

2. **Format your code**:
   ```bash
   cargo fmt
   ```

3. **Check for style issues**:
   ```bash
   cargo clippy
   ```

### Code Organization

```
src/
├── main.rs              # Application entry point
├── api.rs               # API client implementation
├── config.rs            # Configuration management
├── log_config.rs        # Logging configuration
├── monitoring/          # Monitoring modules
│   ├── mod.rs
│   ├── cpu.rs
│   ├── memory.rs
│   ├── disk.rs
│   └── network.rs
├── security/            # Security modules
│   ├── mod.rs
│   ├── firewall.rs
│   └── services.rs
└── utils/               # Utility functions
    ├── mod.rs
    └── helpers.rs
```

### Naming Conventions

- **Functions**: `snake_case`
- **Variables**: `snake_case`
- **Constants**: `SCREAMING_SNAKE_CASE`
- **Types**: `PascalCase`
- **Modules**: `snake_case`
- **Files**: `snake_case.rs`

### Error Handling

Use `anyhow` for error handling and `thiserror` for custom error types:

```rust
use anyhow::{Context, Result};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MonitoringError {
    #[error("Failed to read CPU information: {0}")]
    CpuReadError(String),
    #[error("Invalid configuration: {0}")]
    ConfigError(String),
}

pub fn get_cpu_info() -> Result<CpuInfo> {
    // Implementation
    Ok(cpu_info)
}
```

### Documentation

All public APIs must be documented:

```rust
/// Retrieves CPU usage information from the system.
///
/// # Returns
///
/// Returns a `Result` containing `CpuInfo` on success, or an error on failure.
///
/// # Examples
///
/// ```rust
/// use staffmon::monitoring::get_cpu_info;
///
/// match get_cpu_info() {
///     Ok(cpu_info) => println!("CPU Usage: {}%", cpu_info.usage_percent),
///     Err(e) => eprintln!("Error: {}", e),
/// }
/// ```
pub fn get_cpu_info() -> Result<CpuInfo> {
    // Implementation
}
```

## Testing

### Unit Tests

Write unit tests for all new functionality:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu_info_creation() {
        let cpu_info = CpuInfo {
            usage_percent: 50.0,
            temperature_celsius: Some(65.0),
            frequency_mhz: 2400.0,
        };

        assert_eq!(cpu_info.usage_percent, 50.0);
        assert_eq!(cpu_info.temperature_celsius, Some(65.0));
        assert_eq!(cpu_info.frequency_mhz, 2400.0);
    }

    #[test]
    fn test_invalid_cpu_usage() {
        // Test edge cases and error conditions
    }
}
```

### Integration Tests

Create integration tests in the `tests/` directory:

```rust
// tests/api_tests.rs
use staffmon::api::ApiClient;

#[tokio::test]
async fn test_api_health_check() {
    let client = ApiClient::new("http://localhost:8080", "test-key");
    let result = client.health_check().await;
    assert!(result.is_ok());
}
```

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_cpu_info_creation

# Run tests with coverage (requires cargo-tarpaulin)
cargo tarpaulin --out Html
```

### Test Coverage

We aim for at least 80% test coverage. Use `cargo-tarpaulin` to check coverage:

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Run coverage
cargo tarpaulin --out Html
```

## Pull Request Process

### Before Submitting

1. **Ensure tests pass**:
   ```bash
   cargo test
   cargo clippy
   cargo fmt --check
   ```

2. **Update documentation**:
   - Update README.md if needed
   - Add/update API documentation
   - Update configuration examples

3. **Check for breaking changes**:
   - If your change breaks existing functionality, document it clearly
   - Consider backward compatibility

### Creating a Pull Request

1. **Push your branch** to your fork
2. **Create a Pull Request** on GitHub
3. **Fill out the PR template**:
   - Describe the changes
   - Link related issues
   - Add screenshots if applicable
   - List any breaking changes

### PR Template

```markdown
## Description
Brief description of the changes

## Type of Change
- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] Documentation update

## Testing
- [ ] Unit tests pass
- [ ] Integration tests pass
- [ ] Manual testing completed

## Checklist
- [ ] My code follows the style guidelines of this project
- [ ] I have performed a self-review of my own code
- [ ] I have commented my code, particularly in hard-to-understand areas
- [ ] I have made corresponding changes to the documentation
- [ ] My changes generate no new warnings
- [ ] I have added tests that prove my fix is effective or that my feature works
- [ ] New and existing unit tests pass locally with my changes
- [ ] Any dependent changes have been merged and published in downstream modules

## Related Issues
Closes #(issue number)
```

### Review Process

1. **Automated checks** must pass
2. **Code review** by maintainers
3. **Address feedback** and make requested changes
4. **Maintainer approval** required for merge

## Issue Reporting

### Bug Reports

When reporting bugs, please include:

1. **Environment information**:
   - Operating system and version
   - Rust version
   - StaffLinuxMonitor version
   - Hardware specifications

2. **Steps to reproduce**:
   - Clear, step-by-step instructions
   - Expected vs actual behavior

3. **Logs and error messages**:
   - Relevant log files
   - Error messages
   - Stack traces

4. **Additional context**:
   - Configuration files (sanitized)
   - System resources at time of issue

### Feature Requests

When requesting features, please include:

1. **Problem description**: What problem does this solve?
2. **Proposed solution**: How should it work?
3. **Use cases**: Who would benefit from this?
4. **Alternatives considered**: What other approaches were considered?

### Issue Template

```markdown
## Bug Report / Feature Request

### Environment
- OS: [e.g., Ubuntu 20.04]
- Rust Version: [e.g., 1.70.0]
- StaffLinuxMonitor Version: [e.g., 1.0.2]

### Description
[Describe the issue or feature request]

### Steps to Reproduce
1. [First step]
2. [Second step]
3. [And so on...]

### Expected Behavior
[What you expected to happen]

### Actual Behavior
[What actually happened]

### Additional Information
[Any other context, logs, screenshots, etc.]
```

## Documentation

### Code Documentation

- Document all public APIs
- Include examples in documentation
- Keep documentation up to date with code changes

### User Documentation

- Update README.md for user-facing changes
- Add configuration examples
- Update troubleshooting guides

### API Documentation

- Document all API endpoints
- Include request/response examples
- Document error codes and messages

## Release Process

### Versioning

We follow [Semantic Versioning](https://semver.org/):

- **MAJOR**: Breaking changes
- **MINOR**: New features, backward compatible
- **PATCH**: Bug fixes, backward compatible

### Release Checklist

1. **Update version** in `Cargo.toml`
2. **Update CHANGELOG.md** with release notes
3. **Create release branch**:
   ```bash
   git checkout -b release/v1.0.3
   ```
4. **Update documentation** if needed
5. **Run full test suite**:
   ```bash
   cargo test
   cargo clippy
   cargo fmt --check
   ```
6. **Create release** on GitHub
7. **Tag the release**:
   ```bash
   git tag -a v1.0.3 -m "Release v1.0.3"
   git push origin v1.0.3
   ```

### Release Notes Template

```markdown
## [1.0.3] - 2024-03-19

### Added
- New feature A
- New feature B

### Changed
- Improved performance of X
- Updated dependency Y

### Fixed
- Bug fix A
- Bug fix B

### Breaking Changes
- None

### Migration Guide
- Any migration steps if needed
```

## Community Guidelines

### Communication

- Be respectful and professional
- Use clear, concise language
- Provide constructive feedback
- Help other contributors

### Recognition

Contributors will be recognized in:
- README.md contributors section
- Release notes
- GitHub contributors page

### Getting Help

- **GitHub Issues**: For bug reports and feature requests
- **GitHub Discussions**: For questions and general discussion
- **Wiki**: For detailed documentation

## License

By contributing to StaffLinuxMonitor, you agree that your contributions will be licensed under the MIT License.

## Contact

- **Maintainer**: [Your Name](mailto:your.email@example.com)
- **GitHub**: [@yourusername](https://github.com/yourusername)
- **Discussions**: [GitHub Discussions](https://github.com/forniya/StaffLinuxMonitor/discussions)

---

Thank you for contributing to StaffLinuxMonitor! 🚀 