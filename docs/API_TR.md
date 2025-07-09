# API Dökümanı

## Genel Bakış

StaffLinuxMonitor, uzaktan izleme ve veri toplama için bir REST API sunar. API ile sistem bilgilerini alabilir, izleme ayarlarını yapılandırabilir ve daemon'u yönetebilirsiniz.

## Temel URL

```
http://localhost:8080/api/v1
```

## Kimlik Doğrulama

Tüm API isteklerinde bir API anahtarı gereklidir. İstek başlıklarına ekleyin:

```
Authorization: Bearer api-anahtarınız
```

## Endpointler

### Sistem Bilgisi

#### GET /system/info

Kapsamlı sistem bilgisini döndürür.

**Yanıt:**
```json
{
  "timestamp": "2024-03-19T10:30:00Z",
  "hostname": "server01",
  "cpu": {
    "usage_percent": 45.2,
    "temperature_celsius": 65.0,
    "frequency_mhz": 2400.0
  },
  ...
}
```

#### GET /system/cpu

CPU bilgilerini döndürür.

**Yanıt:**
```json
{
  "usage_percent": 45.2,
  "temperature_celsius": 65.0,
  "frequency_mhz": 2400.0,
  "cores": 8,
  "model": "Intel(R) Core(TM) i7-8700K CPU @ 3.70GHz"
}
```

#### GET /system/memory

Bellek kullanım bilgisini döndürür.

**Yanıt:**
```json
{
  "total_mb": 16384,
  "used_mb": 8192,
  "free_mb": 8192,
  "usage_percent": 50.0,
  "swap_total_mb": 8192,
  "swap_used_mb": 1024,
  "swap_free_mb": 7168
}
```

#### GET /system/disks

Disk kullanım bilgisini döndürür.

**Yanıt:**
```json
[
  {
    "name": "/dev/sda1",
    "total_gb": 500.0,
    "used_gb": 250.0,
    "free_gb": 250.0,
    "usage_percent": 50.0,
    "mount_point": "/",
    "filesystem": "ext4"
  }
]
```

#### GET /system/network

Ağ arayüzü bilgilerini döndürür.

**Yanıt:**
```json
{
  "interfaces": [
    {
      "name": "eth0",
      "ip_addresses": ["192.168.1.100"],
      "mac_address": "00:11:22:33:44:55",
      "rx_bytes": 1024000,
      "tx_bytes": 512000,
      "rx_packets": 1000,
      "tx_packets": 500,
      "status": "up"
    }
  ]
}
```

### Servisler

#### GET /services

İzlenen servislerin listesini döndürür.

**Yanıt:**
```json
[
  {
    "name": "nginx",
    "active": true,
    "enabled": true,
    "version": "1.18.0",
    "status": "running",
    "pid": 1234
  }
]
```

#### GET /services/{service_name}

Belirli bir servisin bilgisini döndürür.

**Parametreler:**
- `service_name` (string): Servis adı

**Yanıt:**
```json
{
  "name": "nginx",
  "active": true,
  "enabled": true,
  "version": "1.18.0",
  "status": "running",
  "pid": 1234,
  "uptime": "2 gün, 3 saat",
  "memory_usage_mb": 25.5,
  "cpu_usage_percent": 0.5
}
```

#### POST /services/{service_name}/restart

Belirli bir servisi yeniden başlatır.

**Parametreler:**
- `service_name` (string): Servis adı

**Yanıt:**
```json
{
  "success": true,
  "message": "nginx servisi başarıyla yeniden başlatıldı",
  "timestamp": "2024-03-19T10:30:00Z"
}
```

### Güvenlik

#### GET /security/status

Güvenlik durumunu döndürür.

**Yanıt:**
```json
{
  "firewall_enabled": true,
  "fail2ban_active": true,
  "open_ports": [22, 80, 443],
  "package_updates": ["nginx", "openssl"],
  "security_updates": 5,
  "last_security_scan": "2024-03-19T10:00:00Z",
  "vulnerabilities": []
}
```

#### GET /security/ports

Açık portları döndürür.

**Yanıt:**
```json
[
  {
    "port": 22,
    "protocol": "tcp",
    "service": "ssh",
    "state": "LISTEN",
    "process": "sshd"
  }
]
```

### Yapılandırma

#### GET /config

Mevcut yapılandırmayı döndürür.

**Yanıt:**
```json
{
  "update_interval": 2,
  "enable_daemon": false,
  ...
}
```

#### PUT /config

Yapılandırmayı günceller.

**İstek Gövdesi:**
```json
{
  "update_interval": 5,
  "log_level": "debug",
  "enable_daemon": true
}
```

**Yanıt:**
```json
{
  "success": true,
  "message": "Yapılandırma başarıyla güncellendi",
  "timestamp": "2024-03-19T10:30:00Z"
}
```

### İzleme Kontrolü

#### POST /monitor/start

Daemon'u başlatır.

**Yanıt:**
```json
{
  "success": true,
  "message": "Daemon başarıyla başlatıldı",
  "pid": 1234,
  "timestamp": "2024-03-19T10:30:00Z"
}
```

#### POST /monitor/stop

Daemon'u durdurur.

**Yanıt:**
```json
{
  "success": true,
  "message": "Daemon başarıyla durduruldu",
  "timestamp": "2024-03-19T10:30:00Z"
}
```

