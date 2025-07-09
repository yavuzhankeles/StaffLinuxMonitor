# Configuration Guide

## Overview

StaffLinuxMonitor uses TOML (Tom's Obvious, Minimal Language) for configuration. The configuration file allows you to customize monitoring behavior, API settings, logging, and output formats.

## Configuration File Location

The tool looks for configuration files in the following order:

1. Command-line specified file (`--config` option)
2. `./config.toml` (current directory)
3. `~/.config/staffmon/config.toml` (user config directory)
4. `/etc/staffmon/config.toml` (system config directory)

## Configuration Structure

### Basic Configuration

```toml
# StaffLinuxMonitor Configuration File
# Version: 1.0.2

# Monitoring Settings
[monitoring]
update_interval = 2                    # Update interval in seconds (1-60)
enable_daemon = false                  # Run as daemon process
enable_foreground = true               # Show output in foreground
enable_json_output = true              # Save data to JSON file
enable_api = true                      # Enable REST API server

# Logging Configuration
[logging]
level = "info"                         # Log level: trace, debug, info, warn, error
file = "/var/log/staffmon.log"         # Log file path
max_size = "100MB"                     # Maximum log file size
backup_count = 5                       # Number of backup files to keep
enable_console = true                  # Output logs to console
enable_syslog = false                  # Output logs to syslog

# API Configuration
[api]
enabled = true                         # Enable API server
host = "127.0.0.1"                    # API server host
port = 8080                           # API server port
api_key = "your-secure-api-key-here"  # API authentication key
timeout_seconds = 30                  # Request timeout
retry_count = 3                       # Number of retries
rate_limit = 100                      # Requests per minute
enable_cors = false                   # Enable CORS for web clients
enable_ssl = false                    # Enable HTTPS
ssl_cert = "/path/to/cert.pem"        # SSL certificate path
ssl_key = "/path/to/key.pem"          # SSL private key path

# Output Configuration
[output]
json_file = "/var/log/staffmon_data.json"  # JSON output file path
csv_file = ""                              # CSV output file path (optional)
prometheus_file = ""                        # Prometheus metrics file (optional)
enable_compression = false                  # Compress output files
max_file_size = "1GB"                      # Maximum output file size
retention_days = 30                        # Data retention period

# Monitoring Features
[features]
enable_cpu_monitoring = true           # Monitor CPU usage and temperature
enable_memory_monitoring = true        # Monitor memory usage
enable_disk_monitoring = true          # Monitor disk usage
enable_network_monitoring = true       # Monitor network interfaces
enable_service_monitoring = true       # Monitor system services
enable_security_monitoring = true      # Monitor security status
enable_process_monitoring = false      # Monitor process list (resource intensive)
enable_hardware_monitoring = true      # Monitor hardware information
enable_uptime_monitoring = true        # Monitor system uptime

# Service Monitoring
[services]
# List of services to monitor (empty = monitor all)
monitored_services = [
    "nginx",
    "apache2",
    "mysql",
    "postgresql",
    "redis",
    "docker"
]

# Services to exclude from monitoring
excluded_services = [
    "systemd-timesyncd",
    "systemd-resolved"
]

# Security Monitoring
[security]
enable_firewall_check = true           # Check firewall status
enable_fail2ban_check = true           # Check fail2ban status
enable_package_updates = true          # Check for package updates
enable_port_scanning = false           # Scan open ports (requires root)
enable_vulnerability_scan = false      # Basic vulnerability scanning
check_ssh_logins = true                # Monitor SSH login attempts
check_sudo_usage = true                # Monitor sudo usage

# Alerting Configuration
[alerts]
enabled = false                        # Enable alerting system
cpu_threshold = 80.0                   # CPU usage threshold (%)
memory_threshold = 85.0                # Memory usage threshold (%)
disk_threshold = 90.0                  # Disk usage threshold (%)
network_threshold = 1000000            # Network usage threshold (bytes/s)

# Alert Channels
[alerts.email]
enabled = false                        # Enable email alerts
smtp_server = "smtp.gmail.com"         # SMTP server
smtp_port = 587                        # SMTP port
username = "your-email@gmail.com"      # SMTP username
password = "your-app-password"         # SMTP password
from_address = "alerts@yourdomain.com" # From email address
to_addresses = [                       # Recipient email addresses
    "admin@yourdomain.com",
    "ops@yourdomain.com"
]

[alerts.slack]
enabled = false                        # Enable Slack alerts
webhook_url = "https://hooks.slack.com/services/..."  # Slack webhook URL
channel = "#alerts"                    # Slack channel
username = "StaffLinuxMonitor"         # Bot username

[alerts.telegram]
enabled = false                        # Enable Telegram alerts
bot_token = "your-bot-token"           # Telegram bot token
chat_id = "your-chat-id"               # Telegram chat ID

# Database Configuration (Future Feature)
[database]
enabled = false                        # Enable database storage
type = "sqlite"                        # Database type: sqlite, postgresql, mysql
connection_string = "staffmon.db"      # Database connection string
max_connections = 10                   # Maximum database connections
enable_migrations = true               # Enable automatic migrations

# External Integrations
[integrations]
# Prometheus Integration
[integrations.prometheus]
enabled = false                        # Enable Prometheus metrics
metrics_path = "/metrics"              # Metrics endpoint path
enable_histograms = true               # Enable histogram metrics

# Grafana Integration
[integrations.grafana]
enabled = false                        # Enable Grafana integration
api_url = "http://localhost:3000"      # Grafana API URL
api_key = "your-grafana-api-key"       # Grafana API key
dashboard_uid = "staffmon"             # Dashboard UID

# Elasticsearch Integration
[integrations.elasticsearch]
enabled = false                        # Enable Elasticsearch integration
url = "http://localhost:9200"          # Elasticsearch URL
index_prefix = "staffmon"              # Index name prefix
username = ""                          # Elasticsearch username
password = ""                          # Elasticsearch password

# Performance Tuning
[performance]
max_threads = 4                        # Maximum number of worker threads
buffer_size = 8192                     # I/O buffer size
enable_caching = true                  # Enable metric caching
cache_ttl = 60                         # Cache TTL in seconds
enable_compression = false             # Enable response compression

# Security Settings
[security_settings]
enable_encryption = false              # Enable data encryption
encryption_key = ""                    # Encryption key (32 bytes)
enable_audit_log = false               # Enable audit logging
audit_log_file = "/var/log/staffmon_audit.log"  # Audit log file
restrict_api_access = false            # Restrict API to localhost only
allowed_ips = [                        # Allowed IP addresses for API access
    "127.0.0.1",
    "192.168.1.0/24"
]

# Custom Metrics
[custom_metrics]
enabled = false                        # Enable custom metric collection
# Define custom metrics here
# Example:
# [custom_metrics.metrics]
# disk_io = "cat /proc/diskstats"
# custom_process_count = "ps aux | wc -l"
```

## Environment Variables

You can also configure StaffLinuxMonitor using environment variables. Environment variables take precedence over configuration file settings.

### Available Environment Variables

```bash
# Monitoring Settings
STAFFMON_UPDATE_INTERVAL=2
STAFFMON_ENABLE_DAEMON=false
STAFFMON_ENABLE_FOREGROUND=true
STAFFMON_ENABLE_JSON_OUTPUT=true
STAFFMON_ENABLE_API=true

# Logging
STAFFMON_LOG_LEVEL=info
STAFFMON_LOG_FILE=/var/log/staffmon.log
STAFFMON_LOG_MAX_SIZE=100MB
STAFFMON_LOG_BACKUP_COUNT=5
STAFFMON_LOG_ENABLE_CONSOLE=true
STAFFMON_LOG_ENABLE_SYSLOG=false

# API Settings
STAFFMON_API_ENABLED=true
STAFFMON_API_HOST=127.0.0.1
STAFFMON_API_PORT=8080
STAFFMON_API_KEY=your-secure-api-key-here
STAFFMON_API_TIMEOUT=30
STAFFMON_API_RETRY_COUNT=3
STAFFMON_API_RATE_LIMIT=100

# Output Settings
STAFFMON_OUTPUT_JSON_FILE=/var/log/staffmon_data.json
STAFFMON_OUTPUT_CSV_FILE=
STAFFMON_OUTPUT_PROMETHEUS_FILE=
STAFFMON_OUTPUT_ENABLE_COMPRESSION=false
STAFFMON_OUTPUT_MAX_FILE_SIZE=1GB
STAFFMON_OUTPUT_RETENTION_DAYS=30

# Feature Flags
STAFFMON_FEATURES_ENABLE_CPU_MONITORING=true
STAFFMON_FEATURES_ENABLE_MEMORY_MONITORING=true
STAFFMON_FEATURES_ENABLE_DISK_MONITORING=true
STAFFMON_FEATURES_ENABLE_NETWORK_MONITORING=true
STAFFMON_FEATURES_ENABLE_SERVICE_MONITORING=true
STAFFMON_FEATURES_ENABLE_SECURITY_MONITORING=true
STAFFMON_FEATURES_ENABLE_PROCESS_MONITORING=false
STAFFMON_FEATURES_ENABLE_HARDWARE_MONITORING=true
STAFFMON_FEATURES_ENABLE_UPTIME_MONITORING=true

# Alerting
STAFFMON_ALERTS_ENABLED=false
STAFFMON_ALERTS_CPU_THRESHOLD=80.0
STAFFMON_ALERTS_MEMORY_THRESHOLD=85.0
STAFFMON_ALERTS_DISK_THRESHOLD=90.0
STAFFMON_ALERTS_NETWORK_THRESHOLD=1000000
```

## Configuration Examples

### Minimal Configuration

```toml
# Minimal configuration for basic monitoring
[monitoring]
update_interval = 5
enable_daemon = false

[logging]
level = "info"
file = "/var/log/staffmon.log"

[api]
enabled = false
```

### Production Configuration

```toml
# Production-ready configuration
[monitoring]
update_interval = 2
enable_daemon = true
enable_foreground = false
enable_json_output = true
enable_api = true

[logging]
level = "warn"
file = "/var/log/staffmon.log"
max_size = "100MB"
backup_count = 10
enable_console = false
enable_syslog = true

[api]
enabled = true
host = "0.0.0.0"
port = 8080
api_key = "your-production-api-key"
timeout_seconds = 30
retry_count = 3
rate_limit = 1000
enable_cors = true
enable_ssl = true
ssl_cert = "/etc/ssl/certs/staffmon.crt"
ssl_key = "/etc/ssl/private/staffmon.key"

[output]
json_file = "/var/log/staffmon_data.json"
enable_compression = true
max_file_size = "10GB"
retention_days = 90

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

[alerts]
enabled = true
cpu_threshold = 85.0
memory_threshold = 90.0
disk_threshold = 95.0

[alerts.email]
enabled = true
smtp_server = "smtp.company.com"
smtp_port = 587
username = "alerts@company.com"
password = "secure-password"
from_address = "alerts@company.com"
to_addresses = ["ops@company.com", "admin@company.com"]

[security_settings]
enable_encryption = true
encryption_key = "your-32-byte-encryption-key-here"
enable_audit_log = true
restrict_api_access = true
allowed_ips = ["10.0.0.0/8", "192.168.1.0/24"]
```

### Development Configuration

```toml
# Development configuration with debug logging
[monitoring]
update_interval = 1
enable_daemon = false
enable_foreground = true
enable_json_output = true
enable_api = true

[logging]
level = "debug"
file = "./staffmon.log"
enable_console = true
enable_syslog = false

[api]
enabled = true
host = "127.0.0.1"
port = 8080
api_key = "dev-api-key"
timeout_seconds = 10
retry_count = 1
rate_limit = 1000
enable_cors = true
enable_ssl = false

[output]
json_file = "./staffmon_data.json"
enable_compression = false
max_file_size = "100MB"
retention_days = 7

[features]
enable_cpu_monitoring = true
enable_memory_monitoring = true
enable_disk_monitoring = true
enable_network_monitoring = true
enable_service_monitoring = true
enable_security_monitoring = true
enable_process_monitoring = true
enable_hardware_monitoring = true
enable_uptime_monitoring = true

[performance]
max_threads = 2
buffer_size = 4096
enable_caching = false
cache_ttl = 30
```

## Configuration Validation

The tool validates configuration files on startup. Common validation errors include:

- Invalid file paths
- Invalid port numbers
- Invalid log levels
- Missing required fields
- Invalid boolean values

### Validation Commands

```bash
# Validate configuration file
./staffmon --validate-config /path/to/config.toml

# Show current configuration
./staffmon --show-config

# Generate default configuration
./staffmon --generate-config > config.toml
```

## Configuration Reloading

Currently, configuration changes require a restart of the monitoring daemon. Future versions will support hot-reloading of configuration changes.

```bash
# Restart daemon to apply configuration changes
sudo systemctl restart staffmon

# Or manually restart
./staffmon --stop
./staffmon --start
```

## Best Practices

### Security

1. **Use strong API keys**: Generate cryptographically secure API keys
2. **Enable SSL/TLS**: Use HTTPS for API communication in production
3. **Restrict API access**: Limit API access to trusted IP addresses
4. **Enable audit logging**: Log all configuration changes and API access
5. **Use environment variables**: Store sensitive configuration in environment variables

### Performance

1. **Optimize update intervals**: Balance monitoring frequency with system resources
2. **Enable compression**: Compress output files to save disk space
3. **Configure retention**: Set appropriate data retention periods
4. **Use daemon mode**: Run in background for production environments
5. **Monitor resource usage**: Disable unused monitoring features

### Monitoring

1. **Set appropriate thresholds**: Configure alert thresholds based on your system
2. **Monitor critical services**: Include essential services in monitoring
3. **Enable security monitoring**: Monitor security-related metrics
4. **Configure alerting**: Set up multiple alert channels for redundancy
5. **Regular maintenance**: Review and update configuration regularly

## Troubleshooting Configuration Issues

### Common Issues

1. **Permission denied errors**: Ensure proper file permissions
2. **Invalid configuration**: Check TOML syntax and validation
3. **Missing dependencies**: Install required system packages
4. **Port conflicts**: Ensure API port is not in use
5. **Disk space**: Monitor output file sizes and retention

### Debug Commands

```bash
# Check configuration syntax
./staffmon --validate-config config.toml

# Run with debug logging
RUST_LOG=debug ./staffmon --config config.toml

# Test API connectivity
curl -H "Authorization: Bearer your-api-key" http://localhost:8080/api/v1/system/info

# Check log files
tail -f /var/log/staffmon.log
``` 