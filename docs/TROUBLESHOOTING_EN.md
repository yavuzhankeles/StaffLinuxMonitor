# Troubleshooting Guide

## Overview

This guide covers common issues you may encounter with StaffLinuxMonitor and their solutions. It provides step-by-step approaches to systematically diagnose and resolve problems.

## Problem Diagnosis Process

### 1. Basic Checks

When you encounter any issue, first perform these basic checks:

```bash
# Check service status
sudo systemctl status staffmon

# Check if process is running
ps aux | grep staffmon

# Check port listening status
sudo netstat -tlnp | grep 8080
# or
sudo ss -tlnp | grep 8080

# Check log files
sudo tail -f /var/log/staffmon.log
```

### 2. Configuration Validation

```bash
# Validate configuration file
sudo staffmon --validate-config /etc/staffmon/config.toml

# Show configuration
sudo staffmon --show-config

# Run in test mode
sudo staffmon --config /etc/staffmon/config.toml --test
```

## Common Issues and Solutions

### Service Won't Start

#### Symptoms
- `systemctl start staffmon` command fails
- Service status shows "failed"
- Error messages in log files

#### Possible Causes and Solutions

**1. Permission Issues**

```bash
# Check file permissions
ls -la /opt/staffmon/staffmon
ls -la /etc/staffmon/
ls -la /var/log/staffmon/

# Fix permissions
sudo chown staffmon:staffmon /opt/staffmon/staffmon
sudo chmod +x /opt/staffmon/staffmon
sudo chown -R staffmon:staffmon /etc/staffmon
sudo chown -R staffmon:staffmon /var/log/staffmon
```

**2. Configuration Error**

```bash
# Check configuration syntax
sudo staffmon --validate-config /etc/staffmon/config.toml

# Check for invalid values
grep -E "(true|false)" /etc/staffmon/config.toml
grep -E "[0-9]+" /etc/staffmon/config.toml
```

**3. Dependency Issues**

```bash
# Check required libraries
ldd /opt/staffmon/staffmon

# Install missing libraries
sudo apt-get install libssl1.1 libc6
# or for CentOS/RHEL
sudo yum install openssl-libs glibc
```

**4. System Limits**

```bash
# Check file descriptor limits
ulimit -n

# Increase limits
echo "staffmon soft nofile 65536" | sudo tee -a /etc/security/limits.conf
echo "staffmon hard nofile 65536" | sudo tee -a /etc/security/limits.conf
```

### API Not Accessible

#### Symptoms
- `curl` commands timeout
- Connection error in web browser
- "Connection refused" messages

#### Possible Causes and Solutions

**1. Service Not Running**

```bash
# Check service status
sudo systemctl status staffmon

# Restart service
sudo systemctl restart staffmon

# Check logs
sudo journalctl -u staffmon -f
```

**2. Wrong Port Configuration**

```bash
# Check listening ports
sudo netstat -tlnp | grep staffmon

# Check port in configuration
grep "port" /etc/staffmon/config.toml

# Check port conflicts
sudo lsof -i :8080
```

**3. Firewall Issue**

```bash
# Check UFW status (Ubuntu)
sudo ufw status

# Allow port in UFW
sudo ufw allow 8080/tcp

# Check iptables status
sudo iptables -L | grep 8080

# Add iptables rule
sudo iptables -A INPUT -p tcp --dport 8080 -j ACCEPT
```

**4. Host Binding Issue**

```bash
# Check host setting in configuration
grep "host" /etc/staffmon/config.toml

# Use 0.0.0.0 instead of localhost
sudo sed -i 's/host = "127.0.0.1"/host = "0.0.0.0"/' /etc/staffmon/config.toml

# Restart service
sudo systemctl restart staffmon
```

### High CPU/Memory Usage

#### Symptoms
- System running slowly
- staffmon showing high resource usage in `top`
- System freezing or unresponsive

#### Possible Causes and Solutions

**1. Too Frequent Updates**

```bash
# Check update interval
grep "update_interval" /etc/staffmon/config.toml

# Increase interval (in seconds)
sudo sed -i 's/update_interval = 1/update_interval = 5/' /etc/staffmon/config.toml
```

