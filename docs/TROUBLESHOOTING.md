# Troubleshooting Guide

## Overview

This guide provides solutions for common issues you may encounter when using StaffLinuxMonitor. Each section includes diagnostic steps and solutions.

## Quick Diagnostic Commands

### System Status Check

```bash
# Check if service is running
sudo systemctl status staffmon

# Check if process is running
ps aux | grep staffmon

# Check if API is responding
curl -f http://localhost:8080/api/v1/health

# Check log files
tail -f /var/log/staffmon.log

# Check configuration
./staffmon --validate-config /etc/staffmon/config.toml
```

### Resource Usage Check

```bash
# Check CPU and memory usage
top -p $(pgrep staffmon)

# Check disk usage
du -sh /var/log/staffmon/*

# Check file descriptors
lsof -p $(pgrep staffmon) | wc -l

# Check network connections
netstat -tlnp | grep staffmon
```

## Common Issues and Solutions

### 1. Service Won't Start

#### Symptoms
- `systemctl start staffmon` fails
- Service shows as "failed" or "inactive"
- No process running

#### Diagnostic Steps

```bash
# Check service status
sudo systemctl status staffmon

# Check detailed logs
sudo journalctl -u staffmon -n 50

# Check configuration syntax
./staffmon --validate-config /etc/staffmon/config.toml

# Check file permissions
ls -la /usr/local/bin/staffmon
ls -la /etc/staffmon/
ls -la /var/log/staffmon/
```

#### Common Solutions

**Permission Issues:**
```bash
# Fix binary permissions
sudo chmod +x /usr/local/bin/staffmon

# Fix directory permissions
sudo chown -R staffmon:staffmon /etc/staffmon
sudo chown -R staffmon:staffmon /var/log/staffmon

# Fix log directory permissions
sudo chmod 755 /var/log/staffmon
```

**Configuration Issues:**
```bash
# Check TOML syntax
./staffmon --validate-config /etc/staffmon/config.toml

# Generate default config
./staffmon --generate-config > /tmp/default_config.toml

# Compare with current config
diff /tmp/default_config.toml /etc/staffmon/config.toml
```

**Port Conflicts:**
```bash
# Check if port is in use
sudo netstat -tlnp | grep 8080
sudo lsof -i :8080

# Kill conflicting process
sudo kill -9 $(lsof -t -i:8080)
```

### 2. API Not Responding

#### Symptoms
- `curl http://localhost:8080/api/v1/system/info` fails
- Connection refused errors
- Timeout errors

#### Diagnostic Steps

```bash
# Check if service is listening
sudo netstat -tlnp | grep 8080

# Test local connectivity
telnet localhost 8080

# Check firewall rules
sudo ufw status
sudo iptables -L

# Test with verbose curl
curl -v http://localhost:8080/api/v1/system/info
```

#### Common Solutions

**API Not Enabled:**
```toml
# In config.toml
[api]
enabled = true
host = "127.0.0.1"
port = 8080
```

**Firewall Blocking:**
```bash
# Allow port through UFW
sudo ufw allow 8080/tcp

# Allow port through iptables
sudo iptables -A INPUT -p tcp --dport 8080 -j ACCEPT
```

**Wrong Host Binding:**
```toml
# Change from localhost to all interfaces
[api]
host = "0.0.0.0"  # Instead of "127.0.0.1"
```

### 3. High Resource Usage

#### Symptoms
- High CPU usage (>50%)
- High memory usage (>500MB)
- Slow system response
- Large log files

#### Diagnostic Steps

```bash
# Check resource usage
top -p $(pgrep staffmon)
htop -p $(pgrep staffmon)

# Check log file sizes
du -sh /var/log/staffmon/*

# Check update frequency
grep "update_interval" /etc/staffmon/config.toml

# Check enabled features
grep -E "enable_.*_monitoring" /etc/staffmon/config.toml
```

#### Common Solutions

**Reduce Update Frequency:**
```toml
[monitoring]
update_interval = 5  # Increase from 2 to 5 seconds
```

**Disable Unused Features:**
```toml
[features]
enable_process_monitoring = false  # Most resource intensive
enable_hardware_monitoring = false
enable_uptime_monitoring = false
```

**Enable Log Rotation:**
```toml
[logging]
max_size = "50MB"      # Reduce from 100MB
backup_count = 3       # Reduce from 5
```

**Enable Compression:**
```toml
[output]
enable_compression = true
```

### 4. Permission Denied Errors

#### Symptoms
- "Permission denied" in logs
- Cannot read system files
- Cannot write to log files

#### Diagnostic Steps

```bash
# Check user and group
id staffmon

# Check file permissions
ls -la /var/log/staffmon/
ls -la /etc/staffmon/

# Check SELinux context (RHEL/CentOS)
ls -Z /usr/local/bin/staffmon
```

#### Common Solutions

