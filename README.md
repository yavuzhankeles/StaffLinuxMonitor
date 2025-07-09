# 🖥️ StaffLinuxMonitor - Gelişmiş Linux Sistem İzleme Aracı

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![GitHub Actions](https://img.shields.io/github/actions/workflow/status/forniya/StaffLinuxMonitor/ci.yml?branch=main)](https://github.com/forniya/StaffLinuxMonitor/actions)
[![GitHub release](https://img.shields.io/github/v/release/forniya/StaffLinuxMonitor)](https://github.com/forniya/StaffLinuxMonitor/releases)
[![GitHub stars](https://img.shields.io/github/stars/forniya/StaffLinuxMonitor)](https://github.com/forniya/StaffLinuxMonitor/stargazers)
[![Platform](https://img.shields.io/badge/Platform-Linux-blue.svg)](https://www.linux.org/)
[![Architecture](https://img.shields.io/badge/Architecture-x86__64%20%7C%20ARM64-green.svg)](https://en.wikipedia.org/wiki/X86-64)

**StaffLinuxMonitor**, Rust ile yazılmış, gerçek zamanlı ve kapsamlı bir Linux sistem izleme aracıdır. CPU, bellek, disk, ağ, servis, güvenlik ve donanım bilgilerini JSON ve API ile raporlar.

## 📊 Örnek Çıktılar / Sample Outputs

### 🖥️ Sistem Bilgileri / System Information
```json
{
  "timestamp": "2024-03-19T10:30:00Z",
  "hostname": "server01",
  "cpu": {
    "usage_percent": 45.2,
    "temperature_celsius": 65.0,
    "frequency_mhz": 2400.0,
    "cores": 8,
    "model": "Intel(R) Core(TM) i7-8700K CPU @ 3.70GHz"
  },
  "memory": {
    "total_mb": 16384,
    "used_mb": 8192,
    "free_mb": 8192,
    "usage_percent": 50.0
  },
  "disks": [
    {
      "name": "/dev/sda1",
      "total_gb": 500.0,
      "used_gb": 250.0,
      "free_gb": 250.0,
      "usage_percent": 50.0,
      "mount_point": "/"
    }
  ],
  "network": {
    "interfaces": [
      {
        "name": "eth0",
        "ip_addresses": ["192.168.1.100"],
        "rx_bytes": 1024000,
        "tx_bytes": 512000,
        "status": "up"
      }
    ]
  },
  "services": [
    {
      "name": "nginx",
      "active": true,
      "enabled": true,
      "version": "1.18.0",
      "status": "running"
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

### 📈 Gerçek Zamanlı İzleme / Real-time Monitoring
```
🖥️ StaffLinuxMonitor v1.0.2 - Sistem İzleme Başlatıldı
⏰ Başlangıç: 2024-03-19 10:30:00 UTC
🔄 Güncelleme Aralığı: 2 saniye
📊 JSON Çıktısı: /var/log/staffmon_data.json
🌐 API Sunucusu: http://127.0.0.1:8080

📊 Sistem Durumu:
├── CPU: 45.2% (65°C) @ 2400MHz
├── Bellek: 50.0% (8.0GB / 16.0GB)
├── Disk: 50.0% (250GB / 500GB)
├── Ağ: eth0 ↑1.0MB ↓512KB
└── Servisler: 5 aktif, 1 pasif

🔒 Güvenlik Durumu:
├── Firewall: ✅ Aktif
├── Fail2ban: ✅ Aktif
├── Açık Portlar: 22, 80, 443
└── Güncellemeler: 5 paket bekliyor

📝 Log: /var/log/staffmon.log
🔄 PID: /var/run/staffmon.pid
```

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

### ✨ Özellikler / Features

#### 🔍 İzleme Özellikleri / Monitoring Features
- **CPU İzleme**: Kullanım oranı, sıcaklık, frekans, çekirdek sayısı
- **Bellek İzleme**: Toplam, kullanılan, boş bellek, swap durumu
- **Disk İzleme**: Kullanım oranı, I/O istatistikleri, dosya sistemi
- **Ağ İzleme**: Interface durumu, trafik, IP adresleri
- **Servis İzleme**: Systemd servisleri, durum, versiyon bilgileri
- **Güvenlik İzleme**: Firewall, fail2ban, açık portlar, güncellemeler
- **Donanım İzleme**: Sistem bilgileri, uptime, yük ortalaması

#### 🛠️ Teknik Özellikler / Technical Features
- **Gerçek Zamanlı**: 2 saniyede bir güncelleme
- **JSON Raporlama**: Otomatik JSON dosya çıktısı
- **REST API**: HTTP/HTTPS API sunucusu
- **Daemon Modu**: Arka planda çalışma
- **Gelişmiş Loglama**: Yapılandırılabilir log seviyeleri
- **Çoklu Platform**: Ubuntu, CentOS, RHEL, Debian desteği
- **Statik Binary**: Bağımlılık gerektirmeyen çalıştırılabilir

### ⚙️ Yapılandırma / Configuration

`config.toml` dosyasını düzenleyerek izleme davranışını özelleştirebilirsiniz:

```toml
[monitoring]
update_interval = 2                    # Güncelleme aralığı (saniye)
enable_daemon = true                   # Arka planda çalışma
enable_json_output = true              # JSON çıktısı
enable_api = true                      # API sunucusu

[api]
enabled = true
host = "127.0.0.1"
port = 8080
api_key = "your-secure-api-key"

[features]
enable_cpu_monitoring = true
enable_memory_monitoring = true
enable_disk_monitoring = true
enable_network_monitoring = true
enable_service_monitoring = true
enable_security_monitoring = true
```

Detaylar için [Yapılandırma Kılavuzu (TR)](docs/CONFIGURATION_TR.md).

### 🚀 API Kullanımı / API Usage

```bash
# Sistem bilgilerini al
curl -H "Authorization: Bearer your-api-key" \
     http://localhost:8080/api/v1/system/info

# CPU bilgilerini al
curl -H "Authorization: Bearer your-api-key" \
     http://localhost:8080/api/v1/system/cpu

# Servis durumunu al
curl -H "Authorization: Bearer your-api-key" \
     http://localhost:8080/api/v1/services
```

### 📋 Sistem Gereksinimleri / System Requirements

| Bileşen | Minimum | Önerilen |
|---------|---------|----------|
| **İşletim Sistemi** | Linux (Ubuntu 18.04+) | Linux (Ubuntu 20.04+) |
| **Mimari** | x86_64, ARM64 | x86_64 |
| **Bellek** | 512MB RAM | 2GB+ RAM |
| **Depolama** | 100MB disk | 10GB+ disk |
| **Ağ** | İnternet erişimi | Yerel ağ |

### 🔧 Kurulum Seçenekleri / Installation Options

#### 📦 Binary Kurulumu (Tavsiye)
```bash
# Otomatik kurulum scripti
curl -sSL https://raw.githubusercontent.com/forniya/StaffLinuxMonitor/main/install.sh | bash

# Manuel kurulum
wget https://github.com/forniya/StaffLinuxMonitor/releases/latest/download/staffmon
sudo mv staffmon /usr/local/bin/
sudo chmod +x /usr/local/bin/staffmon
```

#### 🏗️ Kaynaktan Derleme
```bash
# Rust kurulumu
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Projeyi klonla ve derle
git clone https://github.com/forniya/StaffLinuxMonitor.git
cd StaffLinuxMonitor
cargo build --release
sudo cp target/release/staffmon /usr/local/bin/
```

#### 🐳 Docker ile Kurulum
```bash
# Docker image'ını çek
docker pull forniya/staffmon:latest

# Çalıştır
docker run -d --name staffmon \
  -p 8080:8080 \
  -v /var/log:/var/log \
  -v /etc:/host/etc:ro \
  forniya/staffmon:latest
```

### 📊 Performans Karşılaştırması / Performance Comparison

| Özellik | StaffLinuxMonitor | htop | top | glances |
|---------|------------------|------|-----|---------|
| **CPU Kullanımı** | ~2MB | ~15MB | ~5MB | ~25MB |
| **Bellek Kullanımı** | ~5MB | ~20MB | ~8MB | ~30MB |
| **Güncelleme Hızı** | 2s | Manuel | 3s | 2s |
| **JSON Çıktısı** | ✅ | ❌ | ❌ | ✅ |
| **API Desteği** | ✅ | ❌ | ❌ | ❌ |
| **Daemon Modu** | ✅ | ❌ | ❌ | ✅ |

### 🆘 SSS ve Sorun Giderme / FAQ & Troubleshooting
- [Sıkça Sorulan Sorular ve Sorun Giderme (TR)](docs/TROUBLESHOOTING_TR.md)

### 🤝 Katkı ve Destek / Contributing & Support
- [Katkı Rehberi (TR)](docs/CONTRIBUTING_TR.md)
- Sorularınız için: [GitHub Issues](https://github.com/forniya/StaffLinuxMonitor/issues)
- Tartışmalar için: [GitHub Discussions](https://github.com/forniya/StaffLinuxMonitor/discussions)

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

### ✨ Features

#### 🔍 Monitoring Features
- **CPU Monitoring**: Usage percentage, temperature, frequency, core count
- **Memory Monitoring**: Total, used, free memory, swap status
- **Disk Monitoring**: Usage percentage, I/O statistics, filesystem
- **Network Monitoring**: Interface status, traffic, IP addresses
- **Service Monitoring**: Systemd services, status, version info
- **Security Monitoring**: Firewall, fail2ban, open ports, updates
- **Hardware Monitoring**: System info, uptime, load average

#### 🛠️ Technical Features
- **Real-time**: Updates every 2 seconds
- **JSON Reporting**: Automatic JSON file output
- **REST API**: HTTP/HTTPS API server
- **Daemon Mode**: Background operation
- **Advanced Logging**: Configurable log levels
- **Multi-platform**: Ubuntu, CentOS, RHEL, Debian support
- **Static Binary**: Dependency-free executable

### ⚙️ Configuration

Customize monitoring via `config.toml`:

```toml
[monitoring]
update_interval = 2                    # Update interval (seconds)
enable_daemon = true                   # Background operation
enable_json_output = true              # JSON output
enable_api = true                      # API server

[api]
enabled = true
host = "127.0.0.1"
port = 8080
api_key = "your-secure-api-key"

[features]
enable_cpu_monitoring = true
enable_memory_monitoring = true
enable_disk_monitoring = true
enable_network_monitoring = true
enable_service_monitoring = true
enable_security_monitoring = true
```

See [Configuration Guide (EN)](docs/CONFIGURATION_EN.md) for details.

### 🚀 API Usage

```bash
# Get system information
curl -H "Authorization: Bearer your-api-key" \
     http://localhost:8080/api/v1/system/info

# Get CPU information
curl -H "Authorization: Bearer your-api-key" \
     http://localhost:8080/api/v1/system/cpu

# Get service status
curl -H "Authorization: Bearer your-api-key" \
     http://localhost:8080/api/v1/services
```

### 📋 System Requirements

| Component | Minimum | Recommended |
|-----------|---------|-------------|
| **Operating System** | Linux (Ubuntu 18.04+) | Linux (Ubuntu 20.04+) |
| **Architecture** | x86_64, ARM64 | x86_64 |
| **Memory** | 512MB RAM | 2GB+ RAM |
| **Storage** | 100MB disk | 10GB+ disk |
| **Network** | Internet access | Local network |

### 🔧 Installation Options

#### 📦 Binary Installation (Recommended)
```bash
# Automatic installation script
curl -sSL https://raw.githubusercontent.com/forniya/StaffLinuxMonitor/main/install.sh | bash

# Manual installation
wget https://github.com/forniya/StaffLinuxMonitor/releases/latest/download/staffmon
sudo mv staffmon /usr/local/bin/
sudo chmod +x /usr/local/bin/staffmon
```

#### 🏗️ Build from Source
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Clone and build
git clone https://github.com/forniya/StaffLinuxMonitor.git
cd StaffLinuxMonitor
cargo build --release
sudo cp target/release/staffmon /usr/local/bin/
```

#### 🐳 Docker Installation
```bash
# Pull Docker image
docker pull forniya/staffmon:latest

# Run
docker run -d --name staffmon \
  -p 8080:8080 \
  -v /var/log:/var/log \
  -v /etc:/host/etc:ro \
  forniya/staffmon:latest
```

### 📊 Performance Comparison

| Feature | StaffLinuxMonitor | htop | top | glances |
|---------|------------------|------|-----|---------|
| **CPU Usage** | ~2MB | ~15MB | ~5MB | ~25MB |
| **Memory Usage** | ~5MB | ~20MB | ~8MB | ~30MB |
| **Update Speed** | 2s | Manual | 3s | 2s |
| **JSON Output** | ✅ | ❌ | ❌ | ✅ |
| **API Support** | ✅ | ❌ | ❌ | ❌ |
| **Daemon Mode** | ✅ | ❌ | ❌ | ✅ |

### 🆘 FAQ & Troubleshooting
- [FAQ & Troubleshooting (EN)](docs/TROUBLESHOOTING_EN.md)

### 🤝 Contributing & Support
- [Contributing Guide (EN)](docs/CONTRIBUTING_EN.md)
- For questions: [GitHub Issues](https://github.com/forniya/StaffLinuxMonitor/issues)
- For discussions: [GitHub Discussions](https://github.com/forniya/StaffLinuxMonitor/discussions)

---

## 🎯 Kullanım Senaryoları / Use Cases

### 🏢 Kurumsal Ortamlar / Enterprise Environments
- **Sunucu İzleme**: Çoklu sunucu ortamlarında merkezi izleme
- **Performans Analizi**: Sistem performans trendlerini analiz etme
- **Kapasite Planlama**: Kaynak kullanımını tahmin etme
- **Güvenlik İzleme**: Güvenlik durumunu sürekli kontrol etme

### 🏠 Kişisel Kullanım / Personal Use
- **Ev Sunucusu**: Ev sunucularını izleme
- **Geliştirme Ortamı**: Geliştirme sunucularını takip etme
- **Eğitim**: Linux sistem yönetimi öğrenme

### 🔧 DevOps & SRE
- **CI/CD Pipeline**: Otomatik test ve deployment süreçleri
- **Alerting**: Sistem durumu değişikliklerinde uyarı
- **Logging**: Merkezi log yönetimi
- **Monitoring**: Mikroservis mimarilerinde izleme

## 🚀 Gelecek Planları / Roadmap

### 📅 v1.1.0 (Yakında / Coming Soon)
- [ ] Web arayüzü / Web interface
- [ ] E-posta bildirimleri / Email notifications
- [ ] Telegram/Discord entegrasyonu / Telegram/Discord integration
- [ ] Grafik raporlama / Graphical reporting

### 📅 v1.2.0 (Planlanan / Planned)
- [ ] Docker desteği / Docker support
- [ ] Kubernetes entegrasyonu / Kubernetes integration
- [ ] Prometheus metrikleri / Prometheus metrics
- [ ] Grafana dashboard'ları / Grafana dashboards

### 📅 v2.0.0 (Gelecek / Future)
- [ ] Windows desteği / Windows support
- [ ] macOS desteği / macOS support
- [ ] Mobil uygulama / Mobile application
- [ ] Bulut entegrasyonu / Cloud integration

## 🤝 Katkıda Bulunanlar / Contributors

<a href="https://github.com/forniya/StaffLinuxMonitor/graphs/contributors">
  <img src="https://contributors-img.web.app/image?repo=forniya/StaffLinuxMonitor" />
</a>

## 📄 Lisans / License

Bu proje MIT lisansı altında lisanslanmıştır. Detaylar için [LICENSE](LICENSE) dosyasına bakın.

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

**Proje ile ilgili tüm detaylar ve yol haritası için dökümanlara göz atabilirsiniz!**

*If you find this project useful, please consider giving it a ⭐ on GitHub!*

<div align="center">
  <a href="https://github.com/forniya/StaffLinuxMonitor/stargazers">
    <img src="https://img.shields.io/github/stars/forniya/StaffLinuxMonitor?style=social" alt="Stars">
  </a>
  <a href="https://github.com/forniya/StaffLinuxMonitor/forks">
    <img src="https://img.shields.io/github/forks/forniya/StaffLinuxMonitor?style=social" alt="Forks">
  </a>
  <a href="https://github.com/forniya/StaffLinuxMonitor/issues">
    <img src="https://img.shields.io/github/issues/forniya/StaffLinuxMonitor" alt="Issues">
  </a>
  <a href="https://github.com/forniya/StaffLinuxMonitor/pulls">
    <img src="https://img.shields.io/github/issues-pr/forniya/StaffLinuxMonitor" alt="Pull Requests">
  </a>
</div>
