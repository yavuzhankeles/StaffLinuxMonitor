# API Documentation

## Overview

StaffLinuxMonitor provides a comprehensive REST API for accessing system monitoring data. The API is designed to be simple, efficient, and secure, supporting both JSON responses and real-time data streaming.

## Base URL

```
http://localhost:8080/api/v1
```

## Authentication

All API endpoints require authentication using an API key. Include the key in the request header:

```bash
Authorization: Bearer your-api-key-here
```

### API Key Configuration

Set your API key in the configuration file:

```toml
[api]
api_key = "your-secure-api-key-here"
```

## Endpoints

### System Information

#### GET /system/info

Returns general system information.

**Response:**
```json
{
  "hostname": "server01",
  "os": "Ubuntu 20.04.3 LTS",
  "kernel": "5.4.0-74-generic",
  "architecture": "x86_64",
  "uptime": 1234567,
  "uptime_formatted": "14 days, 6 hours, 56 minutes",
  "boot_time": "2024-01-15T10:30:00Z",
  "timezone": "UTC",
  "last_update": "2024-01-29T17:26:45Z"
}
```

#### GET /system/status

Returns overall system health status.

**Response:**
```json
{
  "status": "healthy",
  "timestamp": "2024-01-29T17:26:45Z",
  "version": "1.0.2",
  "uptime": 1234567,
  "monitoring_active": true,
  "api_version": "v1"
}
```

### CPU Monitoring

#### GET /cpu/info

Returns detailed CPU information.

**Response:**
```json
{
  "model": "Intel(R) Core(TM) i7-8700K CPU @ 3.70GHz",
  "cores": 6,
  "threads": 12,
  "architecture": "x86_64",
  "frequency": {
    "current": 3700,
    "min": 800,
    "max": 4700
  },
  "cache": {
    "l1": 384,
    "l2": 1536,
    "l3": 12288
  }
}
```

#### GET /cpu/usage

Returns current CPU usage statistics.

**Response:**
```json
{
  "overall": {
    "usage_percent": 45.2,
    "load_average": {
      "1min": 1.25,
      "5min": 1.15,
      "15min": 1.05
    }
  },
  "per_core": [
    {
      "core": 0,
      "usage_percent": 52.1,
      "frequency": 3700
    },
    {
      "core": 1,
      "usage_percent": 38.7,
      "frequency": 3600
    }
  ],
  "temperature": {
    "current": 65.0,
    "unit": "celsius",
    "critical": 95.0
  },
  "timestamp": "2024-01-29T17:26:45Z"
}
```

#### GET /cpu/history

Returns CPU usage history (if enabled).

**Query Parameters:**
- `duration`: Time range in minutes (default: 60)
- `interval`: Data point interval in seconds (default: 30)

**Response:**
```json
{
  "data": [
    {
      "timestamp": "2024-01-29T17:25:00Z",
      "usage_percent": 42.1,
      "temperature": 63.0
    },
    {
      "timestamp": "2024-01-29T17:25:30Z",
      "usage_percent": 45.8,
      "temperature": 65.0
    }
  ],
  "duration": 60,
  "interval": 30
}
```

### Memory Monitoring

#### GET /memory/info

Returns memory hardware information.

**Response:**
```json
{
  "total": 16777216,
  "total_formatted": "16.0 GB",
  "type": "DDR4",
  "speed": 3200,
  "channels": 2,
  "slots": 4,
  "used_slots": 2
}
```

#### GET /memory/usage

Returns current memory usage.

**Response:**
```json
{
  "total": 16777216,
  "available": 8388608,
  "used": 8388608,
  "free": 4194304,
  "cached": 2097152,
  "buffers": 1048576,
  "swap": {
    "total": 2097152,
    "used": 524288,
    "free": 1572864,
    "usage_percent": 25.0
  },
  "usage_percent": 50.0,
  "timestamp": "2024-01-29T17:26:45Z"
}
```

### Disk Monitoring

#### GET /disk/info

Returns disk hardware information.

**Response:**
```json
{
  "disks": [
    {
      "device": "/dev/sda",
      "model": "Samsung SSD 860 EVO 1TB",
      "size": 1000204886016,
      "size_formatted": "1.0 TB",
      "type": "ssd",
      "interface": "SATA",
      "temperature": 35.0
    }
  ]
}
```

#### GET /disk/usage

Returns current disk usage.

**Response:**
```json
{
  "partitions": [
    {
      "device": "/dev/sda1",
      "mountpoint": "/",
      "filesystem": "ext4",
      "total": 1000204886016,
      "used": 500102443008,
      "available": 500102443008,
      "usage_percent": 50.0,
      "inodes": {
        "total": 62500000,
        "used": 31250000,
        "available": 31250000,
        "usage_percent": 50.0
      }
    }
  ],
  "io_stats": {
    "read_bytes": 1234567890,
    "write_bytes": 987654321,
    "read_operations": 12345,
    "write_operations": 9876,
    "read_time": 1234,
    "write_time": 5678
  },
  "timestamp": "2024-01-29T17:26:45Z"
}
```

