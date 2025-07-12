# StaffMon Documentation Index

Welcome to the StaffMon documentation! This index provides links to all available documentation for the StaffMon Linux system monitoring tool.

## 📚 Available Documentation

### Core Documentation

#### 1. [API Documentation](./API_DOCUMENTATION.md)
Comprehensive documentation for all public APIs, functions, and components.
- Data structures and their fields
- API client methods
- Configuration options
- Core functions for system monitoring
- Usage examples with code snippets
- Error handling guidelines

#### 2. [Quick Reference Guide](./QUICK_REFERENCE.md)
A concise guide for developers who want to quickly get started with StaffMon.
- Installation instructions
- Basic usage examples
- Key functions and methods reference
- Common use cases
- Tips and best practices

#### 3. [Developer Guide](./DEVELOPER_GUIDE.md)
In-depth guide for extending and customizing StaffMon.
- Architecture overview
- Creating custom collectors
- Plugin development
- API integration patterns
- Testing strategies
- Contributing guidelines

### Language-Specific Documentation

#### English Documentation
- [API Documentation (EN)](./API_EN.md) - API reference in English
- [Configuration Guide (EN)](./CONFIGURATION_EN.md) - Configuration options
- [Deployment Guide (EN)](./DEPLOYMENT_EN.md) - Deployment instructions
- [Troubleshooting (EN)](./TROUBLESHOOTING_EN.md) - Common issues and solutions
- [Contributing (EN)](./CONTRIBUTING_EN.md) - Contribution guidelines

#### Turkish Documentation
- [API Documentation (TR)](./API_TR.md) - API referansı (Türkçe)
- [Configuration Guide (TR)](./CONFIGURATION_TR.md) - Yapılandırma seçenekleri
- [Deployment Guide (TR)](./DEPLOYMENT_TR.md) - Kurulum talimatları
- [Troubleshooting (TR)](./TROUBLESHOOTING_TR.md) - Yaygın sorunlar ve çözümler
- [Contributing (TR)](./CONTRIBUTING_TR.md) - Katkı kuralları

## 🚀 Getting Started

For new users, we recommend starting with:
1. [Quick Reference Guide](./QUICK_REFERENCE.md) - Get up and running quickly
2. [API Documentation](./API_DOCUMENTATION.md) - Understand the available APIs
3. [Configuration Guide](./CONFIGURATION_EN.md) - Configure StaffMon for your needs

For developers looking to extend StaffMon:
1. [Developer Guide](./DEVELOPER_GUIDE.md) - Learn the architecture and extension points
2. [API Documentation](./API_DOCUMENTATION.md) - Reference for all structures and methods
3. [Contributing Guide](./CONTRIBUTING_EN.md) - Guidelines for contributing code

## 📋 Documentation Structure

```
docs/
├── INDEX.md                    # This file
├── API_DOCUMENTATION.md        # Comprehensive API reference
├── QUICK_REFERENCE.md          # Quick start guide
├── DEVELOPER_GUIDE.md          # Developer and extension guide
├── API_EN.md                   # API docs (English)
├── API_TR.md                   # API docs (Turkish)
├── CONFIGURATION_EN.md         # Configuration (English)
├── CONFIGURATION_TR.md         # Configuration (Turkish)
├── DEPLOYMENT_EN.md            # Deployment (English)
├── DEPLOYMENT_TR.md            # Deployment (Turkish)
├── TROUBLESHOOTING_EN.md       # Troubleshooting (English)
├── TROUBLESHOOTING_TR.md       # Troubleshooting (Turkish)
├── CONTRIBUTING_EN.md          # Contributing (English)
└── CONTRIBUTING_TR.md          # Contributing (Turkish)
```

## 🔑 Key Features Documented

### System Monitoring
- CPU usage and frequency monitoring
- Memory usage tracking
- Disk space monitoring
- Network interface statistics
- System load averages
- Process monitoring

### Security Features
- Firewall status checking
- Fail2ban monitoring
- Open port detection
- Package update tracking
- User access logging

### Service Management
- Service status monitoring
- Service version detection
- Support for multiple init systems (systemd, init.d, rc.d)

### Data Management
- JSON data export
- Remote API integration
- Configurable data collection intervals
- Daemon mode operation

### Extensibility
- Plugin system
- Custom collectors
- Webhook integration
- Custom API endpoints

## 📊 Example Data Structures

All data structures are fully documented in the [API Documentation](./API_DOCUMENTATION.md). Here's a quick overview:

- **SystemInfo**: Main structure containing all system data
- **CpuInfo**: CPU metrics (usage, temperature, frequency)
- **MemoryInfo**: Memory usage statistics
- **DiskInfo**: Disk space information
- **NetworkInfo**: Network interface data
- **ServiceInfo**: System service status
- **SecurityInfo**: Security-related metrics
- **HardwareInfo**: Hardware specifications
- **ProcessInfo**: Process details

## 🛠️ Development Tools

The documentation includes information about:
- Unit testing strategies
- Integration testing
- Benchmark testing
- Performance optimization
- Error handling patterns
- Logging configuration

## 📞 Support

For additional help:
- Check the [Troubleshooting Guide](./TROUBLESHOOTING_EN.md)
- Review the [FAQ section](./TROUBLESHOOTING_EN.md#faq)
- Submit issues on [GitHub](https://github.com/forniya/StaffLinuxMonitor)

## 🤝 Contributing

We welcome contributions! Please see:
- [Contributing Guide (EN)](./CONTRIBUTING_EN.md) for English speakers
- [Contributing Guide (TR)](./CONTRIBUTING_TR.md) for Turkish speakers

## 📄 License

StaffMon is licensed under the MIT License. See the LICENSE file in the repository root for details.