#### GET /monitor/status

Daemon durumunu döndürür.

**Yanıt:**
```json
{
  "running": true,
  "pid": 1234,
  "uptime": "2 gün, 3 saat",
  "last_update": "2024-03-19T10:30:00Z",
  "update_count": 43200
}
```

### Tarihsel Veri

#### GET /history/system

Tarihsel sistem verilerini döndürür.

**Sorgu Parametreleri:**
- `start_time` (string): Başlangıç zamanı (ISO 8601)
- `end_time` (string): Bitiş zamanı (ISO 8601)
- `limit` (integer): Maksimum kayıt sayısı (varsayılan: 100)

**Yanıt:**
```json
{
  "data": [
    {
      "timestamp": "2024-03-19T10:30:00Z",
      "cpu_usage": 45.2,
      "memory_usage": 50.0,
      "disk_usage": 50.0
    }
  ],
  "total": 100,
  "limit": 100
}
```

#### GET /history/alerts

Tarihsel uyarıları döndürür.

**Sorgu Parametreleri:**
- `start_time` (string): Başlangıç zamanı (ISO 8601)
- `end_time` (string): Bitiş zamanı (ISO 8601)
- `severity` (string): Uyarı seviyesi (info, warning, error, critical)
- `limit` (integer): Maksimum kayıt sayısı (varsayılan: 100)

**Yanıt:**
```json
{
  "alerts": [
    {
      "timestamp": "2024-03-19T10:30:00Z",
      "severity": "warning",
      "message": "Yüksek CPU kullanımı algılandı",
      "details": {
        "cpu_usage": 85.5,
        "threshold": 80.0
      }
    }
  ],
  "total": 50,
  "limit": 100
}
```

## Hata Yanıtları

### 400 Geçersiz İstek
```json
{
  "error": "Geçersiz parametre",
  "message": "Sağlanan parametre geçersiz",
  "timestamp": "2024-03-19T10:30:00Z"
}
```

### 401 Yetkisiz
```json
{
  "error": "Yetkisiz",
  "message": "API anahtarı eksik veya hatalı",
  "timestamp": "2024-03-19T10:30:00Z"
}
```

### 404 Bulunamadı
```json
{
  "error": "Bulunamadı",
  "message": "İstenen kaynak bulunamadı",
  "timestamp": "2024-03-19T10:30:00Z"
}
```

### 500 Sunucu Hatası
```json
{
  "error": "Sunucu hatası",
  "message": "Beklenmeyen bir hata oluştu",
  "timestamp": "2024-03-19T10:30:00Z"
}
```

## Oran Sınırlaması

API, kötüye kullanımı önlemek için oran sınırlaması uygular. Varsayılan olarak, API anahtarı başına dakikada 100 istek sınırı vardır.

Aşılırsa 429 yanıtı döner:

```json
{
  "error": "Oran limiti aşıldı",
  "message": "Çok fazla istek",
  "retry_after": 60,
  "timestamp": "2024-03-19T10:30:00Z"
}
```

## WebSocket Desteği

Gerçek zamanlı güncellemeler için WebSocket bağlantısı:

```
ws://localhost:8080/api/v1/ws
```

### WebSocket Olayları

#### system_update
```json
{
  "event": "system_update",
  "data": {
    "timestamp": "2024-03-19T10:30:00Z",
    "cpu_usage": 45.2,
    "memory_usage": 50.0
  }
}
```

#### alert
```json
{
  "event": "alert",
  "data": {
    "timestamp": "2024-03-19T10:30:00Z",
    "severity": "warning",
    "message": "Yüksek CPU kullanımı algılandı"
  }
}
```

## SDK Örnekleri

### Python
```python
import requests

api_key = "api-anahtarınız"
base_url = "http://localhost:8080/api/v1"

headers = {
    "Authorization": f"Bearer {api_key}",
    "Content-Type": "application/json"
}

# Sistem bilgisini al
response = requests.get(f"{base_url}/system/info", headers=headers)
system_info = response.json()
print(f"CPU Kullanımı: {system_info['cpu']['usage_percent']}%")
```

### JavaScript
```javascript
const apiKey = 'api-anahtarınız';
const baseUrl = 'http://localhost:8080/api/v1';

const headers = {
    'Authorization': `Bearer ${apiKey}`,
    'Content-Type': 'application/json'
};

// Sistem bilgisini al
fetch(`${baseUrl}/system/info`, { headers })
    .then(response => response.json())
    .then(data => {
        console.log(`CPU Kullanımı: ${data.cpu.usage_percent}%`);
    });
```

### cURL
```bash
# Sistem bilgisini al
curl -H "Authorization: Bearer api-anahtarınız" \
     -H "Content-Type: application/json" \
     http://localhost:8080/api/v1/system/info

# Daemon başlat
curl -X POST \
     -H "Authorization: Bearer api-anahtarınız" \
     -H "Content-Type: application/json" \
     http://localhost:8080/api/v1/monitor/start
```

## API Versiyonlama

API semantik versiyonlama kullanır. Mevcut sürüm v1'dir. Gelecekteki sürümler `/api/v2`, `/api/v3` gibi yollarla sunulacaktır.

Kırıcı değişiklikler yalnızca ana sürüm güncellemelerinde yapılır ve eski endpointler en az bir ana sürüm boyunca desteklenir. 