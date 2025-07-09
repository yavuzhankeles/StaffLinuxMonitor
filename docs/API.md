# API Documentation

## Overview

StaffLinuxMonitor provides a REST API for remote monitoring and data collection. The API allows you to retrieve system information, configure monitoring settings, and manage the monitoring daemon.

## Base URL

```
http://localhost:8080/api/v1
```

## Authentication

All API requests require authentication using an API key. Include the key in the request headers:

```
Authorization: Bearer your-api-key-here
```

## Endpoints

### System Information

#### GET /system/info

Retrieve comprehensive system information.

**Response:**
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

#### GET /system/cpu

Get CPU-specific information.

**Response:**
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

Get memory usage information.

**Response:**
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

Get disk usage information.

**Response:**
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

Get network interface information.

**Response:**
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

### Services

#### GET /services

Get list of monitored services.

**Response:**
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

Get specific service information.

**Parameters:**
- `service_name` (string): Name of the service

**Response:**
```json
{
  "name": "nginx",
  "active": true,
  "enabled": true,
  "version": "1.18.0",
  "status": "running",
  "pid": 1234,
  "uptime": "2 days, 3 hours",
  "memory_usage_mb": 25.5,
  "cpu_usage_percent": 0.5
}
```

#### POST /services/{service_name}/restart

Restart a specific service.

**Parameters:**
- `service_name` (string): Name of the service

**Response:**
```json
{
  "success": true,
  "message": "Service nginx restarted successfully",
  "timestamp": "2024-03-19T10:30:00Z"
}
```

### Security

#### GET /security/status

Get security status information.

**Response:**
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

Get open ports information.

**Response:**
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

### Configuration

#### GET /config

Get current configuration.

**Response:**
```json
{
  "update_interval": 2,
  "enable_daemon": false,
  "log_level": "info",
  "log_file": "/var/log/staffmon.log",
  "pid_file": "/var/run/staffmon.pid",
  "json_output": true,
  "json_file": "/var/log/staffmon_data.json",
  "api": {
    "base_url": "http://localhost:8080",
    "timeout_seconds": 30,
    "retry_count": 3,
    "rate_limit": 100
  }
}
```

#### PUT /config

Update configuration.

**Request Body:**
```json
{
  "update_interval": 5,
  "log_level": "debug",
  "enable_daemon": true
}
```

**Response:**
```json
{
  "success": true,
  "message": "Configuration updated successfully",
  "timestamp": "2024-03-19T10:30:00Z"
}
```

### Monitoring Control

#### POST /monitor/start

Start the monitoring daemon.

**Response:**
```json
{
  "success": true,
  "message": "Monitoring daemon started successfully",
  "pid": 1234,
  "timestamp": "2024-03-19T10:30:00Z"
}
```

#### POST /monitor/stop

Stop the monitoring daemon.

**Response:**
```json
{
  "success": true,
  "message": "Monitoring daemon stopped successfully",
  "timestamp": "2024-03-19T10:30:00Z"
}
```

#### GET /monitor/status

Get monitoring daemon status.

**Response:**
```json
{
  "running": true,
  "pid": 1234,
  "uptime": "2 days, 3 hours",
  "last_update": "2024-03-19T10:30:00Z",
  "update_count": 43200
}
```

### Historical Data

#### GET /history/system

Get historical system data.

**Query Parameters:**
- `start_time` (string): Start time in ISO 8601 format
- `end_time` (string): End time in ISO 8601 format
- `limit` (integer): Maximum number of records (default: 100)

**Response:**
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

Get historical alerts.

**Query Parameters:**
- `start_time` (string): Start time in ISO 8601 format
- `end_time` (string): End time in ISO 8601 format
- `severity` (string): Alert severity (info, warning, error, critical)
- `limit` (integer): Maximum number of records (default: 100)

**Response:**
```json
{
  "alerts": [
    {
      "timestamp": "2024-03-19T10:30:00Z",
      "severity": "warning",
      "message": "High CPU usage detected",
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

## Error Responses

### 400 Bad Request
```json
{
  "error": "Invalid parameter",
  "message": "The provided parameter is invalid",
  "timestamp": "2024-03-19T10:30:00Z"
}
```

### 401 Unauthorized
```json
{
  "error": "Unauthorized",
  "message": "Invalid or missing API key",
  "timestamp": "2024-03-19T10:30:00Z"
}
```

### 404 Not Found
```json
{
  "error": "Not found",
  "message": "The requested resource was not found",
  "timestamp": "2024-03-19T10:30:00Z"
}
```

### 500 Internal Server Error
```json
{
  "error": "Internal server error",
  "message": "An unexpected error occurred",
  "timestamp": "2024-03-19T10:30:00Z"
}
```

## Rate Limiting

The API implements rate limiting to prevent abuse. By default, the limit is 100 requests per minute per API key.

When rate limited, you'll receive a 429 response:

```json
{
  "error": "Rate limit exceeded",
  "message": "Too many requests",
  "retry_after": 60,
  "timestamp": "2024-03-19T10:30:00Z"
}
```

## WebSocket Support

For real-time updates, the API also supports WebSocket connections:

```
ws://localhost:8080/api/v1/ws
```

### WebSocket Events

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
    "message": "High CPU usage detected"
  }
}
```

## SDK Examples

### Python
```python
import requests

api_key = "your-api-key-here"
base_url = "http://localhost:8080/api/v1"

headers = {
    "Authorization": f"Bearer {api_key}",
    "Content-Type": "application/json"
}

# Get system info
response = requests.get(f"{base_url}/system/info", headers=headers)
system_info = response.json()
print(f"CPU Usage: {system_info['cpu']['usage_percent']}%")
```

### JavaScript
```javascript
const apiKey = 'your-api-key-here';
const baseUrl = 'http://localhost:8080/api/v1';

const headers = {
    'Authorization': `Bearer ${apiKey}`,
    'Content-Type': 'application/json'
};

// Get system info
fetch(`${baseUrl}/system/info`, { headers })
    .then(response => response.json())
    .then(data => {
        console.log(`CPU Usage: ${data.cpu.usage_percent}%`);
    });
```

### cURL
```bash
# Get system information
curl -H "Authorization: Bearer your-api-key-here" \
     -H "Content-Type: application/json" \
     http://localhost:8080/api/v1/system/info

# Start monitoring daemon
curl -X POST \
     -H "Authorization: Bearer your-api-key-here" \
     -H "Content-Type: application/json" \
     http://localhost:8080/api/v1/monitor/start
```

## API Versioning

The API uses semantic versioning. The current version is v1. Future versions will be available at `/api/v2`, `/api/v3`, etc.

Breaking changes will only be introduced in major version updates, and deprecated endpoints will be maintained for at least one major version cycle. 