**2. Too Many Features Enabled**

```bash
# Check enabled features
grep -A 10 "\[features\]" /etc/staffmon/config.toml

# Disable unused features
sudo sed -i 's/enable_process_monitoring = true/enable_process_monitoring = false/' /etc/staffmon/config.toml
```

**3. Large Log Files**

```bash
# Check log file sizes
du -sh /var/log/staffmon/*

# Check log rotation
sudo logrotate -d /etc/logrotate.d/staffmon

# Manual log cleanup
sudo truncate -s 0 /var/log/staffmon.log
```

**4. Insufficient System Resources**

```bash
# Check system resources
free -h
df -h
nproc

# Optimize configuration
sudo tee -a /etc/staffmon/config.toml <<EOF

[performance]
max_threads = 2
buffer_size = 4096
enable_caching = false
cache_ttl = 30
EOF
```

### Disk Space Issues

#### Symptoms
- "No space left on device" errors
- Log files very large
- System running slowly

#### Possible Causes and Solutions

**1. Log Files Too Large**

```bash
# Check disk usage
df -h /var/log

# Find large files
sudo find /var/log/staffmon -type f -size +100M

# Configure log rotation
sudo tee /etc/logrotate.d/staffmon <<EOF
/var/log/staffmon/*.log {
    daily
    missingok
    rotate 7
    compress
    delaycompress
    notifempty
    create 644 staffmon staffmon
    postrotate
        systemctl reload staffmon
    endscript
}
EOF
```

**2. Data Files Too Large**

```bash
# Check data file size
ls -lh /var/log/staffmon_data.json

# Enable compression
sudo sed -i 's/enable_compression = false/enable_compression = true/' /etc/staffmon/config.toml

# Reduce retention period
sudo sed -i 's/retention_days = 30/retention_days = 7/' /etc/staffmon/config.toml
```

**3. Insufficient Disk Space**

```bash
# Analyze disk usage
sudo du -sh /* | sort -hr | head -10

# Clean old files
sudo find /var/log -name "*.log.*" -mtime +7 -delete
sudo find /tmp -mtime +7 -delete
```

### Network Connection Issues

#### Symptoms
- API requests timeout
- Network metrics not collected
- No connection to external services

#### Possible Causes and Solutions

**1. Network Interface Issues**

```bash
# Check network interfaces
ip addr show

# Test network connectivity
ping -c 3 8.8.8.8

# Test DNS resolution
nslookup google.com
```

**2. Firewall Issues**

```bash
# Check firewall status
sudo ufw status
# or
sudo iptables -L

# Open required ports
sudo ufw allow 8080/tcp
sudo ufw allow out 53/tcp
sudo ufw allow out 53/udp
```

**3. Proxy Configuration**

```bash
# Check proxy settings
echo $http_proxy
echo $https_proxy

# Configure proxy settings
export http_proxy="http://proxy.company.com:8080"
export https_proxy="http://proxy.company.com:8080"
```

### Security Issues

#### Symptoms
- API key errors
- Access denied messages
- SSL/TLS errors

#### Possible Causes and Solutions

**1. API Key Issues**

```bash
# Check API key
grep "api_key" /etc/staffmon/config.toml

# Generate new API key
NEW_API_KEY=$(openssl rand -hex 32)
sudo sed -i "s/api_key = \".*\"/api_key = \"$NEW_API_KEY\"/" /etc/staffmon/config.toml

# Test API key
curl -H "Authorization: Bearer $NEW_API_KEY" http://localhost:8080/api/v1/system/info
```

**2. SSL/TLS Issues**

```bash
# Check SSL certificates
openssl x509 -in /etc/ssl/certs/staffmon.crt -text -noout

# Check certificate expiration
openssl x509 -in /etc/ssl/certs/staffmon.crt -noout -dates

# Renew certificates
sudo certbot renew
```

**3. Permission Issues**

```bash
# Check file permissions
ls -la /etc/staffmon/
ls -la /var/log/staffmon/

# Fix permissions
sudo chown -R staffmon:staffmon /etc/staffmon
sudo chmod 600 /etc/staffmon/config.toml
sudo chown -R staffmon:staffmon /var/log/staffmon
```