**Fix User/Group:**
```bash
# Create user if missing
sudo useradd -r -s /bin/false -d /opt/staffmon staffmon

# Set correct ownership
sudo chown -R staffmon:staffmon /opt/staffmon
sudo chown -R staffmon:staffmon /etc/staffmon
sudo chown -R staffmon:staffmon /var/log/staffmon
```

**Fix SELinux (RHEL/CentOS):**
```bash
# Set correct context
sudo chcon -t bin_t /usr/local/bin/staffmon
sudo chcon -R -t staffmon_exec_t /opt/staffmon/

# Or temporarily disable SELinux
sudo setenforce 0
```

### 5. Configuration Issues

#### Symptoms
- Configuration validation fails
- Unexpected behavior
- Missing features

#### Diagnostic Steps

```bash
# Validate configuration
./staffmon --validate-config /etc/staffmon/config.toml

# Show current configuration
./staffmon --show-config

# Check configuration file syntax
cat -n /etc/staffmon/config.toml
```

#### Common Solutions

**Invalid TOML Syntax:**
```toml
# Common syntax errors:
# Missing quotes around strings
api_key = your-api-key-here  # Wrong
api_key = "your-api-key-here"  # Correct

# Missing brackets for sections
[monitoring]  # Must have brackets
update_interval = 2

# Invalid boolean values
enable_daemon = yes  # Wrong
enable_daemon = true  # Correct
```

**Missing Required Sections:**
```toml
# Ensure all required sections are present
[monitoring]
[logging]
[api]
[output]
[features]
```

### 6. Network Issues

#### Symptoms
- Cannot connect to external APIs
- Network monitoring fails
- DNS resolution issues

#### Diagnostic Steps

```bash
# Check network connectivity
ping -c 3 8.8.8.8
nslookup google.com

# Check DNS resolution
cat /etc/resolv.conf

# Check network interfaces
ip addr show

# Test API connectivity
curl -v https://api.example.com
```

#### Common Solutions

**DNS Issues:**
```bash
# Add DNS servers
echo "nameserver 8.8.8.8" | sudo tee -a /etc/resolv.conf
echo "nameserver 8.8.4.4" | sudo tee -a /etc/resolv.conf
```

**Proxy Configuration:**
```bash
# Set proxy environment variables
export HTTP_PROXY=http://proxy.company.com:8080
export HTTPS_PROXY=http://proxy.company.com:8080
export NO_PROXY=localhost,127.0.0.1
```

### 7. Log File Issues

#### Symptoms
- Large log files
- Disk space full
- No logs being written

#### Diagnostic Steps

```bash
# Check log file sizes
du -sh /var/log/staffmon/*

# Check disk space
df -h /var/log/staffmon

# Check log file permissions
ls -la /var/log/staffmon/

# Check log rotation
sudo logrotate -d /etc/logrotate.d/staffmon
```

#### Common Solutions

