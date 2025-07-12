# StaffMon API Documentation

## Table of Contents
- [Overview](#overview)
- [Module Structure](#module-structure)
- [Data Structures](#data-structures)
- [API Module](#api-module)
- [Configuration Module](#configuration-module)
- [Logging Module](#logging-module)
- [Core Functions](#core-functions)
- [Usage Examples](#usage-examples)
- [Error Handling](#error-handling)

## Overview

StaffMon is a comprehensive Linux system monitoring tool written in Rust. It collects detailed system information including CPU, memory, disk, network, services, security, and hardware metrics, and can send this data to a remote API endpoint.

## Module Structure

```
staffmon/
├── src/
│   ├── main.rs         # Main application logic and data structures
│   ├── api.rs          # API client for remote communication
│   ├── config.rs       # Configuration management
│   └── log_config.rs   # Logging configuration
```

## Data Structures

### SystemInfo

The main structure containing all system information.

```rust
#[derive(Debug, Serialize)]
struct SystemInfo {
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

**Example JSON Output:**
```json
{
  "cpu": {
    "usage_percent": 15.5,
    "temperature_celsius": null,
    "frequency_mhz": 2400.0
  },
  "memory": {
    "total_mb": 16384,
    "used_mb": 8192,
    "free_mb": 8192
  },
  "hostname": "server01",
  "timestamp": "2024-01-15T10:30:00+00:00"
}
```

### CpuInfo

CPU information structure.

```rust
#[derive(Debug, Serialize)]
struct CpuInfo {
    usage_percent: f32,
    temperature_celsius: Option<f32>,
    frequency_mhz: f32,
}
```

### MemoryInfo

Memory usage information.

```rust
#[derive(Debug, Serialize)]
struct MemoryInfo {
    total_mb: u64,
    used_mb: u64,
    free_mb: u64,
}
```

### LoadAverage

System load averages.

```rust
#[derive(Debug, Serialize)]
struct LoadAverage {
    one: f64,      // 1-minute load average
    five: f64,     // 5-minute load average
    fifteen: f64,  // 15-minute load average
}
```

### DiskInfo

Disk storage information.

```rust
#[derive(Debug, Serialize)]
struct DiskInfo {
    name: String,
    total_gb: f64,
    used_gb: f64,
    free_gb: f64,
    mount_point: String,
}
```

### NetworkInfo & NetworkInterface

Network interface information.

```rust
#[derive(Debug, Serialize)]
struct NetworkInfo {
    interfaces: Vec<NetworkInterface>,
}

#[derive(Debug, Serialize)]
struct NetworkInterface {
    name: String,
    ip_addresses: Vec<String>,
    rx_bytes: u64,
    tx_bytes: u64,
}
```

### UserAccess

User access and authentication information.

```rust
#[derive(Debug, Serialize)]
struct UserAccess {
    last_ssh_logins: Vec<String>,
    active_users: Vec<String>,
    sudo_users: Vec<String>,
}
```

### ServiceInfo

System service information.

```rust
#[derive(Debug, Serialize)]
struct ServiceInfo {
    name: String,
    active: bool,
    enabled: bool,
    version: Option<String>,
}
```

### SecurityInfo

Security-related information.

```rust
#[derive(Debug, Serialize)]
struct SecurityInfo {
    firewall_enabled: bool,
    fail2ban_active: bool,
    open_ports: Vec<u16>,
    package_updates: Vec<String>,
}
```

### HardwareInfo

Hardware specifications.

```rust
#[derive(Debug, Serialize)]
struct HardwareInfo {
    cpu_model: String,
    cores: u32,
    total_ram_mb: u64,
    disk_info: Vec<String>,
    system_vendor: String,
    system_model: String,
}
```

### UptimeInfo & RebootRecord

System uptime and reboot history.

```rust
#[derive(Debug, Serialize)]
struct UptimeInfo {
    current_uptime: String,
    last_boot_time: String,
    reboot_history: Vec<RebootRecord>,
}

#[derive(Debug, Serialize)]
struct RebootRecord {
    timestamp: String,
    reason: Option<String>,
}
```

### ProcessInfo

Process information (inferred from usage).

```rust
#[derive(Debug, Serialize)]
struct ProcessInfo {
    pid: u32,
    name: String,
    cpu_usage: f32,
    memory_usage: u64,
    status: String,
    user: String,
    command: String,
}
```

## API Module

### ApiClient

HTTP client for sending system information to remote endpoints.

```rust
pub struct ApiClient {
    client: Client,
    config: ApiConfig,
}
```

#### Methods

##### new
Creates a new API client instance.

```rust
pub fn new(config: ApiConfig) -> Result<Self>
```

**Parameters:**
- `config`: ApiConfig structure containing API settings

**Returns:**
- `Result<Self>`: ApiClient instance or error

**Example:**
```rust
let config = ApiConfig::load()?;
let client = ApiClient::new(config)?;
```

##### send_system_info
Sends system information to the configured API endpoint.

```rust
pub fn send_system_info(&self, info: &SystemInfo) -> Result<()>
```

**Parameters:**
- `info`: Reference to SystemInfo structure

**Returns:**
- `Result<()>`: Success or error

**Example:**
```rust
let system_info = get_system_info();
client.send_system_info(&system_info)?;
```

##### get_system_info
Retrieves system information from the API endpoint.

```rust
pub fn get_system_info(&self) -> Result<SystemInfo>
```

**Returns:**
- `Result<SystemInfo>`: SystemInfo from remote server or error

**Example:**
```rust
let remote_info = client.get_system_info()?;
println!("Remote hostname: {}", remote_info.hostname);
```

## Configuration Module

### ApiConfig

Configuration structure for API settings.

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiConfig {
    pub base_url: String,
    pub api_key: String,
    pub timeout_seconds: u64,
    pub retry_count: u32,
    pub rate_limit: u32,
}
```

#### Default Values
```rust
base_url: "http://localhost:8080"
api_key: ""
timeout_seconds: 30
retry_count: 3
rate_limit: 100
```

#### Methods

##### load
Loads configuration from file and environment variables.

```rust
pub fn load() -> Result<Self>
```

**Configuration Priority:**
1. Environment variables (prefix: `STAFFMON_`)
2. `config.toml` file
3. Default values

**Example config.toml:**
```toml
base_url = "https://api.example.com"
api_key = "your-api-key"
timeout_seconds = 60
retry_count = 5
rate_limit = 50
```

**Environment Variables:**
```bash
export STAFFMON_BASE_URL="https://api.example.com"
export STAFFMON_API_KEY="your-api-key"
```

## Logging Module

### init_logger

Initializes the logging system.

```rust
pub fn init_logger() -> Result<(), Box<dyn std::error::Error>>
```

**Features:**
- Creates `logs` directory if not exists
- Writes logs to `logs/staffmon.log`
- Log format: `{timestamp} [{level}] {message}`
- Default log level: INFO

**Example:**
```rust
log_config::init_logger()?;
log::info!("Application started");
```

## Core Functions

### System Information Gathering

#### get_system_info
Main function that collects all system information.

```rust
fn get_system_info() -> SystemInfo
```

**Returns:** Complete SystemInfo structure

#### get_cpu_info
Collects CPU usage and frequency information.

```rust
fn get_cpu_info(sys: &System) -> CpuInfo
```

#### get_memory_info
Collects memory usage statistics.

```rust
fn get_memory_info(sys: &System) -> MemoryInfo
```

#### get_load_average
Retrieves system load averages.

```rust
fn get_load_average() -> LoadAverage
```

#### get_disk_info
Collects disk usage information for all mounted filesystems.

```rust
fn get_disk_info(sys: &System) -> Vec<DiskInfo>
```

#### get_network_info
Gathers network interface information including IP addresses and traffic.

```rust
fn get_network_info(sys: &System) -> NetworkInfo
```

#### get_user_access
Collects user authentication and access information.

```rust
fn get_user_access() -> UserAccess
```

#### get_services
Lists system services with their status.

```rust
fn get_services() -> Vec<ServiceInfo>
```

#### get_security_info
Gathers security-related information.

```rust
fn get_security_info() -> SecurityInfo
```

#### get_hardware_info
Collects hardware specifications.

```rust
fn get_hardware_info() -> HardwareInfo
```

#### get_uptime_info
Retrieves system uptime and reboot history.

```rust
fn get_uptime_info() -> UptimeInfo
```

### Package Management

#### detect_package_manager
Detects the system's package manager.

```rust
fn detect_package_manager() -> PackageManager
```

**Supported Package Managers:**
- Apt (Debian/Ubuntu)
- Yum (RHEL/CentOS)
- Dnf (Fedora)
- Pacman (Arch)
- Zypper (openSUSE)

#### get_package_updates
Retrieves list of available package updates.

```rust
fn get_package_updates() -> Vec<String>
```

### Service Management

#### get_service_status
Checks if a service is active and enabled.

```rust
fn get_service_status(service: &str) -> (bool, bool)
```

**Returns:** `(active, enabled)` tuple

#### get_service_version
Attempts to determine service version.

```rust
fn get_service_version(service: &str) -> Option<String>
```

### Data Persistence

#### save_to_json
Saves system information to a JSON file.

```rust
fn save_to_json(info: &SystemInfo) -> io::Result<()>
```

**File naming:** `system_info_YYYYMMDD_HHMMSS.json`

### Main Application Functions

#### run_monitor
Main monitoring loop that continuously collects and sends data.

```rust
fn run_monitor() -> Result<()>
```

**Features:**
- Loads API configuration
- Collects system info every 2 seconds
- Saves to JSON file
- Sends to remote API
- Logs all operations

## Usage Examples

### Basic Usage

```rust
use staffmon::{get_system_info, save_to_json};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    log_config::init_logger()?;
    
    // Collect system information
    let info = get_system_info();
    
    // Save to JSON
    save_to_json(&info)?;
    
    println!("System monitoring data collected");
    Ok(())
}
```

### API Integration

```rust
use staffmon::{ApiClient, ApiConfig, get_system_info};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config = ApiConfig::load()?;
    
    // Create API client
    let client = ApiClient::new(config)?;
    
    // Collect and send data
    let info = get_system_info();
    client.send_system_info(&info)?;
    
    Ok(())
}
```

### Custom Configuration

```rust
use staffmon::ApiConfig;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create custom configuration
    let config = ApiConfig {
        base_url: "https://monitoring.example.com".to_string(),
        api_key: "secret-key".to_string(),
        timeout_seconds: 45,
        retry_count: 5,
        rate_limit: 60,
    };
    
    // Use configuration...
    Ok(())
}
```

### Daemon Mode

The application runs as a daemon by default:

```rust
fn main() -> Result<()> {
    // Initialize logging
    log_config::init_logger()?;
    
    // Configure daemon
    let daemonize = Daemonize::new()
        .pid_file("/tmp/staffmon.pid")
        .chown_pid_file(true)
        .working_directory(".")
        .user("nobody")
        .group("nobody")
        .umask(0o027);

    // Start daemon
    match daemonize.start() {
        Ok(_) => run_monitor(),
        Err(e) => Err(e.into()),
    }
}
```

## Error Handling

All functions return `Result` types for proper error handling:

```rust
// Handle API errors
match client.send_system_info(&info) {
    Ok(_) => log::info!("Data sent successfully"),
    Err(e) => log::error!("Failed to send data: {}", e),
}

// Handle configuration errors
let config = ApiConfig::load()
    .map_err(|e| {
        log::error!("Configuration error: {}", e);
        e
    })?;
```

## Security Considerations

1. **API Keys**: Store API keys in environment variables or secure configuration files
2. **Permissions**: The daemon runs as `nobody:nobody` for security
3. **File Access**: Log files are created with restricted permissions (umask 0o027)
4. **Network**: Supports HTTPS for secure data transmission

## Performance Considerations

1. **Sampling Rate**: Default 2-second interval between collections
2. **Timeout**: Configurable API timeout (default 30 seconds)
3. **Retry Logic**: Configurable retry count for failed API calls
4. **Rate Limiting**: Built-in rate limiting support

## Troubleshooting

### Common Issues

1. **Permission Denied**: Ensure the user has access to system commands
2. **API Connection Failed**: Check network connectivity and API configuration
3. **Missing Commands**: Some features require specific system commands (e.g., `dmidecode`)
4. **Service Detection**: Different init systems may affect service detection

### Debug Logging

Enable debug logging by modifying the log configuration:

```rust
.build(Root::builder().appender("file").build(log::LevelFilter::Debug))?;
```

## Version Compatibility

- Rust: 1.70.0 or higher
- Linux: Kernel 3.10 or higher
- Systemd: Optional, falls back to init.d or rc.d