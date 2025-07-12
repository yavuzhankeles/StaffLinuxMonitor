# StaffMon Quick Reference Guide

## Installation

```bash
# Clone the repository
git clone https://github.com/forniya/StaffLinuxMonitor.git
cd StaffLinuxMonitor

# Build the project
cargo build --release

# Run (will daemonize)
./target/release/staffmon
```

## Configuration

Create `config.toml`:
```toml
base_url = "https://api.example.com"
api_key = "your-api-key"
timeout_seconds = 30
retry_count = 3
rate_limit = 100
```

Or use environment variables:
```bash
export STAFFMON_BASE_URL="https://api.example.com"
export STAFFMON_API_KEY="your-api-key"
```

## Core Data Structures

### SystemInfo (Main Structure)
```rust
SystemInfo {
    cpu: CpuInfo,
    memory: MemoryInfo,
    load_avg: LoadAverage,
    disks: Vec<DiskInfo>,
    network: NetworkInfo,
    user_access: UserAccess,
    services: Vec<ServiceInfo>,
    security: SecurityInfo,
    hardware: HardwareInfo,
    system_uptime: UptimeInfo,
    hostname: String,
    kernel_version: String,
    os_version: String,
    process_list: Vec<ProcessInfo>,
    timestamp: String,
}
```

## Quick API Usage

### Basic System Info Collection
```rust
use staffmon::get_system_info;

let info = get_system_info();
println!("CPU Usage: {}%", info.cpu.usage_percent);
println!("Memory: {}MB / {}MB", info.memory.used_mb, info.memory.total_mb);
```

### Send to API
```rust
use staffmon::{ApiClient, ApiConfig};

let config = ApiConfig::load()?;
let client = ApiClient::new(config)?;
client.send_system_info(&info)?;
```

### Save to JSON
```rust
use staffmon::save_to_json;

save_to_json(&info)?;
// Creates: system_info_20240115_103000.json
```

## Key Functions

| Function | Purpose | Returns |
|----------|---------|---------|
| `get_system_info()` | Collect all system data | `SystemInfo` |
| `get_cpu_info(&sys)` | CPU metrics | `CpuInfo` |
| `get_memory_info(&sys)` | Memory usage | `MemoryInfo` |
| `get_disk_info(&sys)` | Disk usage | `Vec<DiskInfo>` |
| `get_network_info(&sys)` | Network interfaces | `NetworkInfo` |
| `get_services()` | Service status | `Vec<ServiceInfo>` |
| `get_security_info()` | Security status | `SecurityInfo` |
| `save_to_json(&info)` | Save to file | `io::Result<()>` |

## API Client Methods

| Method | Purpose | Parameters |
|--------|---------|------------|
| `ApiClient::new(config)` | Create client | `ApiConfig` |
| `send_system_info(&info)` | POST data | `&SystemInfo` |
| `get_system_info()` | GET data | None |

## JSON Output Example

```json
{
  "cpu": {
    "usage_percent": 23.5,
    "temperature_celsius": null,
    "frequency_mhz": 2400.0
  },
  "memory": {
    "total_mb": 16384,
    "used_mb": 8192,
    "free_mb": 8192
  },
  "load_avg": {
    "one": 1.23,
    "five": 1.45,
    "fifteen": 1.67
  },
  "disks": [{
    "name": "sda1",
    "total_gb": 500.0,
    "used_gb": 250.0,
    "free_gb": 250.0,
    "mount_point": "/"
  }],
  "network": {
    "interfaces": [{
      "name": "eth0",
      "ip_addresses": ["192.168.1.100"],
      "rx_bytes": 1234567,
      "tx_bytes": 7654321
    }]
  },
  "services": [{
    "name": "nginx",
    "active": true,
    "enabled": true,
    "version": "nginx/1.18.0"
  }],
  "security": {
    "firewall_enabled": true,
    "fail2ban_active": false,
    "open_ports": [22, 80, 443],
    "package_updates": ["package1", "package2"]
  },
  "hostname": "server01",
  "kernel_version": "5.15.0-56-generic",
  "timestamp": "2024-01-15T10:30:00+00:00"
}
```

## Common Use Cases