**Enable Log Rotation:**
```bash
# Create logrotate configuration
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

**Manual Log Cleanup:**
```bash
# Archive old logs
sudo tar -czf /var/log/staffmon/archive_$(date +%Y%m%d).tar.gz /var/log/staffmon/*.log.1

# Remove old log files
sudo find /var/log/staffmon -name "*.log.*" -mtime +7 -delete
```

### 8. Performance Issues

#### Symptoms
- Slow API responses
- High latency
- System becomes unresponsive

#### Diagnostic Steps

```bash
# Check system load
uptime
top

# Check I/O wait
iostat -x 1 5

# Check memory usage
free -h

# Check disk I/O
iotop -p $(pgrep staffmon)
```

#### Common Solutions

**Optimize Configuration:**
```toml
[performance]
max_threads = 2          # Reduce from 4
buffer_size = 4096       # Reduce from 8192
enable_caching = true
cache_ttl = 30           # Reduce from 60
```

**System Tuning:**
```bash
# Increase file descriptor limits
echo "staffmon soft nofile 65536" | sudo tee -a /etc/security/limits.conf
echo "staffmon hard nofile 65536" | sudo tee -a /etc/security/limits.conf

# Optimize kernel parameters
echo "vm.swappiness=10" | sudo tee -a /etc/sysctl.conf
sudo sysctl -p
```

## Advanced Troubleshooting

### Debug Mode

```bash
# Run with debug logging
RUST_LOG=debug ./staffmon --config /etc/staffmon/config.toml

# Run with trace logging
RUST_LOG=trace ./staffmon --config /etc/staffmon/config.toml

# Run in foreground with verbose output
./staffmon --config /etc/staffmon/config.toml --verbose
```

### Profiling

```bash
# Install profiling tools
sudo apt-get install -y perf-tools-unstable

# Profile CPU usage
sudo perf record -g -p $(pgrep staffmon) -- sleep 30
sudo perf report

# Profile memory usage
sudo valgrind --tool=massif /usr/local/bin/staffmon
```

### Network Debugging

```bash
# Capture network traffic
sudo tcpdump -i lo -w staffmon.pcap port 8080

# Analyze with Wireshark
wireshark staffmon.pcap

# Test API endpoints
curl -X GET -H "Authorization: Bearer your-api-key" \
     -H "Content-Type: application/json" \
     http://localhost:8080/api/v1/system/info
```

## Recovery Procedures

### Complete Reset

```bash
# Stop service
sudo systemctl stop staffmon

# Backup current configuration
sudo cp -r /etc/staffmon /etc/staffmon.backup.$(date +%Y%m%d)

# Remove all data
sudo rm -rf /var/log/staffmon/*

# Restore default configuration
sudo cp /etc/staffmon.backup.$(date +%Y%m%d)/config.toml /etc/staffmon/

# Restart service
sudo systemctl start staffmon
```

### Data Recovery

```bash
# Check for backup files
find /var/log/staffmon -name "*.backup" -o -name "*.old"

# Restore from backup
sudo cp /var/log/staffmon/staffmon_data.json.backup /var/log/staffmon/staffmon_data.json

# Fix permissions
sudo chown staffmon:staffmon /var/log/staffmon/staffmon_data.json
```

### Service Recovery

```bash
# Reset systemd service
sudo systemctl reset-failed staffmon

# Reload systemd
sudo systemctl daemon-reload

# Restart service
sudo systemctl restart staffmon

# Check status
sudo systemctl status staffmon
```

## Monitoring and Alerting

### Health Check Script

```bash
#!/bin/bash
# health-check.sh

# Check if service is running
if ! systemctl is-active --quiet staffmon; then
    echo "CRITICAL: StaffLinuxMonitor service is down"
    systemctl restart staffmon
    exit 2
fi

# Check API health
if ! curl -f -s http://localhost:8080/api/v1/health > /dev/null; then
    echo "CRITICAL: StaffLinuxMonitor API is not responding"
    exit 2
fi

# Check disk space
DISK_USAGE=$(df /var/log/staffmon | tail -1 | awk '{print $5}' | sed 's/%//')
if [ "$DISK_USAGE" -gt 90 ]; then
    echo "WARNING: Disk usage is high: ${DISK_USAGE}%"
    exit 1
fi

# Check log file sizes
LARGEST_LOG=$(find /var/log/staffmon -name "*.log" -exec ls -lh {} \; | sort -k5 -hr | head -1)
if [[ $LARGEST_LOG =~ ([0-9]+)M ]]; then
    SIZE=${BASH_REMATCH[1]}
    if [ "$SIZE" -gt 100 ]; then
        echo "WARNING: Large log file detected: $LARGEST_LOG"
        exit 1
    fi
fi

echo "OK: StaffLinuxMonitor is healthy"
exit 0
```

### Automated Recovery

```bash
#!/bin/bash
# auto-recovery.sh

# Check service status
if ! systemctl is-active --quiet staffmon; then
    echo "$(date): Service down, attempting restart"
    systemctl restart staffmon
    
    # Wait and check again
    sleep 10
    if ! systemctl is-active --quiet staffmon; then
        echo "$(date): Restart failed, attempting full recovery"
        
        # Stop service
        systemctl stop staffmon
        
        # Clean up
        rm -f /var/run/staffmon.pid
        
        # Start service
        systemctl start staffmon
        
        # Final check
        sleep 10
        if systemctl is-active --quiet staffmon; then
            echo "$(date): Recovery successful"
        else
            echo "$(date): Recovery failed, manual intervention required"
            # Send alert
        fi
    fi
fi
```

## Getting Help

### Information to Collect

When reporting issues, collect the following information:

```bash
# System information
uname -a
cat /etc/os-release
free -h
df -h

# Service information
systemctl status staffmon
journalctl -u staffmon -n 100

# Configuration
cat /etc/staffmon/config.toml

# Log files
tail -n 50 /var/log/staffmon.log

# Process information
ps aux | grep staffmon
lsof -p $(pgrep staffmon)
```

### Support Channels

- **GitHub Issues**: [Create an issue](https://github.com/forniya/StaffLinuxMonitor/issues)
- **GitHub Discussions**: [Ask questions](https://github.com/forniya/StaffLinuxMonitor/discussions)
- **Documentation**: [Wiki](https://github.com/forniya/StaffLinuxMonitor/wiki)

### Useful Commands Reference

```bash
# Service management
sudo systemctl start staffmon
sudo systemctl stop staffmon
sudo systemctl restart staffmon
sudo systemctl status staffmon
sudo systemctl enable staffmon
sudo systemctl disable staffmon

# Log viewing
sudo journalctl -u staffmon -f
sudo journalctl -u staffmon --since "1 hour ago"
tail -f /var/log/staffmon.log

# Configuration
./staffmon --validate-config /etc/staffmon/config.toml
./staffmon --show-config
./staffmon --generate-config

# API testing
curl -H "Authorization: Bearer your-api-key" http://localhost:8080/api/v1/system/info
curl -H "Authorization: Bearer your-api-key" http://localhost:8080/api/v1/health

# Process management
pgrep staffmon
pkill staffmon
kill -HUP $(pgrep staffmon)
``` 