## Debug and Logging

### Enable Debug Mode

```bash
# Run with debug logging
RUST_LOG=debug sudo staffmon --config /etc/staffmon/config.toml

# Debug specific modules
RUST_LOG=staffmon::api=debug,staffmon::monitoring=debug sudo staffmon --config /etc/staffmon/config.toml
```

### Configure Log Levels

```toml
[logging]
level = "debug"  # trace, debug, info, warn, error
file = "/var/log/staffmon.log"
enable_console = true
enable_syslog = false
```

### Log Analysis

```bash
# Filter error messages
sudo grep "ERROR" /var/log/staffmon.log

# View logs for specific time range
sudo journalctl -u staffmon --since "1 hour ago"

# Monitor log file in real-time
sudo tail -f /var/log/staffmon.log | grep -E "(ERROR|WARN)"
```

## Performance Optimization

### Monitor System Resources

```bash
# Monitor CPU usage
top -p $(pgrep staffmon)

# Monitor memory usage
ps aux | grep staffmon

# Monitor disk I/O
iotop -p $(pgrep staffmon)

# Monitor network usage
iftop -i eth0
```

### Configuration Optimization

```toml
# Performance-optimized configuration
[monitoring]
update_interval = 5  # Less frequent updates
enable_daemon = true
enable_foreground = false

[performance]
max_threads = 2  # Reduce thread count
buffer_size = 4096  # Reduce buffer size
enable_caching = true
cache_ttl = 60

[features]
enable_process_monitoring = false  # Disable resource-intensive feature
enable_security_monitoring = false  # Disable unnecessary features
```

## Emergency Procedures

### Service Completely Down

```bash
# 1. Stop service
sudo systemctl stop staffmon

# 2. Force kill process
sudo pkill -f staffmon

# 3. Clear log files
sudo truncate -s 0 /var/log/staffmon.log

# 4. Backup configuration
sudo cp /etc/staffmon/config.toml /etc/staffmon/config.toml.backup

# 5. Start with default configuration
sudo staffmon --config /etc/staffmon/config.toml --daemon

# 6. Check service status
sudo systemctl status staffmon
```

### Critical Disk Space

```bash
# 1. Check disk usage
df -h

# 2. Find and delete large files
sudo find /var/log -name "*.log.*" -delete
sudo find /tmp -mtime +1 -delete

# 3. Compress log files
sudo gzip /var/log/staffmon.log.*

# 4. Delete old data files
sudo find /var/log/staffmon -name "*.json" -mtime +7 -delete

# 5. Restart service
sudo systemctl restart staffmon
```

### API Security Breach

```bash
# 1. Temporarily disable API
sudo sed -i 's/enabled = true/enabled = false/' /etc/staffmon/config.toml

# 2. Restart service
sudo systemctl restart staffmon

# 3. Check security logs
sudo grep -i "unauthorized\|failed" /var/log/staffmon.log

# 4. Generate new API key
NEW_KEY=$(openssl rand -hex 32)
sudo sed -i "s/api_key = \".*\"/api_key = \"$NEW_KEY\"/" /etc/staffmon/config.toml

# 5. Re-enable API
sudo sed -i 's/enabled = false/enabled = true/' /etc/staffmon/config.toml

# 6. Restart service
sudo systemctl restart staffmon
```

## Getting Support

### Issue Reporting

When experiencing issues, collect the following information:

```bash
# System information
uname -a
cat /etc/os-release
free -h
df -h

# StaffLinuxMonitor information
staffmon --version
systemctl status staffmon
journalctl -u staffmon --no-pager -n 100

# Configuration (hide sensitive information)
grep -v "api_key\|password" /etc/staffmon/config.toml

# Log files
tail -n 100 /var/log/staffmon.log
```

### Community Support

- [GitHub Issues](https://github.com/forniya/StaffLinuxMonitor/issues)
- [GitHub Discussions](https://github.com/forniya/StaffLinuxMonitor/discussions)
- [Wiki](https://github.com/forniya/StaffLinuxMonitor/wiki)

### Professional Support

Professional support services are available for enterprise users. Visit the [support page](https://github.com/forniya/StaffLinuxMonitor/wiki/Support) for details. 