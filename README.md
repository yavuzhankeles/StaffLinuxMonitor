# 🖥️ StaffLinuxMonitor - Gelişmiş Linux Sistem İzleme Aracı

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![GitHub Actions](https://img.shields.io/github/actions/workflow/status/forniya/StaffLinuxMonitor/ci.yml?branch=main)](https://github.com/forniya/StaffLinuxMonitor/actions)
[![GitHub release](https://img.shields.io/github/v/release/forniya/StaffLinuxMonitor)](https://github.com/forniya/StaffLinuxMonitor/releases)
[![GitHub stars](https://img.shields.io/github/stars/forniya/StaffLinuxMonitor)](https://github.com/forniya/StaffLinuxMonitor/stargazers)

**StaffLinuxMonitor**, Rust ile yazılmış, gerçek zamanlı ve kapsamlı bir Linux sistem izleme aracıdır. CPU, bellek, disk, ağ, servis, güvenlik ve donanım bilgilerini JSON ve API ile raporlar.

## 📚 Belgeler / Documentation
- [API Dökümanı (TR)](docs/API_TR.md) | [API Documentation (EN)](docs/API_EN.md)
- [Yapılandırma Kılavuzu (TR)](docs/CONFIGURATION_TR.md) | [Configuration Guide (EN)](docs/CONFIGURATION_EN.md)
- [Kurulum ve Dağıtım (TR)](docs/DEPLOYMENT_TR.md) | [Deployment Guide (EN)](docs/DEPLOYMENT_EN.md)
- [Sorun Giderme (TR)](docs/TROUBLESHOOTING_TR.md) | [Troubleshooting (EN)](docs/TROUBLESHOOTING_EN.md)
- [Katkıda Bulunma (TR)](docs/CONTRIBUTING_TR.md) | [Contributing (EN)](docs/CONTRIBUTING_EN.md)

---

## 🇹🇷 Türkçe Kullanım

### 🚀 Hızlı Başlangıç

#### Binary Kurulumu (Tavsiye Edilen)
1. Son sürümü indirin:
   ```bash
   wget https://github.com/forniya/StaffLinuxMonitor/releases/latest/download/staffmon
   chmod +x staffmon
   ./staffmon
   ```

#### Kaynaktan Derleme
1. Rust kurun:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```
2. Repoyu klonlayın ve derleyin:
   ```bash
   git clone https://github.com/forniya/StaffLinuxMonitor.git
   cd StaffLinuxMonitor
   cargo build --release
   ./target/release/staffmon
   ```

### Özellikler
- Gerçek zamanlı CPU, bellek, disk, ağ, servis, güvenlik ve donanım izleme
- JSON ve API ile raporlama
- Daemon (arka plan) modu
- Gelişmiş loglama
- Çoklu dağıtım desteği
- 2 saniyede bir güncelleme

### Yapılandırma
`config.toml` dosyasını düzenleyerek izleme davranışını özelleştirebilirsiniz. Detaylar için [Yapılandırma Kılavuzu (TR)](docs/CONFIGURATION_TR.md).

### SSS ve Sorun Giderme
- [Sıkça Sorulan Sorular ve Sorun Giderme (TR)](docs/TROUBLESHOOTING_TR.md)

### Katkı ve Destek
- [Katkı Rehberi (TR)](docs/CONTRIBUTING_TR.md)
- Sorularınız için: [GitHub Issues](https://github.com/forniya/StaffLinuxMonitor/issues)

---

# 🇬🇧 English Version

## 🖥️ StaffLinuxMonitor - Advanced Linux System Monitoring Tool

**StaffLinuxMonitor** is a powerful, real-time Linux system monitoring tool written in Rust. It provides comprehensive system insights including CPU, memory, disk usage, network statistics, service status, security information, and hardware details with automatic JSON reporting and API integration.

## 📚 Documentation
- [API Documentation (EN)](docs/API_EN.md) | [API Dökümanı (TR)](docs/API_TR.md)
- [Configuration Guide (EN)](docs/CONFIGURATION_EN.md) | [Yapılandırma Kılavuzu (TR)](docs/CONFIGURATION_TR.md)
- [Deployment Guide (EN)](docs/DEPLOYMENT_EN.md) | [Kurulum ve Dağıtım (TR)](docs/DEPLOYMENT_TR.md)
- [Troubleshooting (EN)](docs/TROUBLESHOOTING_EN.md) | [Sorun Giderme (TR)](docs/TROUBLESHOOTING_TR.md)
- [Contributing (EN)](docs/CONTRIBUTING_EN.md) | [Katkıda Bulunma (TR)](docs/CONTRIBUTING_TR.md)

---

## 🚀 Quick Start

### Binary Installation (Recommended)
1. Download the latest release:
   ```bash
   wget https://github.com/forniya/StaffLinuxMonitor/releases/latest/download/staffmon
   chmod +x staffmon
   ./staffmon
   ```

### Build from Source
1. Install Rust:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```
2. Clone and build:
   ```bash
   git clone https://github.com/forniya/StaffLinuxMonitor.git
   cd StaffLinuxMonitor
   cargo build --release
   ./target/release/staffmon
   ```

### Features
- Real-time monitoring: CPU, memory, disk, network, services, security, hardware
- JSON and API reporting
- Daemon (background) mode
- Advanced logging
- Multi-distribution support
- 2-second update interval

### Configuration
Customize monitoring via `config.toml`. See [Configuration Guide (EN)](docs/CONFIGURATION_EN.md) for details.

### FAQ & Troubleshooting
- [FAQ & Troubleshooting (EN)](docs/TROUBLESHOOTING_EN.md)

### Contributing & Support
- [Contributing Guide (EN)](docs/CONTRIBUTING_EN.md)
- For questions: [GitHub Issues](https://github.com/forniya/StaffLinuxMonitor/issues)

---

**Proje ile ilgili tüm detaylar ve yol haritası için dökümanlara göz atabilirsiniz!**

*If you find this project useful, please consider giving it a ⭐ on GitHub!*