### Network Monitoring

#### GET /network/interfaces

Returns network interface information.

**Response:**
```json
{
  "interfaces": [
    {
      "name": "eth0",
      "type": "ethernet",
      "mac_address": "00:15:5d:01:ca:05",
      "ip_addresses": [
        {
          "address": "192.168.1.100",
          "netmask": "255.255.255.0",
          "family": "inet"
        }
      ],
      "status": "up",
      "speed": 1000,
      "duplex": "full"
    }
  ]
}
```

#### GET /network/usage

Returns current network usage statistics.

**Response:**
```json
{
  "interfaces": [
    {
      "name": "eth0",
      "bytes_sent": 1234567890,
      "bytes_received": 9876543210,
      "packets_sent": 12345,
      "packets_received": 98765,
      "errors_in": 0,
      "errors_out": 0,
      "dropped_in": 0,
      "dropped_out": 0,
      "bandwidth_usage": {
        "in": 1024000,
        "out": 512000
      }
    }
  ],
  "timestamp": "2024-01-29T17:26:45Z"
}
```

### Service Monitoring

#### GET /services/status

Returns system service status.

**Response:**
```json
{
  "services": [
    {
      "name": "nginx",
      "status": "active",
      "state": "running",
      "pid": 1234,
      "memory_usage": 20480,
      "cpu_usage": 0.5,
      "uptime": 3600,
      "restart_count": 1
    },
    {
      "name": "mysql",
      "status": "active",
      "state": "running",
      "pid": 5678,
      "memory_usage": 1048576,
      "cpu_usage": 2.1,
      "uptime": 7200,
      "restart_count": 0
    }
  ],
  "total_services": 25,
  "active_services": 23,
  "failed_services": 2,
  "timestamp": "2024-01-29T17:26:45Z"
}
```

#### GET /services/{service_name}

Returns detailed information about a specific service.

**Response:**
```json
{
  "name": "nginx",
  "status": "active",
  "state": "running",
  "pid": 1234,
  "memory_usage": 20480,
  "cpu_usage": 0.5,
  "uptime": 3600,
  "restart_count": 1,
  "description": "A high performance web server",
  "dependencies": ["systemd"],
  "logs": {
    "recent_entries": [
      "2024-01-29 17:25:00 [notice] 1234#0: start worker processes",
      "2024-01-29 17:25:01 [notice] 1234#0: start worker process 1235"
    ]
  },
  "timestamp": "2024-01-29T17:26:45Z"
}
```

### Security Monitoring

#### GET /security/status

Returns security-related information.

**Response:**
```json
{
  "firewall": {
    "status": "active",
    "rules_count": 25,
    "blocked_connections": 150
  },
  "fail2ban": {
    "status": "active",
    "jails": [
      {
        "name": "sshd",
        "status": "enabled",
        "banned_ips": 5,
        "failures": 25
      }
    ]
  },
  "package_updates": {
    "available": 15,
    "security_updates": 3,
    "last_update_check": "2024-01-29T10:00:00Z"
  },
  "ssh": {
    "active_sessions": 2,
    "failed_attempts": 8,
    "last_login": "2024-01-29T16:30:00Z"
  },
  "timestamp": "2024-01-29T17:26:45Z"
}
```

### Hardware Monitoring

#### GET /hardware/info

Returns hardware information.

**Response:**
```json
{
  "motherboard": {
    "manufacturer": "ASUS",
    "model": "ROG STRIX Z370-E GAMING",
    "bios_version": "1401"
  },
  "gpu": [
    {
      "name": "NVIDIA GeForce GTX 1080",
      "memory": 8589934592,
      "driver_version": "470.82.01",
      "temperature": 65.0,
      "usage_percent": 15.0
    }
  ],
  "storage": [
    {
      "device": "/dev/sda",
      "type": "ssd",
      "model": "Samsung SSD 860 EVO 1TB",
      "size": 1000204886016
    }
  ],
  "timestamp": "2024-01-29T17:26:45Z"
}
```

### Comprehensive Data

#### GET /system/all

Returns all system monitoring data in a single request.

**Response:**
```json
{
  "system": {
    "info": { /* system info */ },
    "status": { /* system status */ }
  },
  "cpu": {
    "info": { /* CPU info */ },
    "usage": { /* CPU usage */ }
  },
  "memory": {
    "info": { /* memory info */ },
    "usage": { /* memory usage */ }
  },
  "disk": {
    "info": { /* disk info */ },
    "usage": { /* disk usage */ }
  },
  "network": {
    "interfaces": { /* network interfaces */ },
    "usage": { /* network usage */ }
  },
  "services": { /* services status */ },
  "security": { /* security status */ },
  "hardware": { /* hardware info */ },
  "timestamp": "2024-01-29T17:26:45Z"
}
```

