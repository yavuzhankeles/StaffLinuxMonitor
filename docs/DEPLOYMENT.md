# Deployment Guide

## Overview

This guide covers deploying StaffLinuxMonitor in various environments, from single-server setups to large-scale production deployments.

## Prerequisites

### System Requirements

- **Operating System**: Linux (Ubuntu 18.04+, CentOS 7+, RHEL 7+, Debian 9+)
- **Architecture**: x86_64, ARM64
- **Memory**: Minimum 512MB RAM, Recommended 2GB+
- **Storage**: Minimum 100MB disk space, Recommended 10GB+
- **Network**: Internet access for updates and API communication

### Software Dependencies

- **Rust Runtime**: 1.70+ (for building from source)
- **System Libraries**: libc, libssl, libcurl
- **System Tools**: systemctl, journalctl (for systemd integration)

## Installation Methods

### Method 1: Binary Installation (Recommended)

#### Step 1: Download Binary

```bash
# Create installation directory
sudo mkdir -p /opt/staffmon
cd /opt/staffmon

# Download latest release
wget https://github.com/forniya/StaffLinuxMonitor/releases/latest/download/staffmon

# Make executable
chmod +x staffmon

# Create symbolic link
sudo ln -sf /opt/staffmon/staffmon /usr/local/bin/staffmon
```

#### Step 2: Create Configuration

```bash
# Create configuration directory
sudo mkdir -p /etc/staffmon

# Create configuration file
sudo tee /etc/staffmon/config.toml > /dev/null <<EOF
[monitoring]
update_interval = 2
enable_daemon = true
enable_foreground = false
enable_json_output = true
enable_api = true

[logging]
level = "info"
file = "/var/log/staffmon.log"
max_size = "100MB"
backup_count = 5
enable_console = false
enable_syslog = true

[api]
enabled = true
host = "127.0.0.1"
port = 8080
api_key = "$(openssl rand -hex 32)"
timeout_seconds = 30
retry_count = 3
rate_limit = 100

[output]
json_file = "/var/log/staffmon_data.json"
enable_compression = true
max_file_size = "1GB"
retention_days = 30

[features]
enable_cpu_monitoring = true
enable_memory_monitoring = true
enable_disk_monitoring = true
enable_network_monitoring = true
enable_service_monitoring = true
enable_security_monitoring = true
enable_process_monitoring = false
enable_hardware_monitoring = true
enable_uptime_monitoring = true
EOF
```

#### Step 3: Create System User

```bash
# Create system user
sudo useradd -r -s /bin/false -d /opt/staffmon staffmon

# Set ownership
sudo chown -R staffmon:staffmon /opt/staffmon
sudo chown -R staffmon:staffmon /etc/staffmon
```

#### Step 4: Create Log Directory

```bash
# Create log directory
sudo mkdir -p /var/log/staffmon
sudo chown staffmon:staffmon /var/log/staffmon
```

### Method 2: Build from Source

#### Step 1: Install Rust

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Install build dependencies
sudo apt-get update
sudo apt-get install -y build-essential pkg-config libssl-dev libcurl4-openssl-dev
```

#### Step 2: Build Project

```bash
# Clone repository
git clone https://github.com/forniya/StaffLinuxMonitor.git
cd StaffLinuxMonitor

# Build release version
cargo build --release

# Install binary
sudo cp target/release/staffmon /usr/local/bin/
sudo chmod +x /usr/local/bin/staffmon
```

## Systemd Service Setup

### Create Service File

```bash
sudo tee /etc/systemd/system/staffmon.service > /dev/null <<EOF
[Unit]
Description=StaffLinuxMonitor System Monitoring Tool
Documentation=https://github.com/forniya/StaffLinuxMonitor
After=network.target

[Service]
Type=simple
User=staffmon
Group=staffmon
ExecStart=/usr/local/bin/staffmon --config /etc/staffmon/config.toml
ExecReload=/bin/kill -HUP \$MAINPID
Restart=always
RestartSec=10
StandardOutput=journal
StandardError=journal
SyslogIdentifier=staffmon

# Security settings
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=/var/log/staffmon /etc/staffmon

# Resource limits
LimitNOFILE=65536
LimitNPROC=4096

[Install]
WantedBy=multi-user.target
EOF
```

### Enable and Start Service

```bash
# Reload systemd
sudo systemctl daemon-reload

# Enable service
sudo systemctl enable staffmon

# Start service
sudo systemctl start staffmon

# Check status
sudo systemctl status staffmon
```

## Docker Deployment

### Dockerfile

```dockerfile
FROM rust:1.70-alpine as builder

