# 🖥️ StaffLinuxMonitor - Advanced Linux System Monitoring Tool

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![GitHub Actions](https://img.shields.io/github/actions/workflow/status/forniya/StaffLinuxMonitor/ci.yml?branch=main)](https://github.com/forniya/StaffLinuxMonitor/actions)
[![GitHub release](https://img.shields.io/github/v/release/forniya/StaffLinuxMonitor)](https://github.com/forniya/StaffLinuxMonitor/releases)
[![GitHub stars](https://img.shields.io/github/stars/forniya/StaffLinuxMonitor)](https://github.com/forniya/StaffLinuxMonitor/stargazers)

**StaffLinuxMonitor** is a powerful, real-time Linux system monitoring tool written in Rust. It provides comprehensive system insights including CPU, memory, disk usage, network statistics, service status, security information, and hardware details with automatic JSON reporting and API integration.

## 🌟 Key Features

### 📊 System Monitoring
- **Real-time CPU monitoring** with usage percentage, temperature, and frequency
- **Memory tracking** with detailed RAM usage statistics
- **Disk space monitoring** across all mounted filesystems
- **Network interface monitoring** with traffic statistics
- **System load average** tracking (1, 5, 15 minute averages)
- **Process monitoring** with resource usage details

### 🔒 Security & Access Monitoring
- **SSH login tracking** and user access monitoring
- **Service status monitoring** (systemd services)
- **Firewall status** and open ports detection
- **Fail2ban integration** and security alerts
- **Package update notifications** for multiple package managers

### 🛠️ Advanced Features
- **Daemon mode** for background operation
- **REST API integration** for remote monitoring
- **JSON export** with timestamped data
- **Comprehensive logging** with configurable levels
- **Multi-distribution support** (Ubuntu, CentOS, Arch, etc.)
- **Hardware information** collection
- **Uptime tracking** with reboot history

### 🔧 Technical Capabilities
- **2-second update intervals** for real-time monitoring
- **Cross-platform compatibility** (Linux distributions)
- **Static binary compilation** for easy deployment
- **Resource-efficient** with minimal system impact
- **Extensible architecture** for custom plugins

## 🚀 Quick Start

### Binary Installation (Recommended)

1. **Download the latest release:**
   ```bash
   wget https://github.com/forniya/StaffLinuxMonitor/releases/latest/download/staffmon
   ```

2. **Make it executable:**
   ```bash
   chmod +x staffmon
   ```

3. **Run the monitor:**
   ```bash
   ./staffmon
   ```

### Build from Source

1. **Install Rust:**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```

2. **Clone the repository:**
   ```bash
   git clone https://github.com/forniya/StaffLinuxMonitor.git
   cd StaffLinuxMonitor
   ```

3. **Build the project:**
   ```bash
   cargo build --release
   ```

4. **Run the application:**
   ```bash
   ./target/release/staffmon
   ```

## 📖 Usage Guide

### Basic Usage

```bash
# Start monitoring in foreground
./staffmon

# Start in daemon mode (background)
./staffmon --daemon

# Start with custom config file
./staffmon --config /path/to/config.toml

# Start with specific log level
./staffmon --log-level debug
```

### Configuration

Create a `config.toml` file for custom settings:

```toml
# API Configuration
base_url = "http://localhost:8080"
api_key = "your-api-key-here"
timeout_seconds = 30
retry_count = 3
rate_limit = 100

# Monitoring Settings
update_interval = 2
enable_daemon = false
log_level = "info"
log_file = "/var/log/staffmon.log"
pid_file = "/var/run/staffmon.pid"

# Export Settings
json_output = true
json_file = "/var/log/staffmon_data.json"
```

### Output Format

The tool generates JSON reports with the following structure:

```json
{
  "timestamp": "2024-03-19T10:30:00Z",
  "hostname": "server01",
  "cpu": {
    "usage_percent": 45.2,
    "temperature_celsius": 65.0,
    "frequency_mhz": 2400.0
  },
  "memory": {
    "total_mb": 16384,
    "used_mb": 8192,
    "free_mb": 8192
  },
  "disks": [
    {
      "name": "/dev/sda1",
      "total_gb": 500.0,
      "used_gb": 250.0,
      "free_gb": 250.0,
      "mount_point": "/"
    }
  ],
  "network": {
    "interfaces": [
      {
        "name": "eth0",
        "ip_addresses": ["192.168.1.100"],
        "rx_bytes": 1024000,
        "tx_bytes": 512000
      }
    ]
  },
  "services": [
    {
      "name": "nginx",
      "active": true,
      "enabled": true,
      "version": "1.18.0"
    }
  ],
  "security": {
    "firewall_enabled": true,
    "fail2ban_active": true,
    "open_ports": [22, 80, 443],
    "package_updates": ["nginx", "openssl"]
  }
}
```

## 🏗️ Architecture

### Core Components

- **System Monitor**: Real-time system metrics collection
- **API Client**: REST API integration for remote monitoring
- **Logger**: Configurable logging system with multiple levels
- **Daemon Manager**: Background process management
- **Configuration Manager**: Flexible configuration handling

### Dependencies

- **sysinfo**: System information gathering
- **serde**: JSON serialization/deserialization
- **tokio**: Async runtime for API operations
- **log4rs**: Advanced logging capabilities
- **daemonize**: Daemon process management

## 🔧 Development

### Prerequisites

- Rust 1.70 or higher
- Linux development headers
- Git

### Development Setup

1. **Clone and setup:**
   ```bash
   git clone https://github.com/forniya/StaffLinuxMonitor.git
   cd StaffLinuxMonitor
   ```

2. **Install dependencies:**
   ```bash
   cargo build
   ```

3. **Run tests:**
   ```bash
   cargo test
   ```

4. **Run with debug logging:**
   ```bash
   RUST_LOG=debug cargo run
   ```

### Project Structure

```
StaffLinuxMonitor/
├── src/
│   ├── main.rs          # Main application entry point
│   ├── api.rs           # API client implementation
│   ├── config.rs        # Configuration management
│   └── log_config.rs    # Logging configuration
├── Cargo.toml           # Rust dependencies and metadata
├── config.toml          # Default configuration
├── README.md            # This file
└── RELEASE_NOTES.md     # Version history
```

## 🗺️ Roadmap

### 🎯 Short Term (1-3 months)

#### Core Improvements
- [ ] **Performance Optimization**
  - Reduce memory footprint by 20%
  - Optimize CPU usage during monitoring
  - Implement efficient data structures

- [ ] **Enhanced Monitoring**
  - GPU monitoring support (NVIDIA/AMD)
  - Docker container monitoring
  - Kubernetes pod monitoring
  - Custom metric collection

- [ ] **Configuration Enhancements**
  - YAML configuration support
  - Environment variable configuration
  - Hot-reload configuration changes
  - Multiple configuration profiles

#### User Experience
- [ ] **CLI Improvements**
  - Interactive TUI (Terminal User Interface)
  - Color-coded output
  - Progress bars and status indicators
  - Command-line argument validation

- [ ] **Output Formats**
  - CSV export support
  - Prometheus metrics format
  - InfluxDB line protocol
  - Custom template-based output

### 🚀 Medium Term (3-6 months)

#### Advanced Features
- [ ] **Alerting System**
  - Email notifications
  - Slack/Discord integration
  - Telegram bot integration
  - Custom webhook support
  - Threshold-based alerts

- [ ] **Data Management**
  - SQLite database integration
  - Data retention policies
  - Historical data analysis
  - Data compression and archiving

- [ ] **Security Enhancements**
  - TLS/SSL encryption for API
  - JWT authentication
  - Role-based access control
  - Audit logging
  - Security vulnerability scanning

#### Integration & APIs
- [ ] **External Integrations**
  - Grafana dashboard templates
  - Prometheus exporter
  - Elasticsearch integration
  - Splunk integration
  - Zabbix agent compatibility

- [ ] **API Enhancements**
  - GraphQL API support
  - WebSocket real-time updates
  - REST API documentation (OpenAPI/Swagger)
  - API rate limiting and throttling

### 🌟 Long Term (6-12 months)

#### Platform Expansion
- [ ] **Cross-Platform Support**
  - Windows monitoring support
  - macOS monitoring support
  - BSD systems support
  - ARM architecture optimization

- [ ] **Cloud Integration**
  - AWS CloudWatch integration
  - Azure Monitor integration
  - Google Cloud Monitoring
  - Multi-cloud monitoring

#### Advanced Analytics
- [ ] **Machine Learning**
  - Anomaly detection
  - Predictive maintenance
  - Resource usage forecasting
  - Automated optimization recommendations

- [ ] **Visualization**
  - Web-based dashboard
  - Real-time charts and graphs
  - Custom dashboard builder
  - Mobile-responsive interface

#### Enterprise Features
- [ ] **Scalability**
  - Distributed monitoring
  - Load balancing
  - High availability setup
  - Multi-tenant support

- [ ] **Compliance & Governance**
  - GDPR compliance features
  - Audit trail management
  - Data privacy controls
  - Regulatory reporting

## ❓ Frequently Asked Questions (FAQ)

### General Questions

**Q: What Linux distributions are supported?**
A: StaffLinuxMonitor supports all major Linux distributions including Ubuntu, CentOS, RHEL, Debian, Arch Linux, Fedora, and SUSE. The tool automatically detects the package manager and adapts accordingly.

**Q: How much system resources does the monitor use?**
A: The monitor is designed to be lightweight, typically using less than 1% CPU and 50MB RAM. The exact usage depends on the monitoring interval and enabled features.

**Q: Can I run multiple instances simultaneously?**
A: Yes, you can run multiple instances with different configuration files. Each instance will create its own PID file and log files.

**Q: Is the tool suitable for production environments?**
A: Yes, StaffLinuxMonitor is designed for production use with features like daemon mode, comprehensive logging, and error handling.

### Installation & Setup

**Q: Do I need root privileges to run the monitor?**
A: Most features work without root privileges, but some system information (like detailed hardware info) may require elevated permissions.

**Q: How do I update the tool?**
A: Download the latest binary from the releases page or rebuild from source. The tool can be updated without stopping the daemon.

**Q: Can I install it via package managers?**
A: Currently, the tool is distributed as a binary. Package manager support is planned for future releases.

**Q: What are the minimum system requirements?**
A: Any modern Linux system with 512MB RAM and 100MB disk space. The tool works on both x86_64 and ARM architectures.

### Configuration & Usage

**Q: How do I configure the monitoring interval?**
A: Set the `update_interval` parameter in your config.toml file. The default is 2 seconds.

**Q: Can I disable specific monitoring features?**
A: Yes, you can configure which features to enable/disable in the configuration file to reduce resource usage.

**Q: How do I enable daemon mode?**
A: Use the `--daemon` flag or set `enable_daemon = true` in your configuration file.

**Q: Where are the log files stored?**
A: By default, logs are stored in `/var/log/staffmon.log`. You can customize this in the configuration.

### Troubleshooting

**Q: The tool shows "Permission denied" errors**
A: Ensure the binary has execute permissions (`chmod +x staffmon`) and check if you need elevated privileges for certain features.

**Q: CPU temperature is showing as "None"**
A: CPU temperature monitoring depends on hardware support and kernel modules. Some systems may not expose temperature sensors.

**Q: API calls are failing**
A: Check your network connectivity, API endpoint configuration, and ensure the API key is correct.

**Q: The daemon won't start**
A: Check if another instance is running, verify the PID file location, and ensure proper permissions for log and PID directories.

### Performance & Optimization

**Q: How can I reduce the tool's resource usage?**
A: Increase the monitoring interval, disable unused features, and use daemon mode for background operation.

**Q: Can I limit the JSON file size?**
A: Implement log rotation or data retention policies in your configuration to manage file sizes.

**Q: How do I monitor specific services only?**
A: Configure the services list in your configuration file to monitor only the services you need.

## 📚 Documentation

### API Reference

The tool provides a REST API for remote monitoring. See the [API Documentation](docs/API.md) for detailed endpoint information.

### Configuration Reference

Complete configuration options and examples are available in the [Configuration Guide](docs/CONFIGURATION.md).

### Deployment Guide

Step-by-step deployment instructions for various environments are provided in the [Deployment Guide](docs/DEPLOYMENT.md).

### Troubleshooting Guide

Common issues and solutions are documented in the [Troubleshooting Guide](docs/TROUBLESHOOTING.md).

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Guidelines

1. **Fork the repository**
2. **Create a feature branch** (`git checkout -b feature/amazing-feature`)
3. **Make your changes** following the coding standards
4. **Add tests** for new functionality
5. **Commit your changes** (`git commit -m 'Add amazing feature'`)
6. **Push to the branch** (`git push origin feature/amazing-feature`)
7. **Open a Pull Request**

### Code Standards

- Follow Rust coding conventions
- Add comprehensive documentation
- Include unit tests for new features
- Update documentation for API changes
- Ensure all tests pass before submitting

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [sysinfo](https://github.com/GuillaumeGomez/sysinfo) - System information library
- [serde](https://serde.rs/) - Serialization framework
- [tokio](https://tokio.rs/) - Async runtime
- [log4rs](https://github.com/estk/log4rs) - Logging framework

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/forniya/StaffLinuxMonitor/issues)
- **Discussions**: [GitHub Discussions](https://github.com/forniya/StaffLinuxMonitor/discussions)
- **Documentation**: [Wiki](https://github.com/forniya/StaffLinuxMonitor/wiki)

## 📊 Project Status

![GitHub stars](https://img.shields.io/github/stars/forniya/StaffLinuxMonitor)
![GitHub forks](https://img.shields.io/github/forks/forniya/StaffLinuxMonitor)
![GitHub issues](https://img.shields.io/github/issues/forniya/StaffLinuxMonitor)
![GitHub pull requests](https://img.shields.io/github/issues-pr/forniya/StaffLinuxMonitor)

---

**Made with ❤️ by the StaffLinuxMonitor Team**

*If you find this project useful, please consider giving it a ⭐ on GitHub!* 