### 1. Monitor Specific Metrics
```rust
let info = get_system_info();

// CPU monitoring
if info.cpu.usage_percent > 80.0 {
    log::warn!("High CPU usage: {}%", info.cpu.usage_percent);
}

// Memory monitoring
let memory_usage_percent = (info.memory.used_mb as f64 / info.memory.total_mb as f64) * 100.0;
if memory_usage_percent > 90.0 {
    log::warn!("High memory usage: {:.1}%", memory_usage_percent);
}

// Disk monitoring
for disk in &info.disks {
    let usage_percent = (disk.used_gb / disk.total_gb) * 100.0;
    if usage_percent > 85.0 {
        log::warn!("Disk {} is {:.1}% full", disk.name, usage_percent);
    }
}
```

### 2. Service Health Check
```rust
let info = get_system_info();

let critical_services = vec!["nginx", "mysql", "redis"];
for service_name in critical_services {
    if let Some(service) = info.services.iter().find(|s| s.name == service_name) {
        if !service.active {
            log::error!("Critical service {} is not active!", service_name);
        }
    } else {
        log::error!("Critical service {} not found!", service_name);
    }
}
```

### 3. Security Audit
```rust
let info = get_system_info();

// Check firewall
if !info.security.firewall_enabled {
    log::warn!("Firewall is disabled!");
}

// Check for updates
if !info.security.package_updates.is_empty() {
    log::info!("{} package updates available", info.security.package_updates.len());
}

// Check open ports
let risky_ports = vec![23, 135, 139, 445]; // Telnet, NetBIOS, SMB
for port in &info.security.open_ports {
    if risky_ports.contains(port) {
        log::warn!("Potentially risky port {} is open", port);
    }
}
```

### 4. Custom Monitoring Loop
```rust
use std::time::Duration;
use std::thread;

fn custom_monitor() -> Result<(), Box<dyn std::error::Error>> {
    let config = ApiConfig::load()?;
    let client = ApiClient::new(config)?;
    
    loop {
        let info = get_system_info();
        
        // Local processing
        if info.cpu.usage_percent > 90.0 {
            // Trigger alert
        }
        
        // Send to API
        client.send_system_info(&info)?;
        
        // Wait 30 seconds
        thread::sleep(Duration::from_secs(30));
    }
}
```

## Error Handling

```rust
// With anyhow
use anyhow::Result;

fn monitor() -> Result<()> {
    let info = get_system_info();
    save_to_json(&info)?;
    Ok(())
}

// With custom error handling
match save_to_json(&info) {
    Ok(_) => println!("Saved successfully"),
    Err(e) => eprintln!("Failed to save: {}", e),
}
```

## Logging

```rust
// Initialize logging
log_config::init_logger()?;

// Use log macros
log::info!("Starting monitoring");
log::warn!("High CPU usage: {}%", cpu_usage);
log::error!("Failed to connect: {}", error);
```

## Daemon Control

```bash
# Start daemon
./staffmon

# Check if running
ps aux | grep staffmon

# Stop daemon
kill $(cat /tmp/staffmon.pid)
```

## Dependencies

Add to your `Cargo.toml`:
```toml
[dependencies]
staffmon = "1.0.2"
# Or for local development
staffmon = { path = "../path/to/staffmon" }
```

## System Requirements

- Linux kernel 3.10+
- Rust 1.70.0+
- System commands: `uptime`, `ip`/`ifconfig`, `who`, `last`
- Optional: `systemctl`, `dmidecode`, `lscpu`

## Tips & Best Practices

1. **Performance**: Cache `System` instance when making multiple calls
   ```rust
   let mut sys = System::new_all();
   sys.refresh_all();
   let cpu = get_cpu_info(&sys);
   let mem = get_memory_info(&sys);
   ```

2. **Error Recovery**: Always handle API failures gracefully
   ```rust
   if let Err(e) = client.send_system_info(&info) {
       log::error!("API failed: {}, saving locally", e);
       save_to_json(&info)?;
   }
   ```

3. **Security**: Never hardcode API keys
   ```rust
   // Bad
   let api_key = "secret-key";
   
   // Good
   let api_key = std::env::var("STAFFMON_API_KEY")?;
   ```

4. **Monitoring**: Set appropriate intervals based on your needs
   - Real-time: 1-5 seconds
   - Normal: 30-60 seconds
   - Low-frequency: 5-15 minutes