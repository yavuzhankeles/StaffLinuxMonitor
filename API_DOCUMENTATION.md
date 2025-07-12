# StaffLinuxMonitor - API Documentation

**Version:** 1.0.2  
**Language:** Rust  
**Platform:** Linux (Ubuntu, CentOS, RHEL, Debian)  

## Table of Contents

1. [Overview](#overview)
2. [Data Structures](#data-structures)
3. [Core Functions](#core-functions)
4. [API Client](#api-client)
5. [Configuration](#configuration)
6. [Logging](#logging)
7. [Usage Examples](#usage-examples)
8. [Error Handling](#error-handling)

---

## Overview

StaffLinuxMonitor is a comprehensive Linux system monitoring tool that provides real-time system information including CPU, memory, disk, network, services, security, and hardware data. The tool can run as a daemon and provides both JSON output and REST API integration.

### Key Features
- Real-time system monitoring (2-second intervals)
- JSON file output
- REST API integration  
- Daemon mode operation
- Multi-platform package manager support
- Service status monitoring
- Security information gathering
- Hardware information collection

---

## Data Structures

### SystemInfo

Main container for all system information.

```rust
#[derive(Debug, Serialize)]
pub struct SystemInfo {
    pub cpu: CpuInfo,
    pub memory: MemoryInfo,
    pub load_avg: LoadAverage,
    pub disks: Vec<DiskInfo>,
    pub network: NetworkInfo,
    pub user_access: UserAccess,
    pub services: Vec<ServiceInfo>,
    pub security: SecurityInfo,
    pub hardware: HardwareInfo,
    pub system_uptime: UptimeInfo,
    pub hostname: String,
    pub kernel_version: String,
    pub os_version: String,
    pub process_list: Vec<ProcessInfo>,
    pub timestamp: String,  // RFC3339 format
}
```

**Usage:**
```rust
let system_info = get_system_info();
println!("Hostname: {}", system_info.hostname);
println!("CPU Usage: {}%", system_info.cpu.usage_percent);
```

---

### CpuInfo

CPU performance and status information.

```rust
#[derive(Debug, Serialize)]
pub struct CpuInfo {
    pub usage_percent: f32,           // Current CPU usage percentage
    pub temperature_celsius: Option<f32>, // CPU temperature (if available)
    pub frequency_mhz: f32,           // Current CPU frequency in MHz
}
```

**Example JSON Output:**
```json
{
  "usage_percent": 45.2,
  "temperature_celsius": null,
  "frequency_mhz": 2400.0
}
```

---

### MemoryInfo

System memory usage statistics.

```rust
#[derive(Debug, Serialize)]
pub struct MemoryInfo {
    pub total_mb: u64,    // Total memory in MB
    pub used_mb: u64,     // Used memory in MB  
    pub free_mb: u64,     // Free memory in MB
}
```

**Example JSON Output:**
```json
{
  "total_mb": 16384,
  "used_mb": 8192,
  "free_mb": 8192
}
```

---

### LoadAverage

System load averages.

```rust
#[derive(Debug, Serialize)]
pub struct LoadAverage {
    pub one: f64,      // 1-minute load average
    pub five: f64,     // 5-minute load average
    pub fifteen: f64,  // 15-minute load average
}
```

---

### DiskInfo

Disk usage information for each mounted filesystem.

```rust
#[derive(Debug, Serialize)]
pub struct DiskInfo {
    pub name: String,         // Disk device name (e.g., "/dev/sda1")
    pub total_gb: f64,        // Total disk space in GB
    pub used_gb: f64,         // Used disk space in GB
    pub free_gb: f64,         // Free disk space in GB
    pub mount_point: String,  // Mount point (e.g., "/", "/home")
}
```

**Example JSON Output:**
```json
{
  "name": "/dev/sda1",
  "total_gb": 500.0,
  "used_gb": 250.0,
  "free_gb": 250.0,
  "mount_point": "/"
}
```

---

### NetworkInfo & NetworkInterface

Network interface information and statistics.

```rust
#[derive(Debug, Serialize)]
pub struct NetworkInfo {
    pub interfaces: Vec<NetworkInterface>,
}

#[derive(Debug, Serialize)]
pub struct NetworkInterface {
    pub name: String,                // Interface name (e.g., "eth0", "wlan0")
    pub ip_addresses: Vec<String>,   // List of IP addresses (IPv4 and IPv6)
    pub rx_bytes: u64,              // Bytes received
    pub tx_bytes: u64,              // Bytes transmitted
}
```

**Example JSON Output:**
```json
{
  "interfaces": [
    {
      "name": "eth0",
      "ip_addresses": ["192.168.1.100", "fe80::a00:27ff:fe4e:66a1"],
      "rx_bytes": 1024000,
      "tx_bytes": 512000
    }
  ]
}
```

---

### UserAccess

User access and login information.

```rust
#[derive(Debug, Serialize)]
pub struct UserAccess {
    pub last_ssh_logins: Vec<String>,  // Last 5 SSH login entries
    pub active_users: Vec<String>,     // Currently active users
    pub sudo_users: Vec<String>,       // Users with sudo privileges
}
```

---

### ServiceInfo

System service status information.

```rust
#[derive(Debug, Serialize)]
pub struct ServiceInfo {
    pub name: String,           // Service name
    pub active: bool,           // Whether service is currently active
    pub enabled: bool,          // Whether service is enabled to start on boot
    pub version: Option<String>, // Service version (if detectable)
}
```

**Example JSON Output:**
```json
{
  "name": "nginx",
  "active": true,
  "enabled": true,
  "version": "nginx/1.18.0"
}
```

---

### SecurityInfo

Security-related system information.

```rust
#[derive(Debug, Serialize)]
pub struct SecurityInfo {
    pub firewall_enabled: bool,       // UFW firewall status
    pub fail2ban_active: bool,        // Fail2ban intrusion prevention status
    pub open_ports: Vec<u16>,         // List of open ports
    pub package_updates: Vec<String>, // Available package updates
}
```

---

### HardwareInfo

Hardware specifications and information.

```rust
#[derive(Debug, Serialize)]
pub struct HardwareInfo {
    pub cpu_model: String,        // CPU model name
    pub cores: u32,               // Number of CPU cores
    pub total_ram_mb: u64,        // Total RAM in MB
    pub disk_info: Vec<String>,   // Block device information
    pub system_vendor: String,    // System manufacturer
    pub system_model: String,     // System model
}
```

---

### UptimeInfo & RebootRecord

System uptime and reboot history.

```rust
#[derive(Debug, Serialize)]
pub struct UptimeInfo {
    pub current_uptime: String,              // Current uptime string
    pub last_boot_time: String,              // Last boot timestamp
    pub reboot_history: Vec<RebootRecord>,   // Historical reboot records
}

#[derive(Debug, Serialize)]
pub struct RebootRecord {
    pub timestamp: String,        // Reboot timestamp
    pub reason: Option<String>,   // Reboot reason (if available)
}
```

---

### ProcessInfo

Process information structure.

```rust
#[derive(Debug, Serialize)]
pub struct ProcessInfo {
    pub pid: u32,              // Process ID
    pub name: String,          // Process name
    pub cpu_usage: f32,        // CPU usage percentage
    pub memory_usage: u64,     // Memory usage in bytes
    pub status: String,        // Process status (running, sleeping, etc.)
    pub user: String,          // User ID running the process
    pub command: String,       // Full command line
}
```

**Example JSON Output:**
```json
{
  "pid": 1234,
  "name": "nginx",
  "cpu_usage": 2.5,
  "memory_usage": 52428800,
  "status": "Running",
  "user": "www-data",
  "command": "nginx: master process /usr/sbin/nginx"
}
```

---

### PackageManager

Enumeration of supported package managers.

```rust
#[derive(Debug)]
pub enum PackageManager {
    Apt,      // Debian/Ubuntu apt
    Yum,      // RHEL/CentOS yum
    Dnf,      // Fedora dnf
    Pacman,   // Arch Linux pacman
    Zypper,   // openSUSE zypper
    Unknown,  // Unsupported or undetected
}
```

---

## Core Functions

### System Information Collection

#### `get_system_info() -> SystemInfo`

**Description:** Collects comprehensive system information including all subsystems.

**Returns:** Complete `SystemInfo` structure with current system state.

**Usage:**
```rust
let system_info = get_system_info();
println!("System collected at: {}", system_info.timestamp);
```

---

#### `get_cpu_info(sys: &System) -> CpuInfo`

**Description:** Collects CPU usage, frequency, and temperature information.

**Parameters:**
- `sys: &System` - sysinfo System instance

**Returns:** `CpuInfo` structure with CPU metrics.

**Usage:**
```rust
let mut sys = System::new_all();
sys.refresh_all();
let cpu_info = get_cpu_info(&sys);
println!("CPU Usage: {}%", cpu_info.usage_percent);
```

---

#### `get_memory_info(sys: &System) -> MemoryInfo`

**Description:** Collects system memory usage statistics.

**Parameters:**
- `sys: &System` - sysinfo System instance

**Returns:** `MemoryInfo` structure with memory metrics.

---

#### `get_load_average() -> LoadAverage`

**Description:** Retrieves system load averages using the `uptime` command.

**Returns:** `LoadAverage` structure with 1, 5, and 15-minute load averages.

**Error Handling:** Panics if `uptime` command fails to execute.

---

#### `get_disk_info(sys: &System) -> Vec<DiskInfo>`

**Description:** Collects disk usage information for all mounted filesystems.

**Parameters:**
- `sys: &System` - sysinfo System instance

**Returns:** Vector of `DiskInfo` structures for each disk.

---

#### `get_network_info(sys: &System) -> NetworkInfo`

**Description:** Collects network interface information and statistics.

**Parameters:**
- `sys: &System` - sysinfo System instance

**Returns:** `NetworkInfo` structure containing all network interfaces.

**Note:** Falls back to `ifconfig` if `ip` command is not available.

---

#### `get_user_access() -> UserAccess`

**Description:** Collects user access information including SSH logins and sudo users.

**Returns:** `UserAccess` structure with user information.

**Commands Used:**
- `last` - for SSH login history
- `who` - for active users
- `getent group sudo` - for sudo users

---

#### `get_services() -> Vec<ServiceInfo>`

**Description:** Collects system service status information.

**Returns:** Vector of `ServiceInfo` structures for all services.

**Supported Init Systems:**
- systemd (via `systemctl`)
- SysV init (via `service`)
- OpenRC (via rc.d scripts)

---

#### `get_security_info() -> SecurityInfo`

**Description:** Collects security-related system information.

**Returns:** `SecurityInfo` structure with security metrics.

**Checks:**
- UFW firewall status
- Fail2ban status
- Open ports (via `ss -tuln`)
- Available package updates

---

#### `get_hardware_info() -> HardwareInfo`

**Description:** Collects hardware specifications and system information.

**Returns:** `HardwareInfo` structure with hardware details.

**Commands Used:**
- `lscpu` - CPU information
- `nproc` - CPU core count
- `free -m` - Memory information
- `lsblk` - Block device information
- `dmidecode` - System vendor/model

---

### Package Management

#### `detect_package_manager() -> PackageManager`

**Description:** Detects the system's package manager.

**Returns:** `PackageManager` enum value.

**Detection Order:**
1. apt (Debian/Ubuntu)
2. yum (RHEL/CentOS)  
3. dnf (Fedora)
4. pacman (Arch)
5. zypper (openSUSE)

**Usage:**
```rust
let pkg_mgr = detect_package_manager();
match pkg_mgr {
    PackageManager::Apt => println!("Debian/Ubuntu system"),
    PackageManager::Yum => println!("RHEL/CentOS system"),
    PackageManager::Unknown => println!("Unsupported package manager"),
    _ => println!("Other supported system"),
}
```

---

#### `get_package_updates() -> Vec<String>`

**Description:** Retrieves list of available package updates.

**Returns:** Vector of package update strings.

**Package Manager Commands:**
- **apt:** `apt list --upgradable`
- **yum:** `yum check-update`
- **dnf:** `dnf check-update`
- **pacman:** `pacman -Qu`
- **zypper:** `zypper list-updates`

---

### Service Management

#### `get_service_status(service: &str) -> (bool, bool)`

**Description:** Gets the active and enabled status of a specific service.

**Parameters:**
- `service: &str` - Service name

**Returns:** Tuple of (active, enabled) boolean values.

**Service Name Variants:** Automatically tries common variants (e.g., mysql, mariadb, mysqld).

---

#### `get_service_version(service: &str) -> Option<String>`

**Description:** Attempts to retrieve the version of a specific service.

**Parameters:**
- `service: &str` - Service name

**Returns:** Optional version string.

**Version Commands Tried:**
- `{service} --version`
- `{service} -v`
- `{service} -V`
- `{service}-server --version`
- `{service}-cli --version`

---

### Utility Functions

#### `get_uptime_info() -> UptimeInfo`

**Description:** Collects system uptime and reboot history.

**Returns:** `UptimeInfo` structure with uptime details.

---

#### `get_reboot_history() -> Vec<RebootRecord>`

**Description:** Retrieves system reboot history from multiple sources.

**Returns:** Vector of `RebootRecord` structures.

**Sources:**
- `last reboot` command
- `journalctl --list-boots` (systemd systems)

---

#### `save_to_json(info: &SystemInfo) -> io::Result<()>`

**Description:** Saves system information to a timestamped JSON file.

**Parameters:**
- `info: &SystemInfo` - System information to save

**Returns:** IO Result indicating success or failure.

**File Format:** `system_info_{YYYYMMDD_HHMMSS}.json`

---

#### `run_monitor() -> Result<()>`

**Description:** Main monitoring loop that continuously collects and saves system information.

**Returns:** Result indicating success or error.

**Behavior:**
- Loads API configuration
- Creates API client
- Runs continuous monitoring loop (2-second intervals)
- Saves JSON files
- Sends data to API endpoint

---

## API Client

### ApiClient

HTTP client for interacting with external APIs.

```rust
pub struct ApiClient {
    client: Client,
    config: ApiConfig,
}
```

#### `ApiClient::new(config: ApiConfig) -> Result<Self>`

**Description:** Creates a new API client with the specified configuration.

**Parameters:**
- `config: ApiConfig` - API configuration

**Returns:** Result containing ApiClient or error.

**Usage:**
```rust
let config = ApiConfig::load()?;
let client = ApiClient::new(config)?;
```

---

#### `send_system_info(&self, info: &SystemInfo) -> Result<()>`

**Description:** Sends system information to the configured API endpoint.

**Parameters:**
- `info: &SystemInfo` - System information to send

**Returns:** Result indicating success or failure.

**HTTP Details:**
- **Method:** POST
- **Endpoint:** `{base_url}/api/v1/system-info`
- **Headers:** `X-API-Key: {api_key}`
- **Content-Type:** `application/json`

**Usage:**
```rust
let system_info = get_system_info();
client.send_system_info(&system_info)?;
```

---

#### `get_system_info(&self) -> Result<SystemInfo>`

**Description:** Retrieves system information from the configured API endpoint.

**Returns:** Result containing SystemInfo or error.

**HTTP Details:**
- **Method:** GET
- **Endpoint:** `{base_url}/api/v1/system-info`
- **Headers:** `X-API-Key: {api_key}`

---

## Configuration

### ApiConfig

Configuration structure for API client settings.

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiConfig {
    pub base_url: String,       // API base URL
    pub api_key: String,        // API authentication key
    pub timeout_seconds: u64,   // Request timeout
    pub retry_count: u32,       // Number of retries
    pub rate_limit: u32,        // Rate limit per minute
}
```

#### `ApiConfig::load() -> Result<Self>`

**Description:** Loads API configuration from multiple sources.

**Returns:** Result containing ApiConfig or error.

**Configuration Sources (in order):**
1. `config.toml` file
2. Environment variables (prefixed with `STAFFMON_`)
3. `.env` file (via dotenv)
4. Default values

**Default Values:**
```rust
ApiConfig {
    base_url: "http://localhost:8080".to_string(),
    api_key: "".to_string(),
    timeout_seconds: 30,
    retry_count: 3,
    rate_limit: 100,
}
```

**Usage:**
```rust
let config = ApiConfig::load()?;
println!("API URL: {}", config.base_url);
```

**Environment Variables:**
- `STAFFMON_BASE_URL`
- `STAFFMON_API_KEY`
- `STAFFMON_TIMEOUT_SECONDS`
- `STAFFMON_RETRY_COUNT`
- `STAFFMON_RATE_LIMIT`

---

## Logging

### Logger Initialization

#### `init_logger() -> Result<(), Box<dyn std::error::Error>>`

**Description:** Initializes the logging system with file output.

**Returns:** Result indicating success or error.

**Configuration:**
- **Log Directory:** `./logs/`
- **Log File:** `logs/staffmon.log`
- **Log Format:** `{YYYY-MM-DD HH:MM:SS} [LEVEL] MESSAGE`
- **Log Level:** Info and above

**Usage:**
```rust
log_config::init_logger()?;
log::info!("Application started");
```

---

## Usage Examples

### Basic System Information Collection

```rust
use staffmon::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger
    log_config::init_logger()?;
    
    // Get system information
    let system_info = get_system_info();
    
    // Print basic info
    println!("Hostname: {}", system_info.hostname);
    println!("OS: {}", system_info.os_version);
    println!("Kernel: {}", system_info.kernel_version);
    println!("CPU Usage: {}%", system_info.cpu.usage_percent);
    println!("Memory Usage: {}/{} MB", 
             system_info.memory.used_mb, 
             system_info.memory.total_mb);
    
    // Save to JSON
    save_to_json(&system_info)?;
    
    Ok(())
}
```

### API Integration Example

```rust
use staffmon::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config = config::ApiConfig::load()?;
    
    // Create API client
    let client = api::ApiClient::new(config)?;
    
    // Get system information
    let system_info = get_system_info();
    
    // Send to API
    client.send_system_info(&system_info)?;
    
    println!("System information sent to API successfully");
    
    Ok(())
}
```

### Continuous Monitoring Example

```rust
use staffmon::*;
use std::time::Duration;
use std::thread;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    log_config::init_logger()?;
    
    let config = config::ApiConfig::load()?;
    let client = api::ApiClient::new(config)?;
    
    loop {
        let system_info = get_system_info();
        
        // Log current stats
        log::info!("CPU: {}%, Memory: {}%, Load: {}", 
                   system_info.cpu.usage_percent,
                   (system_info.memory.used_mb as f64 / system_info.memory.total_mb as f64) * 100.0,
                   system_info.load_avg.one);
        
        // Save and send
        save_to_json(&system_info)?;
        client.send_system_info(&system_info)?;
        
        // Wait 2 seconds
        thread::sleep(Duration::from_secs(2));
    }
}
```

### Service Status Checking

```rust
use staffmon::*;

fn check_critical_services() {
    let critical_services = vec!["nginx", "mysql", "redis", "ssh"];
    
    for service_name in critical_services {
        let (active, enabled) = get_service_status(service_name);
        let version = get_service_version(service_name);
        
        println!("Service: {}", service_name);
        println!("  Active: {}", active);
        println!("  Enabled: {}", enabled);
        if let Some(v) = version {
            println!("  Version: {}", v);
        }
        
        if !active {
            log::warn!("Critical service {} is not active!", service_name);
        }
    }
}
```

### Package Update Checking

```rust
use staffmon::*;

fn check_updates() {
    let pkg_mgr = detect_package_manager();
    println!("Package Manager: {:?}", pkg_mgr);
    
    let updates = get_package_updates();
    
    if updates.is_empty() {
        println!("System is up to date");
    } else {
        println!("Available updates: {}", updates.len());
        for update in updates.iter().take(10) {
            println!("  {}", update);
        }
        if updates.len() > 10 {
            println!("  ... and {} more", updates.len() - 10);
        }
    }
}
```

---

## Error Handling

The library uses Rust's standard `Result` type for error handling. Common error types include:

### API Errors
- **Network errors:** Connection timeouts, DNS resolution failures
- **Authentication errors:** Invalid API keys
- **HTTP errors:** 4xx/5xx status codes

### System Errors
- **Command execution failures:** Missing system commands
- **Permission errors:** Insufficient privileges for system information
- **File I/O errors:** JSON file creation/writing failures

### Configuration Errors
- **Invalid configuration:** Malformed TOML files
- **Missing configuration:** Required environment variables not set

### Example Error Handling

```rust
use staffmon::*;

fn safe_monitoring() {
    match get_system_info() {
        Ok(info) => {
            if let Err(e) = save_to_json(&info) {
                log::error!("Failed to save JSON: {}", e);
            }
            
            // Try API upload with error handling
            match config::ApiConfig::load() {
                Ok(config) => {
                    match api::ApiClient::new(config) {
                        Ok(client) => {
                            if let Err(e) = client.send_system_info(&info) {
                                log::error!("Failed to send to API: {}", e);
                            }
                        }
                        Err(e) => log::error!("Failed to create API client: {}", e),
                    }
                }
                Err(e) => log::error!("Failed to load config: {}", e),
            }
        }
        Err(e) => log::error!("Failed to get system info: {}", e),
    }
}
```

---

## Performance Notes

- **Memory Usage:** Approximately 5MB RAM during operation
- **CPU Impact:** Minimal (~2% CPU usage during collection)
- **Update Frequency:** Default 2-second intervals (configurable)
- **File Output:** Timestamped JSON files with pretty formatting
- **Network:** Configurable timeouts and retry mechanisms

---

## Platform Support

### Tested Distributions
- Ubuntu 18.04+
- Debian 9+
- CentOS 7+
- RHEL 7+
- Fedora 28+

### Architecture Support
- x86_64 (primary)
- ARM64 (supported)

### Requirements
- Linux kernel 3.10+
- Standard GNU utilities (`uptime`, `free`, `lscpu`, etc.)
- Network access for API functionality (optional)

---

This documentation covers all public APIs, functions, and components of the StaffLinuxMonitor system. For additional examples and deployment information, refer to the existing documentation in the `docs/` directory.