# Install build dependencies
RUN apk add --no-cache musl-dev openssl-dev

# Copy source code
WORKDIR /app
COPY . .

# Build the application
RUN cargo build --release

# Runtime stage
FROM alpine:latest

# Install runtime dependencies
RUN apk add --no-cache libc6-compat openssl

# Create user
RUN addgroup -g 1001 -S staffmon && \
    adduser -S staffmon -u 1001

# Copy binary
COPY --from=builder /app/target/release/staffmon /usr/local/bin/
RUN chmod +x /usr/local/bin/staffmon

# Create directories
RUN mkdir -p /etc/staffmon /var/log/staffmon && \
    chown -R staffmon:staffmon /etc/staffmon /var/log/staffmon

# Switch to non-root user
USER staffmon

# Expose API port
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD wget --no-verbose --tries=1 --spider http://localhost:8080/api/v1/health || exit 1

# Run the application
CMD ["/usr/local/bin/staffmon", "--config", "/etc/staffmon/config.toml"]
```

### Docker Compose

```yaml
version: '3.8'

services:
  staffmon:
    build: .
    container_name: staffmon
    restart: unless-stopped
    ports:
      - "8080:8080"
    volumes:
      - ./config.toml:/etc/staffmon/config.toml:ro
      - staffmon_logs:/var/log/staffmon
      - /proc:/host/proc:ro
      - /sys:/host/sys:ro
      - /etc:/host/etc:ro
    environment:
      - RUST_LOG=info
    cap_add:
      - SYS_ADMIN
    security_opt:
      - seccomp:unconfined
    network_mode: host

volumes:
  staffmon_logs:
```

### Run with Docker

```bash
# Build and run
docker-compose up -d

# Check logs
docker-compose logs -f staffmon

# Access API
curl http://localhost:8080/api/v1/system/info
```

## Kubernetes Deployment

### ConfigMap

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: staffmon-config
  namespace: monitoring
data:
  config.toml: |
    [monitoring]
    update_interval = 2
    enable_daemon = false
    enable_foreground = true
    enable_json_output = true
    enable_api = true

    [logging]
    level = "info"
    file = "/var/log/staffmon.log"
    enable_console = true

    [api]
    enabled = true
    host = "0.0.0.0"
    port = 8080
    api_key = "your-api-key-here"

    [output]
    json_file = "/var/log/staffmon_data.json"
    enable_compression = true
    max_file_size = "100MB"
    retention_days = 7

    [features]
    enable_cpu_monitoring = true
    enable_memory_monitoring = true
    enable_disk_monitoring = true
    enable_network_monitoring = true
    enable_service_monitoring = true
    enable_security_monitoring = true
    enable_process_monitoring = false
    enable_hardware_monitoring = true
    enable_uptime_monitoring = true
```

### Deployment

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: staffmon
  namespace: monitoring
  labels:
    app: staffmon
spec:
  replicas: 1
  selector:
    matchLabels:
      app: staffmon
  template:
    metadata:
      labels:
        app: staffmon
    spec:
      serviceAccountName: staffmon-sa
      containers:
      - name: staffmon
        image: staffmon:latest
        imagePullPolicy: Always
        ports:
        - containerPort: 8080
          name: api
        volumeMounts:
        - name: config
          mountPath: /etc/staffmon
          readOnly: true
        - name: logs
          mountPath: /var/log/staffmon
        - name: proc
          mountPath: /host/proc
          readOnly: true
        - name: sys
          mountPath: /host/sys
          readOnly: true
        - name: etc
          mountPath: /host/etc
          readOnly: true
        env:
        - name: RUST_LOG
          value: "info"
        resources:
          requests:
            memory: "128Mi"
            cpu: "100m"
          limits:
            memory: "512Mi"
            cpu: "500m"
        livenessProbe:
          httpGet:
            path: /api/v1/health
            port: 8080
          initialDelaySeconds: 30
          periodSeconds: 30
        readinessProbe:
          httpGet:
            path: /api/v1/health
            port: 8080
          initialDelaySeconds: 5
          periodSeconds: 10
      volumes:
      - name: config
        configMap:
          name: staffmon-config
      - name: logs
        emptyDir: {}
      - name: proc
        hostPath:
          path: /proc
      - name: sys
        hostPath:
          path: /sys
      - name: etc
        hostPath:
          path: /etc
```

### Service

```yaml
apiVersion: v1
kind: Service
metadata:
  name: staffmon-service
  namespace: monitoring