## Error Handling

### Error Response Format

```json
{
  "error": {
    "code": 404,
    "message": "Resource not found",
    "details": "The requested endpoint does not exist"
  },
  "timestamp": "2024-01-29T17:26:45Z"
}
```

### Common Error Codes

- `400` - Bad Request
- `401` - Unauthorized (invalid API key)
- `403` - Forbidden
- `404` - Not Found
- `429` - Too Many Requests (rate limit exceeded)
- `500` - Internal Server Error
- `503` - Service Unavailable

## Rate Limiting

API requests are rate-limited to prevent abuse. Default limits:

- **Authenticated requests**: 100 requests per minute
- **Unauthenticated requests**: 10 requests per minute

Rate limit headers are included in responses:

```
X-RateLimit-Limit: 100
X-RateLimit-Remaining: 95
X-RateLimit-Reset: 1643476800
```

## CORS Support

For web applications, CORS can be enabled in the configuration:

```toml
[api]
enable_cors = true
```

## SSL/TLS Support

For production deployments, enable SSL/TLS:

```toml
[api]
enable_ssl = true
ssl_cert = "/path/to/certificate.pem"
ssl_key = "/path/to/private-key.pem"
```

## Examples

### Using curl

```bash
# Get system information
curl -H "Authorization: Bearer your-api-key" \
     http://localhost:8080/api/v1/system/info

# Get CPU usage
curl -H "Authorization: Bearer your-api-key" \
     http://localhost:8080/api/v1/cpu/usage

# Get all system data
curl -H "Authorization: Bearer your-api-key" \
     http://localhost:8080/api/v1/system/all
```

### Using Python

```python
import requests

API_BASE = "http://localhost:8080/api/v1"
API_KEY = "your-api-key"

headers = {"Authorization": f"Bearer {API_KEY}"}

# Get system info
response = requests.get(f"{API_BASE}/system/info", headers=headers)
system_info = response.json()

# Get CPU usage
response = requests.get(f"{API_BASE}/cpu/usage", headers=headers)
cpu_usage = response.json()

print(f"CPU Usage: {cpu_usage['overall']['usage_percent']}%")
```

### Using JavaScript

```javascript
const API_BASE = "http://localhost:8080/api/v1";
const API_KEY = "your-api-key";

const headers = {
  "Authorization": `Bearer ${API_KEY}`,
  "Content-Type": "application/json"
};

// Get system information
fetch(`${API_BASE}/system/info`, { headers })
  .then(response => response.json())
  .then(data => {
    console.log("System Info:", data);
  });

// Get CPU usage
fetch(`${API_BASE}/cpu/usage`, { headers })
  .then(response => response.json())
  .then(data => {
    console.log("CPU Usage:", data.overall.usage_percent + "%");
  });
```

## Health Check

#### GET /health

Returns API health status (no authentication required).

**Response:**
```json
{
  "status": "healthy",
  "timestamp": "2024-01-29T17:26:45Z",
  "version": "1.0.2",
  "uptime": 1234567
}
```

## Metrics Endpoint

#### GET /metrics

Returns Prometheus-compatible metrics (no authentication required).

**Response:**
```
# HELP staffmon_cpu_usage_percent CPU usage percentage
# TYPE staffmon_cpu_usage_percent gauge
staffmon_cpu_usage_percent 45.2

# HELP staffmon_memory_usage_percent Memory usage percentage
# TYPE staffmon_memory_usage_percent gauge
staffmon_memory_usage_percent 50.0

# HELP staffmon_disk_usage_percent Disk usage percentage
# TYPE staffmon_disk_usage_percent gauge
staffmon_disk_usage_percent{device="/dev/sda1"} 50.0
```

## WebSocket Support

For real-time updates, WebSocket connections are supported:

```javascript
const ws = new WebSocket("ws://localhost:8080/api/v1/ws");

ws.onmessage = function(event) {
  const data = JSON.parse(event.data);
  console.log("Real-time update:", data);
};

// Subscribe to CPU updates
ws.send(JSON.stringify({
  "action": "subscribe",
  "topic": "cpu"
}));
```

## API Versioning

The API uses semantic versioning. Current version is v1. Future versions will be available at `/api/v2`, `/api/v3`, etc.

## Support

For API-related questions and issues:

- [GitHub Issues](https://github.com/forniya/StaffLinuxMonitor/issues)
- [Documentation](https://github.com/forniya/StaffLinuxMonitor/wiki)
- [Configuration Guide](CONFIGURATION_EN.md) 