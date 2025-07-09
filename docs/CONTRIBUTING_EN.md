# Contributing Guide

## Welcome! 🎉

Thank you for your interest in contributing to the StaffLinuxMonitor project! This guide explains how you can contribute to the project.

## Table of Contents

- [Ways to Contribute](#ways-to-contribute)
- [Development Environment Setup](#development-environment-setup)
- [Code Writing Guidelines](#code-writing-guidelines)
- [Writing Tests](#writing-tests)
- [Pull Request Process](#pull-request-process)
- [Reporting and Communication](#reporting-and-communication)
- [License](#license)

## Ways to Contribute

### 🐛 Bug Reporting

When you find a bug, please create a [GitHub Issue](https://github.com/forniya/StaffLinuxMonitor/issues). Include the following in your bug report:

- **Operating System**: Ubuntu 20.04, CentOS 8, etc.
- **StaffLinuxMonitor Version**: `staffmon --version`
- **Bug Description**: What happened and what was expected?
- **Steps**: Steps to reproduce the bug
- **Log Files**: Relevant error messages
- **Screenshots**: If applicable

### 💡 Feature Requests

Use [GitHub Discussions](https://github.com/forniya/StaffLinuxMonitor/discussions) for new feature suggestions. Explain in your suggestion:

- **Feature Description**: What should it do?
- **Use Case**: When would it be useful?
- **Alternatives**: What existing solutions are there?
- **Priority**: How important is it?

### 📚 Documentation

Documentation improvements are always welcome:

- README updates
- API documentation
- Installation guides
- Example configurations
- Translation contributions

### 🔧 Code Contributions

For code contributions:

1. [Fork](https://github.com/forniya/StaffLinuxMonitor/fork) the repository
2. Create a feature branch
3. Make your changes
4. Write tests
5. Submit a Pull Request

## Development Environment Setup

### Prerequisites

```bash
# Rust (1.70+)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# System dependencies (Ubuntu/Debian)
sudo apt-get update
sudo apt-get install -y build-essential pkg-config libssl-dev libcurl4-openssl-dev

# System dependencies (CentOS/RHEL)
sudo yum groupinstall "Development Tools"
sudo yum install openssl-devel libcurl-devel
```

### Clone Project

```bash
# Clone repository
git clone https://github.com/forniya/StaffLinuxMonitor.git
cd StaffLinuxMonitor

# Install development dependencies
cargo build

# Run tests
cargo test
```

### IDE Setup

**VS Code (Recommended)**

```bash
# Install Rust extensions
code --install-extension rust-lang.rust-analyzer
code --install-extension vadimcn.vscode-lldb
code --install-extension serayuzgur.crates
```

**IntelliJ IDEA / CLion**

- Install Rust plugin
- Open Cargo.toml file
- Define project as Rust project

## Code Writing Guidelines

### Rust Code Style

Our project follows the [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/style/naming/README.html):

```rust
// ✅ Correct
pub struct SystemInfo {
    hostname: String,
    os_version: String,
    kernel_version: String,
}

impl SystemInfo {
    pub fn new() -> Self {
        Self {
            hostname: String::new(),
            os_version: String::new(),
            kernel_version: String::new(),
        }
    }
}

// ❌ Wrong
pub struct system_info {
    hostname: String,
    osVersion: String,
    kernel_version: String,
}
```

### File Organization

```
src/
├── main.rs              # Main entry point
├── lib.rs               # Library entry point
├── api/                 # API module
│   ├── mod.rs
│   ├── routes.rs
│   └── handlers.rs
├── monitoring/          # Monitoring module
│   ├── mod.rs
│   ├── cpu.rs
│   ├── memory.rs
│   └── disk.rs
├── config/              # Configuration module
│   ├── mod.rs
│   └── settings.rs
└── utils/               # Utility functions
    ├── mod.rs
    └── helpers.rs
```

### Function Writing Guidelines

```rust
/// Collects and returns system information
/// 
/// # Arguments
/// * `config` - Configuration settings
/// 
/// # Returns
/// * `Result<SystemInfo, Error>` - System information or error
/// 
/// # Examples
/// ```
/// let config = Config::default();
/// let info = collect_system_info(&config)?;
/// ```
pub fn collect_system_info(config: &Config) -> Result<SystemInfo, Error> {
    // Implementation
}
```

### Error Handling

```rust
// ✅ Correct - Use custom error types
#[derive(Debug, thiserror::Error)]
pub enum MonitoringError {
    #[error("Failed to get CPU info: {0}")]
    CpuError(String),
    #[error("Failed to get memory info: {0}")]
    MemoryError(String),
    #[error("Failed to get disk info: {0}")]
    DiskError(String),
}

// ❌ Wrong - Use generic Error
pub fn get_cpu_info() -> Result<CpuInfo, Box<dyn std::error::Error>> {
    // ...
}
```

### Logging

```rust
use log::{info, warn, error, debug};

// ✅ Correct - Use appropriate log levels
pub fn start_monitoring(config: &Config) -> Result<(), Error> {
    info!("Starting monitoring...");
    
    if config.update_interval < 1 {
        warn!("Update interval too low, setting to 1 second");
    }
    
    match initialize_monitoring(config) {
        Ok(_) => {
            info!("Monitoring started successfully");
            Ok(())
        }
        Err(e) => {
            error!("Failed to start monitoring: {}", e);
            Err(e)
        }
    }
}
```

## Writing Tests

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu_usage_calculation() {
        let cpu_info = CpuInfo {
            usage_percent: 50.0,
            temperature: Some(45.0),
            cores: 4,
        };
        
        assert_eq!(cpu_info.usage_percent, 50.0);
        assert_eq!(cpu_info.cores, 4);
        assert!(cpu_info.temperature.is_some());
    }

    #[test]
    fn test_invalid_cpu_usage() {
        let result = CpuInfo::new(-10.0);
        assert!(result.is_err());
    }
}
```

### Integration Tests

```rust
// tests/integration_test.rs
use staffmon::api::start_server;
use staffmon::config::Config;

#[tokio::test]
async fn test_api_endpoints() {
    let config = Config::default();
    let server = start_server(config).await.unwrap();
    
    // Test API endpoints
    let client = reqwest::Client::new();
    let response = client
        .get("http://localhost:8080/api/v1/system/info")
        .send()
        .await
        .unwrap();
    
    assert_eq!(response.status(), 200);
}
```

### Running Tests

```bash
# Run all tests
cargo test

# Run specific tests
cargo test test_cpu_usage

# Test coverage (requires: cargo-tarpaulin)
cargo tarpaulin --out Html

# Benchmark tests
cargo bench
```

## Pull Request Process

### 1. Create Issue

First, create an [Issue](https://github.com/forniya/StaffLinuxMonitor/issues):

- **Bug fix**: What bug are you fixing?
- **Feature**: What feature are you adding?
- **Documentation**: What documentation are you updating?

### 2. Create Branch

```bash
# Update main branch
git checkout main
git pull origin main

# Create feature branch
git checkout -b feature/your-feature-name
# or
git checkout -b fix/your-bug-fix
```

### 3. Make Changes

```bash
# Make your changes
# ...

# Stage changes
git add .

# Create commit
git commit -m "feat: Add CPU temperature monitoring feature

- Added CPU temperature sensor support
- Added temperature alerts
- Updated documentation

Closes #123"
```

### 4. Commit Message Guidelines

Use [Conventional Commits](https://www.conventionalcommits.org/) standard:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes
- `refactor`: Code refactoring
- `test`: Adding/editing tests
- `chore`: Maintenance tasks

**Examples:**
```
feat(api): Add REST API endpoints
fix(monitoring): Fix CPU usage calculation error
docs(readme): Update installation instructions
test(cpu): Add CPU monitoring tests
```

### 5. Submit Pull Request

1. Create Pull Request on GitHub
2. Fill out the template:

```markdown
## Change Type
- [ ] Bug fix
- [ ] New feature
- [ ] Documentation update
- [ ] Test addition

## Description
What does this PR do?

## Tested
- [ ] Unit tests pass
- [ ] Integration tests pass
- [ ] Manual testing done

## Screenshots
Add screenshots if applicable

## Checklist
- [ ] Code follows style guidelines
- [ ] Documentation updated
- [ ] Tests added
- [ ] Commit messages follow conventional commits
```

### 6. Code Review

- At least one maintainer approval required
- CI/CD tests must pass
- Code coverage should not decrease

## Reporting and Communication

### Security Issues

Please report security issues **privately**:

- **Email**: security@staffmon.com
- **GitHub Security**: [Security Advisory](https://github.com/forniya/StaffLinuxMonitor/security/advisories)

### Communication Channels

- **GitHub Issues**: Bug reports and feature requests
- **GitHub Discussions**: General discussions and questions
- **Discord**: [Community server](https://discord.gg/staffmon)
- **Email**: info@staffmon.com

### Community Guidelines

1. **Be respectful**: Respect everyone's opinions
2. **Be constructive**: Make criticisms constructive
3. **Be helpful**: Help other contributors
4. **Be open to learning**: Be open to new ideas and approaches

## License

By contributing to this project, you agree that your contributions will be licensed under the [MIT License](LICENSE).

## Acknowledgments

Thank you to all contributors! Your names will be listed in the [CONTRIBUTORS.md](CONTRIBUTORS.md) file.

---

**Have questions?** Ask in [GitHub Discussions](https://github.com/forniya/StaffLinuxMonitor/discussions)! 