# StaffMon Developer Guide

## Table of Contents

1. [Getting Started](#getting-started)
2. [Basic Usage](#basic-usage)
3. [Advanced Usage](#advanced-usage)
4. [Integration Patterns](#integration-patterns)
5. [Best Practices](#best-practices)
6. [Troubleshooting](#troubleshooting)

## Getting Started

### Prerequisites

- Rust 1.70+ installed
- Linux system with systemd or init.d
- Root or sudo access for system monitoring
- Network access for API integration

### Installation

```bash
# Clone the repository
git clone https://github.com/forniya/StaffLinuxMonitor.git
cd StaffLinuxMonitor

# Build the project
cargo build --release

# Install system dependencies
sudo apt-get install sysstat dmidecode
```

### Quick Start

```rust
use staffmon::{get_system_info, save_to_json};

fn main() -> Result<()> {
    // Collect system information
    let system_info = get_system_info();
    
    // Save to JSON file
    save_to_json(&system_info)?;
    
    println!("System information collected and saved!");
    Ok(())
}
```

## Basic Usage

### System Information Collection

```rust
use staffmon::get_system_info;

fn collect_basic_info() {
    let info = get_system_info();
    
    println!("Hostname: {}", info.hostname);
    println!("OS: {}", info.os_version);
    println!("Kernel: {}", info.kernel_version);
    println!("CPU Usage: {:.1}%", info.cpu.usage_percent);
    println!("Memory: {}/{} MB", info.memory.used_mb, info.memory.total_mb);
}
```

### Service Monitoring

```rust
use staffmon::{get_services, get_service_status};

fn monitor_services() {
    let services = get_services();
    
    for service in &services {
        let status = if service.active { "Running" } else { "Stopped" };
        println!("{}: {}", service.name, status);
        
        if let Some(version) = &service.version {
            println!("  Version: {}", version);
        }
    }
}

fn check_specific_service(service_name: &str) {
    let (active, enabled) = get_service_status(service_name);
    println!("{} - Active: {}, Enabled: {}", service_name, active, enabled);
}
```

### Network Monitoring

```rust
use staffmon::get_system_info;

fn monitor_network() {
    let info = get_system_info();
    
    for interface in &info.network.interfaces {
        println!("Interface: {}", interface.name);
        println!("  IP Addresses: {:?}", interface.ip_addresses);
        println!("  RX: {} bytes", interface.rx_bytes);
        println!("  TX: {} bytes", interface.tx_bytes);
    }
}
```

### Security Monitoring

```rust
use staffmon::get_security_info;

fn check_security() {
    let security = get_security_info();
    
    println!("Firewall: {}", if security.firewall_enabled { "Enabled" } else { "Disabled" });
    println!("Fail2ban: {}", if security.fail2ban_active { "Active" } else { "Inactive" });
    println!("Open Ports: {:?}", security.open_ports);
    println!("Available Updates: {}", security.package_updates.len());
}
```

## Advanced Usage

### Custom Monitoring Loop

```rust
use staffmon::{get_system_info, save_to_json, ApiClient, ApiConfig};
use std::time::Duration;
use std::thread;

fn custom_monitor(interval_seconds: u64) -> Result<()> {
    let config = ApiConfig::load()?;
    let client = ApiClient::new(config)?;
    
    loop {
        let system_info = get_system_info();
        
        // Save locally
        if let Err(e) = save_to_json(&system_info) {
            eprintln!("Failed to save: {}", e);
        }
        
        // Send to API
        if let Err(e) = client.send_system_info(&system_info) {
            eprintln!("Failed to send to API: {}", e);
        }
        
        thread::sleep(Duration::from_secs(interval_seconds));
    }
}
```

### Selective Data Collection

```rust
use staffmon::{get_cpu_info, get_memory_info, get_disk_info, System};

fn collect_specific_data() {
    let mut sys = System::new_all();
    sys.refresh_all();
    
    // Only CPU and memory
    let cpu_info = get_cpu_info(&sys);
    let mem_info = get_memory_info(&sys);
    
    println!("CPU: {:.1}% at {} MHz", cpu_info.usage_percent, cpu_info.frequency_mhz);
    println!("Memory: {}/{} MB", mem_info.used_mb, mem_info.total_mb);
    
    // Only disk information
    let disks = get_disk_info(&sys);
    for disk in &disks {
        if disk.total_gb > 10.0 { // Only disks larger than 10GB
            println!("{}: {:.1}GB free of {:.1}GB", disk.name, disk.free_gb, disk.total_gb);
        }
    }
}
```

### Error Handling with Retry Logic

```rust
use staffmon::{ApiClient, ApiConfig, get_system_info};
use std::time::Duration;
use std::thread;

fn send_with_retry(max_retries: u32) -> Result<()> {
    let config = ApiConfig::load()?;
    let client = ApiClient::new(config)?;
    let system_info = get_system_info();
    
    for attempt in 1..=max_retries {
        match client.send_system_info(&system_info) {
            Ok(_) => {
                println!("Successfully sent system info");
                return Ok(());
            }
            Err(e) => {
                eprintln!("Attempt {} failed: {}", attempt, e);
                if attempt < max_retries {
                    thread::sleep(Duration::from_secs(2 * attempt)); // Exponential backoff
                }
            }
        }
    }
    
    Err(anyhow::anyhow!("Failed to send after {} attempts", max_retries))
}
```

### Configuration Management

```rust
use staffmon::ApiConfig;
use std::env;

fn setup_configuration() -> Result<ApiConfig> {
    // Set environment variables
    env::set_var("STAFFMON_BASE_URL", "https://api.example.com");
    env::set_var("STAFFMON_API_KEY", "your-secret-key");
    env::set_var("STAFFMON_TIMEOUT_SECONDS", "60");
    
    // Load configuration
    let config = ApiConfig::load()?;
    
    println!("API URL: {}", config.base_url);
    println!("Timeout: {} seconds", config.timeout_seconds);
    
    Ok(config)
}
```

## Integration Patterns

### Web Service Integration

```rust
use staffmon::{get_system_info, ApiClient, ApiConfig};
use actix_web::{web, App, HttpServer, HttpResponse};
use serde_json::json;

async fn health_check() -> HttpResponse {
    let system_info = get_system_info();
    
    HttpResponse::Ok().json(json!({
        "status": "healthy",
        "hostname": system_info.hostname,
        "cpu_usage": system_info.cpu.usage_percent,
        "memory_usage": system_info.memory.used_mb,
        "timestamp": system_info.timestamp
    }))
}

async fn system_status() -> HttpResponse {
    let system_info = get_system_info();
    HttpResponse::Ok().json(system_info)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health_check))
            .route("/status", web::get().to(system_status))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

### Database Integration

```rust
use staffmon::get_system_info;
use sqlx::{PgPool, Row};
use chrono::Utc;

async fn store_system_info(pool: &PgPool) -> Result<()> {
    let system_info = get_system_info();
    
    sqlx::query(
        "INSERT INTO system_metrics (hostname, cpu_usage, memory_used, memory_total, timestamp) 
         VALUES ($1, $2, $3, $4, $5)"
    )
    .bind(&system_info.hostname)
    .bind(system_info.cpu.usage_percent)
    .bind(system_info.memory.used_mb)
    .bind(system_info.memory.total_mb)
    .bind(Utc::now())
    .execute(pool)
    .await?;
    
    Ok(())
}
```

### Message Queue Integration

```rust
use staffmon::get_system_info;
use lapin::Connection;
use serde_json;

async fn publish_system_info(connection: &Connection) -> Result<()> {
    let system_info = get_system_info();
    let payload = serde_json::to_string(&system_info)?;
    
    let channel = connection.create_channel().await?;
    channel
        .basic_publish(
            "",
            "system.metrics",
            lapin::options::BasicPublishOptions::default(),
            &payload.as_bytes(),
            lapin::BasicProperties::default(),
        )
        .await?;
    
    Ok(())
}
```

### Prometheus Metrics

```rust
use staffmon::get_system_info;
use prometheus::{Counter, Gauge, Registry};

fn create_metrics() -> (Registry, Gauge, Counter) {
    let registry = Registry::new();
    
    let cpu_usage = Gauge::new("system_cpu_usage_percent", "CPU usage percentage").unwrap();
    let memory_usage = Gauge::new("system_memory_usage_bytes", "Memory usage in bytes").unwrap();
    let metrics_total = Counter::new("system_metrics_total", "Total metrics collected").unwrap();
    
    registry.register(Box::new(cpu_usage.clone())).unwrap();
    registry.register(Box::new(memory_usage.clone())).unwrap();
    registry.register(Box::new(metrics_total.clone())).unwrap();
    
    (registry, cpu_usage, metrics_total)
}

fn update_metrics(cpu_gauge: &Gauge, metrics_counter: &Counter) {
    let system_info = get_system_info();
    
    cpu_gauge.set(system_info.cpu.usage_percent as f64);
    metrics_counter.inc();
}
```

## Best Practices

### 1. Error Handling

```rust
use anyhow::Result;
use log::{error, info, warn};

fn robust_monitoring() -> Result<()> {
    info!("Starting system monitoring");
    
    let system_info = match get_system_info() {
        Ok(info) => info,
        Err(e) => {
            error!("Failed to collect system info: {}", e);
            return Err(e);
        }
    };
    
    // Handle partial failures gracefully
    match save_to_json(&system_info) {
        Ok(_) => info!("System info saved successfully"),
        Err(e) => warn!("Failed to save system info: {}", e),
    }
    
    Ok(())
}
```

### 2. Resource Management

```rust
use std::time::Duration;
use std::thread;

fn resource_efficient_monitoring() {
    let mut sys = System::new_all();
    
    loop {
        // Refresh system data efficiently
        sys.refresh_cpu();
        sys.refresh_memory();
        
        let cpu_info = get_cpu_info(&sys);
        let mem_info = get_memory_info(&sys);
        
        // Process data...
        
        // Sleep to prevent excessive resource usage
        thread::sleep(Duration::from_secs(5));
    }
}
```

### 3. Configuration Validation

```rust
use staffmon::ApiConfig;

fn validate_config(config: &ApiConfig) -> Result<()> {
    if config.base_url.is_empty() {
        return Err(anyhow::anyhow!("Base URL cannot be empty"));
    }
    
    if config.api_key.is_empty() {
        return Err(anyhow::anyhow!("API key cannot be empty"));
    }
    
    if config.timeout_seconds == 0 {
        return Err(anyhow::anyhow!("Timeout must be greater than 0"));
    }
    
    Ok(())
}
```

### 4. Logging Best Practices

```rust
use log::{debug, error, info, warn};

fn comprehensive_logging() -> Result<()> {
    info!("Starting comprehensive system monitoring");
    
    let system_info = get_system_info();
    debug!("Collected system info for host: {}", system_info.hostname);
    
    // Log performance metrics
    info!("CPU Usage: {:.1}%, Memory: {}/{} MB", 
          system_info.cpu.usage_percent,
          system_info.memory.used_mb,
          system_info.memory.total_mb);
    
    // Log warnings for high resource usage
    if system_info.cpu.usage_percent > 80.0 {
        warn!("High CPU usage detected: {:.1}%", system_info.cpu.usage_percent);
    }
    
    if system_info.memory.used_mb as f32 / system_info.memory.total_mb as f32 > 0.9 {
        warn!("High memory usage detected: {:.1}%", 
              system_info.memory.used_mb as f32 / system_info.memory.total_mb as f32 * 100.0);
    }
    
    Ok(())
}
```

### 5. Security Considerations

```rust
use staffmon::get_system_info;

fn secure_monitoring() -> Result<()> {
    let system_info = get_system_info();
    
    // Sanitize sensitive information
    let sanitized_info = SystemInfo {
        hostname: system_info.hostname,
        cpu: system_info.cpu,
        memory: system_info.memory,
        // ... other fields
        user_access: UserAccess {
            last_ssh_logins: vec![], // Don't expose login details
            active_users: vec![],    // Don't expose user details
            sudo_users: vec![],      // Don't expose sudo users
        },
        // ... rest of fields
    };
    
    // Send sanitized information
    Ok(())
}
```

## Troubleshooting

### Common Issues

1. **Permission Denied Errors**
```rust
// Ensure proper permissions
use std::process::Command;

fn check_permissions() -> Result<()> {
    let output = Command::new("id").output()?;
    let uid = String::from_utf8_lossy(&output.stdout);
    
    if !uid.contains("uid=0") {
        warn!("Running without root privileges. Some features may be limited.");
    }
    
    Ok(())
}
```

2. **API Connection Issues**
```rust
use staffmon::{ApiClient, ApiConfig};

fn test_api_connection() -> Result<()> {
    let config = ApiConfig::load()?;
    let client = ApiClient::new(config)?;
    
    // Test connection with timeout
    match client.get_system_info() {
        Ok(_) => info!("API connection successful"),
        Err(e) => error!("API connection failed: {}", e),
    }
    
    Ok(())
}
```

3. **Service Detection Issues**
```rust
use staffmon::get_service_status;

fn diagnose_service_issues() {
    let test_services = vec!["nginx", "apache2", "mysql", "redis"];
    
    for service in test_services {
        let (active, enabled) = get_service_status(service);
        println!("{}: Active={}, Enabled={}", service, active, enabled);
    }
}
```

### Debug Mode

```rust
use log::LevelFilter;
use env_logger;

fn enable_debug_mode() {
    env_logger::Builder::new()
        .filter_level(LevelFilter::Debug)
        .init();
    
    log::debug!("Debug mode enabled");
}
```

### Performance Monitoring

```rust
use std::time::Instant;

fn monitor_performance() {
    let start = Instant::now();
    let system_info = get_system_info();
    let duration = start.elapsed();
    
    println!("System info collection took: {:?}", duration);
    
    if duration.as_millis() > 1000 {
        warn!("System info collection is taking longer than expected");
    }
}
```

This developer guide provides comprehensive examples and patterns for integrating StaffMon into various applications and systems. The examples cover common use cases and best practices for production deployments.