spec:
  selector:
    app: staffmon
  ports:
  - name: api
    port: 8080
    targetPort: 8080
  type: ClusterIP
```

### ServiceAccount

```yaml
apiVersion: v1
kind: ServiceAccount
metadata:
  name: staffmon-sa
  namespace: monitoring
```

### RBAC (Optional)

```yaml
apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRole
metadata:
  name: staffmon-role
rules:
- apiGroups: [""]
  resources: ["nodes", "pods", "services"]
  verbs: ["get", "list", "watch"]
- apiGroups: [""]
  resources: ["nodes/proxy"]
  verbs: ["get", "list", "watch"]

---
apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRoleBinding
metadata:
  name: staffmon-role-binding
roleRef:
  apiGroup: rbac.authorization.k8s.io
  kind: ClusterRole
  name: staffmon-role
subjects:
- kind: ServiceAccount
  name: staffmon-sa
  namespace: monitoring
```

## Production Deployment

### High Availability Setup

#### Load Balancer Configuration

```nginx
upstream staffmon_backend {
    server 192.168.1.10:8080;
    server 192.168.1.11:8080;
    server 192.168.1.12:8080;
}

server {
    listen 80;
    server_name staffmon.yourdomain.com;

    location / {
        proxy_pass http://staffmon_backend;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

#### SSL/TLS Configuration

```bash
# Generate SSL certificate
sudo certbot certonly --standalone -d staffmon.yourdomain.com

# Update configuration
sudo tee -a /etc/staffmon/config.toml <<EOF

[api]
enable_ssl = true
ssl_cert = "/etc/letsencrypt/live/staffmon.yourdomain.com/fullchain.pem"
ssl_key = "/etc/letsencrypt/live/staffmon.yourdomain.com/privkey.pem"
EOF
```

### Monitoring and Alerting

#### Prometheus Configuration

```yaml
# prometheus.yml
global:
  scrape_interval: 15s

scrape_configs:
  - job_name: 'staffmon'
    static_configs:
      - targets: ['localhost:8080']
    metrics_path: '/api/v1/metrics'
    scheme: 'http'
    scrape_interval: 30s
```

#### Grafana Dashboard

```json
{
  "dashboard": {
    "title": "StaffLinuxMonitor Dashboard",
    "panels": [
      {
        "title": "CPU Usage",
        "type": "graph",
        "targets": [
          {
            "expr": "staffmon_cpu_usage_percent",
            "legendFormat": "CPU Usage %"
          }
        ]
      },
      {
        "title": "Memory Usage",
        "type": "graph",
        "targets": [
          {
            "expr": "staffmon_memory_usage_percent",
            "legendFormat": "Memory Usage %"
          }
        ]
      }
    ]
  }
}
```

### Backup and Recovery

#### Backup Script

```bash
#!/bin/bash
# backup-staffmon.sh

BACKUP_DIR="/backup/staffmon"
DATE=$(date +%Y%m%d_%H%M%S)

# Create backup directory
mkdir -p "$BACKUP_DIR"

# Backup configuration
cp -r /etc/staffmon "$BACKUP_DIR/config_$DATE"

# Backup logs
tar -czf "$BACKUP_DIR/logs_$DATE.tar.gz" /var/log/staffmon

# Backup data
cp /var/log/staffmon_data.json "$BACKUP_DIR/data_$DATE.json"

# Clean old backups (keep last 7 days)
find "$BACKUP_DIR" -type d -mtime +7 -exec rm -rf {} \;
find "$BACKUP_DIR" -name "*.tar.gz" -mtime +7 -delete
find "$BACKUP_DIR" -name "*.json" -mtime +7 -delete

echo "Backup completed: $BACKUP_DIR"
```

#### Cron Job

```bash
# Add to crontab
0 2 * * * /usr/local/bin/backup-staffmon.sh
```

## Security Considerations

### Firewall Configuration

```bash
# UFW (Ubuntu)
sudo ufw allow 8080/tcp
sudo ufw allow from 192.168.1.0/24 to any port 8080

# iptables
sudo iptables -A INPUT -p tcp --dport 8080 -s 192.168.1.0/24 -j ACCEPT
sudo iptables -A INPUT -p tcp --dport 8080 -j DROP
```

### API Security

```toml
# Enhanced security configuration
[api]
enabled = true
host = "127.0.0.1"  # Bind to localhost only
port = 8080
api_key = "your-very-secure-api-key"
enable_ssl = true
enable_cors = false
rate_limit = 60  # Reduce rate limit

[security_settings]
enable_encryption = true
encryption_key = "your-32-byte-encryption-key"
enable_audit_log = true
restrict_api_access = true
allowed_ips = ["127.0.0.1", "192.168.1.0/24"]
```

### SELinux Configuration (RHEL/CentOS)

```bash
# Create SELinux policy
sudo tee staffmon.te <<EOF
module staffmon 1.0;

require {
    type staffmon_t;
    type staffmon_exec_t;
    type var_log_t;
    type etc_t;
    class file { read write execute };
    class dir { read write };
}

allow staffmon_t var_log_t:file { read write };
allow staffmon_t etc_t:file { read };
EOF

# Compile and load policy
sudo make -f /usr/share/selinux/devel/Makefile staffmon.pp
sudo semodule -i staffmon.pp
```

## Troubleshooting

### Common Issues

#### Service Won't Start

```bash
# Check service status
sudo systemctl status staffmon

# Check logs
sudo journalctl -u staffmon -f

# Check configuration
sudo staffmon --validate-config /etc/staffmon/config.toml

# Check permissions
ls -la /opt/staffmon/
ls -la /etc/staffmon/
ls -la /var/log/staffmon/
```

#### API Not Accessible

```bash
# Check if service is listening
sudo netstat -tlnp | grep 8080

# Test API locally
curl -H "Authorization: Bearer your-api-key" http://127.0.0.1:8080/api/v1/system/info

# Check firewall
sudo ufw status
sudo iptables -L
```

#### High Resource Usage

```bash
# Check process resources
ps aux | grep staffmon
top -p $(pgrep staffmon)

# Check log file sizes
du -sh /var/log/staffmon/*

# Optimize configuration
# Reduce update_interval
# Disable unused features
# Enable compression
```

### Performance Tuning

#### System Limits

```bash
# Increase file descriptor limits
echo "staffmon soft nofile 65536" | sudo tee -a /etc/security/limits.conf
echo "staffmon hard nofile 65536" | sudo tee -a /etc/security/limits.conf

# Increase process limits
echo "staffmon soft nproc 4096" | sudo tee -a /etc/security/limits.conf
echo "staffmon hard nproc 4096" | sudo tee -a /etc/security/limits.conf
```

#### Kernel Parameters

```bash
# Optimize for monitoring
echo "vm.swappiness=10" | sudo tee -a /etc/sysctl.conf
echo "vm.dirty_ratio=15" | sudo tee -a /etc/sysctl.conf
echo "vm.dirty_background_ratio=5" | sudo tee -a /etc/sysctl.conf

# Apply changes
sudo sysctl -p
```

## Maintenance

### Regular Maintenance Tasks

```bash
#!/bin/bash
# maintenance.sh

# Rotate logs
sudo logrotate /etc/logrotate.d/staffmon

# Clean old data
find /var/log/staffmon -name "*.json" -mtime +30 -delete

# Update system packages
sudo apt-get update && sudo apt-get upgrade -y

# Restart service
sudo systemctl restart staffmon

# Check service health
curl -f http://localhost:8080/api/v1/health || echo "Service unhealthy"
```

### Monitoring Script

```bash
#!/bin/bash
# monitor-staffmon.sh

# Check if service is running
if ! systemctl is-active --quiet staffmon; then
    echo "StaffLinuxMonitor service is down!"
    systemctl restart staffmon
    # Send alert
fi

# Check API health
if ! curl -f -s http://localhost:8080/api/v1/health > /dev/null; then
    echo "StaffLinuxMonitor API is not responding!"
    # Send alert
fi

# Check disk space
DISK_USAGE=$(df /var/log/staffmon | tail -1 | awk '{print $5}' | sed 's/%//')
if [ "$DISK_USAGE" -gt 90 ]; then
    echo "Disk usage is high: ${DISK_USAGE}%"
    # Send alert
fi
```

## Support and Resources

### Documentation
- [API Documentation](API.md)
- [Configuration Guide](CONFIGURATION.md)
- [Troubleshooting Guide](TROUBLESHOOTING.md)

### Community
- [GitHub Issues](https://github.com/forniya/StaffLinuxMonitor/issues)
- [GitHub Discussions](https://github.com/forniya/StaffLinuxMonitor/discussions)
- [Wiki](https://github.com/forniya/StaffLinuxMonitor/wiki)

### Monitoring Tools
- [Prometheus](https://prometheus.io/)
- [Grafana](https://grafana.com/)
- [AlertManager](https://prometheus.io/docs/alerting/latest/alertmanager/) 