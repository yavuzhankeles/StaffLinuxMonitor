# StaffMon API Reference Documentation

## Table of Contents

1. [Overview](#overview)
2. [Data Structures](#data-structures)
3. [Core Functions](#core-functions)
4. [API Client](#api-client)
5. [Configuration](#configuration)
6. [Logging](#logging)
7. [Examples](#examples)
8. [Error Handling](#error-handling)

## Overview

StaffMon is a comprehensive Linux system monitoring tool written in Rust. It provides real-time system information collection, monitoring, and reporting capabilities through both local JSON output and REST API integration.

### Key Features

- **System Information Collection**: CPU, memory, disk, network, and process monitoring
- **Service Management**: Service status and version detection
- **Security Monitoring**: Firewall status, open ports, and package updates
- **User Access Tracking**: SSH logins, active users, and sudo access
- **Hardware Information**: Detailed hardware specifications
- **Uptime and Reboot History**: System uptime tracking and reboot analysis
- **API Integration**: REST API client for remote monitoring
- **Daemon Mode**: Background service operation
- **Comprehensive Logging**: Structured logging with file output

## Data Structures

### SystemInfo

The main data structure containing all system information.

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

**Fields:**
- `cpu`: CPU usage and performance information
- `memory`: Memory usage statistics
- `load_avg`: System load average (1, 5, 15 minute averages)
- `disks`: List of disk partitions and usage
- `network`: Network interface information
- `user_access`: User login and access information
- `services`: System services status
- `security`: Security-related information
- `hardware`: Hardware specifications
- `system_uptime`: System uptime and reboot history
- `hostname`: System hostname
- `kernel_version`: Linux kernel version
- `os_version`: Operating system version
- `process_list`: List of running processes
- `timestamp`: ISO 8601 timestamp of data collection

### CpuInfo

CPU performance and usage information.

```rust
#[derive(Debug, Serialize)]
struct CpuInfo {
    usage_percent: f32,
    temperature_celsius: Option<f32>,
    frequency_mhz: f32,
}
```

**Fields:**
- `usage_percent`: CPU usage percentage (0.0 - 100.0)
- `temperature_celsius`: CPU temperature in Celsius (if available)
- `frequency_mhz`: Current CPU frequency in MHz

### MemoryInfo

Memory usage statistics.

```rust
#[derive(Debug, Serialize)]
struct MemoryInfo {
    total_mb: u64,
    used_mb: u64,
    free_mb: u64,
}
```

**Fields:**
- `total_mb`: Total memory in MB
- `used_mb`: Used memory in MB
- `free_mb`: Free memory in MB

### LoadAverage

System load average information.

```rust
#[derive(Debug, Serialize)]
struct LoadAverage {
    one: f64,
    five: f64,
    fifteen: f64,
}
```

**Fields:**
- `one`: 1-minute load average
- `five`: 5-minute load average
- `fifteen`: 15-minute load average

### DiskInfo

Disk partition information.

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

**Fields:**
- `name`: Device name (e.g., "/dev/sda1")
- `total_gb`: Total disk space in GB
- `used_gb`: Used disk space in GB
- `free_gb`: Free disk space in GB
- `mount_point`: Mount point path

### NetworkInfo

Network interface information.

```rust
#[derive(Debug, Serialize)]
struct NetworkInfo {
    interfaces: Vec<NetworkInterface>,
}
```

### NetworkInterface

Individual network interface details.

```rust
#[derive(Debug, Serialize)]
struct NetworkInterface {
    name: String,
    ip_addresses: Vec<String>,
    rx_bytes: u64,
    tx_bytes: u64,
}
```

**Fields:**
- `name`: Interface name (e.g., "eth0", "wlan0")
- `ip_addresses`: List of IP addresses (IPv4 and IPv6)
- `rx_bytes`: Received bytes
- `tx_bytes`: Transmitted bytes

### UserAccess

User access and login information.

```rust
#[derive(Debug, Serialize)]
struct UserAccess {
    last_ssh_logins: Vec<String>,
    active_users: Vec<String>,
    sudo_users: Vec<String>,
}
```

**Fields:**
- `last_ssh_logins`: Recent SSH login entries
- `active_users`: Currently logged-in users
- `sudo_users`: Users with sudo privileges

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

**Fields:**
- `name`: Service name
- `active`: Whether the service is currently running
- `enabled`: Whether the service is enabled to start on boot
- `version`: Service version (if available)

### SecurityInfo

Security-related system information.

```rust
#[derive(Debug, Serialize)]
struct SecurityInfo {
    firewall_enabled: bool,
    fail2ban_active: bool,
    open_ports: Vec<u16>,
    package_updates: Vec<String>,
}
```

**Fields:**
- `firewall_enabled`: Whether UFW firewall is active
- `fail2ban_active`: Whether fail2ban is running
- `open_ports`: List of open network ports
- `package_updates`: Available package updates

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

**Fields:**
- `cpu_model`: CPU model name
- `cores`: Number of CPU cores
- `total_ram_mb`: Total RAM in MB
- `disk_info`: Disk device information
- `system_vendor`: System manufacturer
- `system_model`: System model

### UptimeInfo

System uptime and reboot information.

```rust
#[derive(Debug, Serialize)]
struct UptimeInfo {
    current_uptime: String,
    last_boot_time: String,
    reboot_history: Vec<RebootRecord>,
}
```

**Fields:**
- `current_uptime`: Current system uptime string
- `last_boot_time`: Last system boot time
- `reboot_history`: History of system reboots

### RebootRecord

Individual reboot record.

```rust
#[derive(Debug, Serialize)]
struct RebootRecord {
    timestamp: String,
    reason: Option<String>,
}
```

**Fields:**
- `timestamp`: Reboot timestamp
- `reason`: Reboot reason (if available)

### ProcessInfo

Process information (implied from usage).

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

**Fields:**
- `pid`: Process ID
- `name`: Process name
- `cpu_usage`: CPU usage percentage
- `memory_usage`: Memory usage in bytes
- `status`: Process status
- `user`: Process owner
- `command`: Full command line

### PackageManager

Supported package managers.

```rust
#[derive(Debug)]
enum PackageManager {
    Apt,
    Yum,
    Dnf,
    Pacman,
    Zypper,
    Unknown,
}
```

## Core Functions

### System Information Collection

#### `get_system_info() -> SystemInfo`

Collects comprehensive system information from all available sources.

**Returns:** Complete system information structure

**Example:**
```rust
let system_info = get_system_info();
println!("Hostname: {}", system_info.hostname);
println!("CPU Usage: {:.1}%", system_info.cpu.usage_percent);
```

#### `get_cpu_info(sys: &System) -> CpuInfo`

Collects CPU-specific information.

**Parameters:**
- `sys`: System instance from sysinfo crate

**Returns:** CPU information structure

**Example:**
```rust
let mut sys = System::new_all();
sys.refresh_all();
let cpu_info = get_cpu_info(&sys);
println!("CPU Frequency: {} MHz", cpu_info.frequency_mhz);
```

#### `get_memory_info(sys: &System) -> MemoryInfo`

Collects memory usage information.

**Parameters:**
- `sys`: System instance from sysinfo crate

**Returns:** Memory information structure

**Example:**
```rust
let mut sys = System::new_all();
sys.refresh_all();
let mem_info = get_memory_info(&sys);
println!("Memory Usage: {}/{} MB", mem_info.used_mb, mem_info.total_mb);
```

#### `get_load_average() -> LoadAverage`

Gets system load average information.

**Returns:** Load average structure

**Example:**
```rust
let load_avg = get_load_average();
println!("1-min load: {:.2}", load_avg.one);
```

#### `get_disk_info(sys: &System) -> Vec<DiskInfo>`

Collects disk partition information.

**Parameters:**
- `sys`: System instance from sysinfo crate

**Returns:** Vector of disk information structures

**Example:**
```rust
let mut sys = System::new_all();
sys.refresh_all();
let disks = get_disk_info(&sys);
for disk in &disks {
    println!("{}: {:.1}GB used of {:.1}GB", disk.name, disk.used_gb, disk.total_gb);
}
```

#### `get_network_info(sys: &System) -> NetworkInfo`

Collects network interface information.

**Parameters:**
- `sys`: System instance from sysinfo crate

**Returns:** Network information structure

**Example:**
```rust
let mut sys = System::new_all();
sys.refresh_all();
let net_info = get_network_info(&sys);
for interface in &net_info.interfaces {
    println!("{}: {:?}", interface.name, interface.ip_addresses);
}
```

### Service Management

#### `get_services() -> Vec<ServiceInfo>`

Collects information about system services.

**Returns:** Vector of service information structures

**Example:**
```rust
let services = get_services();
for service in &services {
    if service.active {
        println!("{} is running", service.name);
    }
}
```

#### `get_service_status(service: &str) -> (bool, bool)`

Checks the status of a specific service.

**Parameters:**
- `service`: Service name to check

**Returns:** Tuple of (is_active, is_enabled)

**Example:**
```rust
let (active, enabled) = get_service_status("nginx");
println!("Nginx active: {}, enabled: {}", active, enabled);
```

#### `get_service_version(service: &str) -> Option<String>`

Gets the version of a specific service.

**Parameters:**
- `service`: Service name

**Returns:** Service version string or None

**Example:**
```rust
if let Some(version) = get_service_version("nginx") {
    println!("Nginx version: {}", version);
}
```

### Security Functions

#### `get_security_info() -> SecurityInfo`

Collects security-related system information.

**Returns:** Security information structure

**Example:**
```rust
let security = get_security_info();
println!("Firewall enabled: {}", security.firewall_enabled);
println!("Open ports: {:?}", security.open_ports);
```

#### `get_package_updates() -> Vec<String>`

Gets available package updates.

**Returns:** Vector of package names with available updates

**Example:**
```rust
let updates = get_package_updates();
println!("Available updates: {}", updates.len());
for update in &updates {
    println!("  {}", update);
}
```

#### `detect_package_manager() -> PackageManager`

Detects the system's package manager.

**Returns:** Package manager enumeration

**Example:**
```rust
match detect_package_manager() {
    PackageManager::Apt => println!("Using APT package manager"),
    PackageManager::Yum => println!("Using YUM package manager"),
    PackageManager::Dnf => println!("Using DNF package manager"),
    PackageManager::Pacman => println!("Using Pacman package manager"),
    PackageManager::Zypper => println!("Using Zypper package manager"),
    PackageManager::Unknown => println!("Unknown package manager"),
}
```

### User and Access Functions

#### `get_user_access() -> UserAccess`

Collects user access and login information.

**Returns:** User access information structure

**Example:**
```rust
let user_access = get_user_access();
println!("Active users: {:?}", user_access.active_users);
println!("Recent SSH logins: {:?}", user_access.last_ssh_logins);
```

### Hardware Functions

#### `get_hardware_info() -> HardwareInfo`

Collects hardware specifications.

**Returns:** Hardware information structure

**Example:**
```rust
let hardware = get_hardware_info();
println!("CPU: {}", hardware.cpu_model);
println!("Cores: {}", hardware.cores);
println!("RAM: {} MB", hardware.total_ram_mb);
```

### Uptime Functions

#### `get_uptime_info() -> UptimeInfo`

Collects system uptime information.

**Returns:** Uptime information structure

**Example:**
```rust
let uptime = get_uptime_info();
println!("Current uptime: {}", uptime.current_uptime);
println!("Last boot: {}", uptime.last_boot_time);
```

#### `get_reboot_history() -> Vec<RebootRecord>`

Gets system reboot history.

**Returns:** Vector of reboot records

**Example:**
```rust
let reboots = get_reboot_history();
for reboot in &reboots {
    println!("Reboot at {}: {:?}", reboot.timestamp, reboot.reason);
}
```

### Utility Functions

#### `save_to_json(info: &SystemInfo) -> io::Result<()>`

Saves system information to a JSON file.

**Parameters:**
- `info`: System information to save

**Returns:** IO result

**Example:**
```rust
let system_info = get_system_info();
if let Err(e) = save_to_json(&system_info) {
    eprintln!("Failed to save JSON: {}", e);
}
```

#### `run_monitor() -> Result<()>`

Runs the main monitoring loop.

**Returns:** Result indicating success or failure

**Example:**
```rust
if let Err(e) = run_monitor() {
    eprintln!("Monitoring failed: {}", e);
}
```

## API Client

### ApiClient

REST API client for sending system information to remote servers.

#### `ApiClient::new(config: ApiConfig) -> Result<Self>`

Creates a new API client instance.

**Parameters:**
- `config`: API configuration

**Returns:** API client instance or error

**Example:**
```rust
let config = ApiConfig::load()?;
let client = ApiClient::new(config)?;
```

#### `send_system_info(&self, info: &SystemInfo) -> Result<()>`

Sends system information to the API server.

**Parameters:**
- `info`: System information to send

**Returns:** Result indicating success or failure

**Example:**
```rust
let system_info = get_system_info();
client.send_system_info(&system_info)?;
```

#### `get_system_info(&self) -> Result<SystemInfo>`

Retrieves system information from the API server.

**Returns:** System information or error

**Example:**
```rust
let system_info = client.get_system_info()?;
println!("Retrieved system info for: {}", system_info.hostname);
```

## Configuration

### ApiConfig

API client configuration structure.

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

**Fields:**
- `base_url`: API server base URL
- `api_key`: Authentication API key
- `timeout_seconds`: Request timeout in seconds
- `retry_count`: Number of retry attempts
- `rate_limit`: Rate limiting requests per minute

#### `ApiConfig::default() -> Self`

Creates default API configuration.

**Returns:** Default configuration

**Example:**
```rust
let config = ApiConfig::default();
// base_url: "http://localhost:8080"
// timeout_seconds: 30
// retry_count: 3
// rate_limit: 100
```

#### `ApiConfig::load() -> Result<Self>`

Loads configuration from file and environment variables.

**Returns:** Loaded configuration or error

**Configuration Sources:**
1. `config.toml` file
2. Environment variables with `STAFFMON_` prefix
3. `.env` file
4. Default values

**Example:**
```rust
let config = ApiConfig::load()?;
println!("API URL: {}", config.base_url);
```

**Configuration File Example (`config.toml`):**
```toml
base_url = "https://api.example.com"
api_key = "your-api-key-here"
timeout_seconds = 30
retry_count = 3
rate_limit = 100
```

**Environment Variables:**
```bash
export STAFFMON_BASE_URL="https://api.example.com"
export STAFFMON_API_KEY="your-api-key-here"
export STAFFMON_TIMEOUT_SECONDS=30
export STAFFMON_RETRY_COUNT=3
export STAFFMON_RATE_LIMIT=100
```

## Logging

### `init_logger() -> Result<(), Box<dyn std::error::Error>>`

Initializes the logging system.

**Returns:** Result indicating success or failure

**Features:**
- Creates `logs/` directory if it doesn't exist
- Logs to `logs/staffmon.log` file
- Uses pattern: `{d(%Y-%m-%d %H:%M:%S)} [{l}] {m}{n}`
- Log level: Info

**Example:**
```rust
if let Err(e) = log_config::init_logger() {
    eprintln!("Failed to initialize logger: {}", e);
}
```

**Log Format:**
```
2024-01-15 10:30:45 [INFO] StaffMon başlatılıyor...
2024-01-15 10:30:45 [INFO] API yapılandırması yüklendi
2024-01-15 10:30:45 [INFO] API istemcisi oluşturuldu
2024-01-15 10:30:45 [INFO] Sistem izleme başlatıldı
```

## Examples

### Basic System Information Collection

```rust
use staffmon::{get_system_info, save_to_json};

fn main() -> Result<()> {
    // Collect system information
    let system_info = get_system_info();
    
    // Print basic information
    println!("Hostname: {}", system_info.hostname);
    println!("OS: {}", system_info.os_version);
    println!("Kernel: {}", system_info.kernel_version);
    println!("CPU Usage: {:.1}%", system_info.cpu.usage_percent);
    println!("Memory Usage: {}/{} MB", 
             system_info.memory.used_mb, 
             system_info.memory.total_mb);
    
    // Save to JSON file
    save_to_json(&system_info)?;
    
    Ok(())
}
```

### Service Monitoring

```rust
use staffmon::{get_services, get_service_status};

fn main() {
    // Get all services
    let services = get_services();
    
    println!("System Services:");
    for service in &services {
        let status = if service.active { "Running" } else { "Stopped" };
        let enabled = if service.enabled { "Enabled" } else { "Disabled" };
        println!("  {}: {} ({})", service.name, status, enabled);
        
        if let Some(version) = &service.version {
            println!("    Version: {}", version);
        }
    }
    
    // Check specific service
    let (active, enabled) = get_service_status("nginx");
    println!("Nginx - Active: {}, Enabled: {}", active, enabled);
}
```

### Network Monitoring

```rust
use staffmon::{get_system_info, System};

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();
    
    let system_info = get_system_info();
    
    println!("Network Interfaces:");
    for interface in &system_info.network.interfaces {
        println!("  {}:", interface.name);
        println!("    IP Addresses: {:?}", interface.ip_addresses);
        println!("    RX: {} bytes", interface.rx_bytes);
        println!("    TX: {} bytes", interface.tx_bytes);
    }
}
```

### Security Monitoring

```rust
use staffmon::get_security_info;

fn main() {
    let security = get_security_info();
    
    println!("Security Status:");
    println!("  Firewall (UFW): {}", 
             if security.firewall_enabled { "Enabled" } else { "Disabled" });
    println!("  Fail2ban: {}", 
             if security.fail2ban_active { "Active" } else { "Inactive" });
    println!("  Open Ports: {:?}", security.open_ports);
    println!("  Available Updates: {}", security.package_updates.len());
    
    for update in &security.package_updates {
        println!("    {}", update);
    }
}
```

### API Integration

```rust
use staffmon::{ApiClient, ApiConfig, get_system_info};

fn main() -> Result<()> {
    // Load configuration
    let config = ApiConfig::load()?;
    
    // Create API client
    let client = ApiClient::new(config)?;
    
    // Collect and send system information
    let system_info = get_system_info();
    client.send_system_info(&system_info)?;
    
    println!("System information sent to API successfully");
    
    Ok(())
}
```

### Continuous Monitoring

```rust
use staffmon::{run_monitor, log_config};
use std::time::Duration;
use std::thread;

fn main() -> Result<()> {
    // Initialize logging
    log_config::init_logger()?;
    
    // Run monitoring loop
    run_monitor()?;
    
    Ok(())
}
```

### Custom Monitoring Loop

```rust
use staffmon::{get_system_info, save_to_json, ApiClient, ApiConfig};
use std::time::Duration;
use std::thread;

fn main() -> Result<()> {
    let config = ApiConfig::load()?;
    let client = ApiClient::new(config)?;
    
    loop {
        // Collect system information
        let system_info = get_system_info();
        
        // Save to local JSON file
        if let Err(e) = save_to_json(&system_info) {
            eprintln!("Failed to save JSON: {}", e);
        }
        
        // Send to API
        if let Err(e) = client.send_system_info(&system_info) {
            eprintln!("Failed to send to API: {}", e);
        }
        
        // Wait before next collection
        thread::sleep(Duration::from_secs(60)); // 1 minute interval
    }
}
```

## Error Handling

StaffMon uses the `anyhow` crate for error handling, providing a unified error type (`Result<T>`) that can handle various error types.

### Common Error Patterns

```rust
use anyhow::Result;

// Function that can fail
fn process_system_info() -> Result<()> {
    let system_info = get_system_info();
    
    // Handle potential errors
    save_to_json(&system_info)?;
    
    let config = ApiConfig::load()?;
    let client = ApiClient::new(config)?;
    client.send_system_info(&system_info)?;
    
    Ok(())
}

// Error handling in main
fn main() {
    if let Err(e) = process_system_info() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
```

### Error Types

- **IO Errors**: File operations, network requests
- **Configuration Errors**: Missing or invalid configuration files
- **API Errors**: Network timeouts, authentication failures
- **System Errors**: Command execution failures, permission issues

### Logging Errors

```rust
use log::{error, info};

fn monitor_system() -> Result<()> {
    info!("Starting system monitoring");
    
    let system_info = get_system_info();
    
    match save_to_json(&system_info) {
        Ok(_) => info!("System info saved successfully"),
        Err(e) => error!("Failed to save system info: {}", e),
    }
    
    match ApiConfig::load() {
        Ok(config) => {
            match ApiClient::new(config) {
                Ok(client) => {
                    match client.send_system_info(&system_info) {
                        Ok(_) => info!("System info sent to API"),
                        Err(e) => error!("Failed to send to API: {}", e),
                    }
                }
                Err(e) => error!("Failed to create API client: {}", e),
            }
        }
        Err(e) => error!("Failed to load API config: {}", e),
    }
    
    Ok(())
}
```

## Best Practices

### 1. Error Handling
Always handle potential errors from system information collection functions.

### 2. Resource Management
Use appropriate timeouts and retry mechanisms for API calls.

### 3. Logging
Implement comprehensive logging for debugging and monitoring.

### 4. Configuration
Use environment variables for sensitive configuration like API keys.

### 5. Performance
Consider the frequency of data collection to avoid system impact.

### 6. Security
Validate and sanitize all system information before sending to external APIs.

## Dependencies

StaffMon relies on the following key dependencies:

- **sysinfo**: System information collection
- **serde**: Serialization/deserialization
- **reqwest**: HTTP client for API communication
- **tokio**: Async runtime
- **log**: Logging framework
- **anyhow**: Error handling
- **chrono**: Date/time handling
- **daemonize**: Daemon process management

## License

This project is licensed under the MIT License. See the LICENSE file for details.