// Yeni build tetikleme yorumu
// System Monitor - Linux System Monitoring Tool
use serde::Serialize;
use std::process::Command;
use sysinfo::{System, SystemExt, CpuExt, DiskExt, NetworkExt, NetworksExt};
use std::time::Duration;
use std::thread;
use std::io;
use std::fs;
use std::path::Path;
use std::io::Write;
use chrono;
mod config;
mod api;
mod log_config;
use std::collections::HashMap;
use anyhow::Result;
use daemonize::Daemonize;
use log::{info, error, warn};

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

#[derive(Debug, Serialize)]
struct CpuInfo {
    usage_percent: f32,
    temperature_celsius: Option<f32>,
    frequency_mhz: f32,
}

#[derive(Debug, Serialize)]
struct MemoryInfo {
    total_mb: u64,
    used_mb: u64,
    free_mb: u64,
}

#[derive(Debug, Serialize)]
struct LoadAverage {
    one: f64,
    five: f64,
    fifteen: f64,
}

#[derive(Debug, Serialize)]
struct DiskInfo {
    name: String,
    total_gb: f64,
    used_gb: f64,
    free_gb: f64,
    mount_point: String,
}

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

#[derive(Debug, Serialize)]
struct UserAccess {
    last_ssh_logins: Vec<String>,
    active_users: Vec<String>,
    sudo_users: Vec<String>,
}

#[derive(Debug, Serialize)]
struct ServiceInfo {
    name: String,
    active: bool,
    enabled: bool,
    version: Option<String>,
}

#[derive(Debug, Serialize)]
struct SecurityInfo {
    firewall_enabled: bool,
    fail2ban_active: bool,
    open_ports: Vec<u16>,
    package_updates: Vec<String>,
}

#[derive(Debug, Serialize)]
struct HardwareInfo {
    cpu_model: String,
    cores: u32,
    total_ram_mb: u64,
    disk_info: Vec<String>,
    system_vendor: String,
    system_model: String,
}

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

#[derive(Debug)]
enum PackageManager {
    Apt,
    Yum,
    Dnf,
    Pacman,
    Zypper,
    Unknown,
}

fn detect_package_manager() -> PackageManager {
    if Command::new("which").arg("apt").output().map(|o| o.status.success()).unwrap_or(false) {
        PackageManager::Apt
    } else if Command::new("which").arg("yum").output().map(|o| o.status.success()).unwrap_or(false) {
        PackageManager::Yum
    } else if Command::new("which").arg("dnf").output().map(|o| o.status.success()).unwrap_or(false) {
        PackageManager::Dnf
    } else if Command::new("which").arg("pacman").output().map(|o| o.status.success()).unwrap_or(false) {
        PackageManager::Pacman
    } else if Command::new("which").arg("zypper").output().map(|o| o.status.success()).unwrap_or(false) {
        PackageManager::Zypper
    } else {
        PackageManager::Unknown
    }
}

fn get_package_updates() -> Vec<String> {
    let pkg_manager = detect_package_manager();
    
    match pkg_manager {
        PackageManager::Apt => {
            let _ = Command::new("apt")
                .args(["update"])
                .output();
            
            Command::new("apt")
                .args(["list", "--upgradable"])
                .output()
                .map(|output| {
                    String::from_utf8_lossy(&output.stdout)
                        .lines()
                        .skip(1)
                        .map(String::from)
                        .collect()
                })
                .unwrap_or_default()
        },
        PackageManager::Yum => {
            Command::new("yum")
                .args(["check-update"])
                .output()
                .map(|output| {
                    String::from_utf8_lossy(&output.stdout)
                        .lines()
                        .filter(|line| !line.starts_with("Loaded plugins:") && !line.is_empty())
                        .map(String::from)
                        .collect()
                })
                .unwrap_or_default()
        },
        PackageManager::Dnf => {
            Command::new("dnf")
                .args(["check-update"])
                .output()
                .map(|output| {
                    String::from_utf8_lossy(&output.stdout)
                        .lines()
                        .filter(|line| !line.starts_with("Last metadata") && !line.is_empty())
                        .map(String::from)
                        .collect()
                })
                .unwrap_or_default()
        },
        PackageManager::Pacman => {
            Command::new("pacman")
                .args(["-Qu"])
                .output()
                .map(|output| {
                    String::from_utf8_lossy(&output.stdout)
                        .lines()
                        .map(String::from)
                        .collect()
                })
                .unwrap_or_default()
        },
        PackageManager::Zypper => {
            Command::new("zypper")
                .args(["list-updates"])
                .output()
                .map(|output| {
                    String::from_utf8_lossy(&output.stdout)
                        .lines()
                        .filter(|line| !line.starts_with("S") && !line.is_empty())
                        .map(String::from)
                        .collect()
                })
                .unwrap_or_default()
        },
        PackageManager::Unknown => Vec::new(),
    }
}

fn get_service_status(service: &str) -> (bool, bool) {
    // Try different service names for each service
    let service_names = match service {
        "mysql" => vec!["mysql", "mariadb", "mysqld"],
        "redis" => vec!["redis", "redis-server"],
        "nginx" => vec!["nginx", "nginx-main"],
        _ => vec![service],
    };

    for service_name in service_names {
        // systemd kullanılıyorsa
        if Command::new("which").arg("systemctl").output().map(|o| o.status.success()).unwrap_or(false) {
            let status = Command::new("systemctl")
                .args(["is-active", service_name])
                .output()
                .map(|output| String::from_utf8_lossy(&output.stdout).trim() == "active")
                .unwrap_or(false);

            let enabled = Command::new("systemctl")
                .args(["is-enabled", service_name])
                .output()
                .map(|output| String::from_utf8_lossy(&output.stdout).trim() == "enabled")
                .unwrap_or(false);

            if status || enabled {
                return (status, enabled);
            }
        }
        // init.d kullanılıyorsa
        else if Command::new("which").arg("service").output().map(|o| o.status.success()).unwrap_or(false) {
            let status = Command::new("service")
                .args([service_name, "status"])
                .output()
                .map(|output| String::from_utf8_lossy(&output.stdout).contains("running"))
                .unwrap_or(false);

            let enabled = Command::new("chkconfig")
                .args(["--list", service_name])
                .output()
                .map(|output| String::from_utf8_lossy(&output.stdout).contains("on"))
                .unwrap_or(false);

            if status || enabled {
                return (status, enabled);
            }
        }
        // rc.d kullanılıyorsa
        else {
            let status = Command::new(format!("/etc/rc.d/{}", service_name))
                .arg("status")
                .output()
                .map(|output| String::from_utf8_lossy(&output.stdout).contains("running"))
                .unwrap_or(false);

            let enabled = Command::new("rc-update")
                .args(["show", service_name])
                .output()
                .map(|output| String::from_utf8_lossy(&output.stdout).contains("default"))
                .unwrap_or(false);

            if status || enabled {
                return (status, enabled);
            }
        }
    }

    // If no service was found active or enabled, return false
    (false, false)
}

fn get_services() -> Vec<ServiceInfo> {
    let mut services = Vec::new();
    
    // systemd kullanılıyorsa
    if Command::new("which").arg("systemctl").output().map(|o| o.status.success()).unwrap_or(false) {
        if let Ok(output) = Command::new("systemctl")
            .args(["list-units", "--type=service", "--state=loaded"])
            .output() {
            if let Ok(output_str) = String::from_utf8(output.stdout) {
                for line in output_str.lines() {
                    if let Some(service_name) = line.split_whitespace().next() {
                        if !service_name.is_empty() && !service_name.contains("@") {
                            let (active, enabled) = get_service_status(service_name);
                            let version = get_service_version(service_name);
                            
                            services.push(ServiceInfo {
                                name: service_name.to_string(),
                                active,
                                enabled,
                                version,
                            });
                        }
                    }
                }
            }
        }
    }
    // init.d kullanılıyorsa
    else if Command::new("which").arg("service").output().map(|o| o.status.success()).unwrap_or(false) {
        if let Ok(output) = Command::new("ls")
            .args(["/etc/init.d/"])
            .output() {
            if let Ok(output_str) = String::from_utf8(output.stdout) {
                for service_name in output_str.lines() {
                    let service_name = service_name.trim();
                    if !service_name.is_empty() {
                        let (active, enabled) = get_service_status(service_name);
                        let version = get_service_version(service_name);
                        
                        services.push(ServiceInfo {
                            name: service_name.to_string(),
                            active,
                            enabled,
                            version,
                        });
                    }
                }
            }
        }
    }
    // rc.d kullanılıyorsa
    else {
        if let Ok(output) = Command::new("ls")
            .args(["/etc/rc.d/"])
            .output() {
            if let Ok(output_str) = String::from_utf8(output.stdout) {
                for service_name in output_str.lines() {
                    let service_name = service_name.trim();
                    if !service_name.is_empty() {
                        let (active, enabled) = get_service_status(service_name);
                        let version = get_service_version(service_name);
                        
                        services.push(ServiceInfo {
                            name: service_name.to_string(),
                            active,
                            enabled,
                            version,
                        });
                    }
                }
            }
        }
    }

    services
}

fn get_service_version(service: &str) -> Option<String> {
    // Önce servis adından komut adını çıkar
    let cmd_name = service.split('.').next().unwrap_or(service);
    
    // Versiyon komutlarını dene
    let version_commands = vec![
        (cmd_name.to_string(), vec!["--version"]),
        (cmd_name.to_string(), vec!["-v"]),
        (cmd_name.to_string(), vec!["-V"]),
        (format!("{}-server", cmd_name).to_string(), vec!["--version"]),
        (format!("{}-cli", cmd_name).to_string(), vec!["--version"]),
    ];

    for (cmd, args) in version_commands {
        if let Ok(output) = Command::new(&cmd).args(&args).output() {
            // Önce stdout'u kontrol et
            if let Ok(version) = String::from_utf8(output.stdout) {
                if !version.is_empty() {
                    return version.lines().next().map(String::from);
                }
            }
            // Sonra stderr'i kontrol et
            if let Ok(version) = String::from_utf8(output.stderr) {
                if !version.is_empty() {
                    return version.lines().next().map(String::from);
                }
            }
        }
    }

    None
}

fn get_security_info() -> SecurityInfo {
    let firewall_enabled = Command::new("ufw")
        .arg("status")
        .output()
        .map(|output| {
            String::from_utf8_lossy(&output.stdout)
                .contains("Status: active")
        })
        .unwrap_or(false);

    let fail2ban_active = Command::new("fail2ban-client")
        .arg("status")
        .output()
        .map(|output| {
            String::from_utf8_lossy(&output.stdout)
                .contains("Status")
        })
        .unwrap_or(false);

    let open_ports = Command::new("ss")
        .args(["-tuln"])
        .output()
        .map(|output| {
            String::from_utf8_lossy(&output.stdout)
                .lines()
                .filter_map(|line| {
                    line.split_whitespace()
                        .nth(4)
                        .and_then(|addr| addr.split(':').last())
                        .and_then(|port| port.parse().ok())
                })
                .collect()
        })
        .unwrap_or_default();

    let package_updates = get_package_updates();

    SecurityInfo {
        firewall_enabled,
        fail2ban_active,
        open_ports,
        package_updates,
    }
}

fn get_cpu_info(sys: &System) -> CpuInfo {
    let cpu = sys.global_cpu_info();
    CpuInfo {
        usage_percent: cpu.cpu_usage(),
        temperature_celsius: None, // Requires additional system-specific implementation
        frequency_mhz: cpu.frequency() as f32,
    }
}

fn get_memory_info(sys: &System) -> MemoryInfo {
    let total = sys.total_memory();
    let used = sys.used_memory();
    MemoryInfo {
        total_mb: total / 1024 / 1024,
        used_mb: used / 1024 / 1024,
        free_mb: (total - used) / 1024 / 1024,
    }
}

fn get_load_average() -> LoadAverage {
    let output = Command::new("uptime")
        .output()
        .expect("Failed to execute uptime command");
    let output_str = String::from_utf8_lossy(&output.stdout);
    let load_avg: Vec<f64> = output_str
        .split("load average:")
        .nth(1)
        .unwrap_or("")
        .split(',')
        .map(|s| s.trim().parse().unwrap_or(0.0))
        .collect();
    
    LoadAverage {
        one: load_avg.get(0).copied().unwrap_or(0.0),
        five: load_avg.get(1).copied().unwrap_or(0.0),
        fifteen: load_avg.get(2).copied().unwrap_or(0.0),
    }
}

fn get_disk_info(sys: &System) -> Vec<DiskInfo> {
    sys.disks()
        .iter()
        .map(|disk| DiskInfo {
            name: disk.name().to_string_lossy().into_owned(),
            total_gb: disk.total_space() as f64 / 1024.0 / 1024.0 / 1024.0,
            used_gb: (disk.total_space() - disk.available_space()) as f64 / 1024.0 / 1024.0 / 1024.0,
            free_gb: disk.available_space() as f64 / 1024.0 / 1024.0 / 1024.0,
            mount_point: disk.mount_point().to_string_lossy().into_owned(),
        })
        .collect()
}

fn get_network_info(sys: &System) -> NetworkInfo {
    let interfaces = sys.networks()
        .iter()
        .map(|(name, data)| {
            let ip_output = Command::new("ip")
                .args(["addr", "show", name])
                .output()
                .ok()
                .and_then(|output| {
                    String::from_utf8_lossy(&output.stdout)
                        .lines()
                        .filter_map(|line| {
                            if line.contains("inet ") {
                                line.split_whitespace()
                                    .nth(1)
                                    .map(|s| s.split('/').next().unwrap_or("").to_string())
                            } else if line.contains("inet6 ") {
                                line.split_whitespace()
                                    .nth(1)
                                    .map(|s| s.split('/').next().unwrap_or("").to_string())
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<String>>()
                        .into()
                })
                .unwrap_or_default();

            // Eğer ip komutu IP bulamazsa, ifconfig'i dene
            let mut ip_addresses = ip_output;
            if ip_addresses.is_empty() {
                if let Ok(output) = Command::new("ifconfig")
                    .arg(name)
                    .output() {
                    let output_str = String::from_utf8_lossy(&output.stdout);
                    for line in output_str.lines() {
                        if line.contains("inet ") {
                            if let Some(ip) = line.split_whitespace()
                                .nth(1)
                                .map(|s| s.to_string()) {
                                ip_addresses.push(ip);
                            }
                        } else if line.contains("inet6 ") {
                            if let Some(ip) = line.split_whitespace()
                                .nth(1)
                                .map(|s| s.to_string()) {
                                ip_addresses.push(ip);
                            }
                        }
                    }
                }
            }

            NetworkInterface {
                name: name.clone(),
                ip_addresses,
                rx_bytes: data.received(),
                tx_bytes: data.transmitted(),
            }
        })
        .collect();

    NetworkInfo { interfaces }
}

fn get_user_access() -> UserAccess {
    let last_ssh = Command::new("last")
        .output()
        .map(|output| {
            String::from_utf8_lossy(&output.stdout)
                .lines()
                .take(5)
                .map(String::from)
                .collect()
        })
        .unwrap_or_default();

    let active_users = Command::new("who")
        .output()
        .map(|output| {
            String::from_utf8_lossy(&output.stdout)
                .lines()
                .map(String::from)
                .collect()
        })
        .unwrap_or_default();

    let sudo_users = Command::new("getent")
        .args(["group", "sudo"])
        .output()
        .map(|output| {
            String::from_utf8_lossy(&output.stdout)
                .split(':')
                .nth(3)
                .unwrap_or("")
                .split(',')
                .map(String::from)
                .collect()
        })
        .unwrap_or_default();

    UserAccess {
        last_ssh_logins: last_ssh,
        active_users,
        sudo_users,
    }
}

fn get_hardware_info() -> HardwareInfo {
    let cpu_info = Command::new("lscpu")
        .output()
        .map(|output| {
            String::from_utf8_lossy(&output.stdout)
                .lines()
                .find(|line| line.contains("Model name"))
                .and_then(|line| line.split(':').nth(1))
                .map(|s| s.trim().to_string())
                .unwrap_or_default()
        })
        .unwrap_or_default();

    let cores = Command::new("nproc")
        .output()
        .map(|output| {
            String::from_utf8_lossy(&output.stdout)
                .trim()
                .parse()
                .unwrap_or(0)
        })
        .unwrap_or(0);

    let total_ram = Command::new("free")
        .arg("-m")
        .output()
        .map(|output| {
            String::from_utf8_lossy(&output.stdout)
                .lines()
                .nth(1)
                .and_then(|line| line.split_whitespace().nth(1))
                .and_then(|s| s.parse().ok())
                .unwrap_or(0)
        })
        .unwrap_or(0);

    let disk_info = Command::new("lsblk")
        .output()
        .map(|output| {
            String::from_utf8_lossy(&output.stdout)
                .lines()
                .map(String::from)
                .collect()
        })
        .unwrap_or_default();

    let system_info = Command::new("dmidecode")
        .output()
        .map(|output| {
            let output_str = String::from_utf8_lossy(&output.stdout);
            let vendor = output_str
                .lines()
                .find(|line| line.contains("Vendor:"))
                .and_then(|line| line.split(':').nth(1))
                .map(|s| s.trim().to_string())
                .unwrap_or_default();
            let model = output_str
                .lines()
                .find(|line| line.contains("Product Name:"))
                .and_then(|line| line.split(':').nth(1))
                .map(|s| s.trim().to_string())
                .unwrap_or_default();
            (vendor, model)
        })
        .unwrap_or_default();

    HardwareInfo {
        cpu_model: cpu_info,
        cores,
        total_ram_mb: total_ram,
        disk_info,
        system_vendor: system_info.0,
        system_model: system_info.1,
    }
}

fn get_uptime_info() -> UptimeInfo {
    // Mevcut uptime bilgisini al
    let uptime_output = Command::new("uptime")
        .output()
        .expect("uptime komutu çalıştırılamadı");
    let uptime_str = String::from_utf8_lossy(&uptime_output.stdout);
    
    // Son boot zamanını al
    let boot_time = Command::new("who")
        .args(["-b"])
        .output()
        .expect("who komutu çalıştırılamadı");
    let boot_str = String::from_utf8_lossy(&boot_time.stdout);
    
    // Reboot geçmişini al
    let reboot_history = get_reboot_history();

    UptimeInfo {
        current_uptime: uptime_str.trim().to_string(),
        last_boot_time: boot_str.trim().to_string(),
        reboot_history,
    }
}

fn get_reboot_history() -> Vec<RebootRecord> {
    let mut history = Vec::new();
    
    // last reboot komutundan reboot geçmişini al
    if let Ok(output) = Command::new("last")
        .arg("reboot")
        .output() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        
        for line in output_str.lines() {
            if line.contains("reboot") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 4 {
                    let timestamp = format!("{} {} {}", parts[3], parts[4], parts[5]);
                    let reason = if parts.len() > 6 {
                        Some(parts[6..].join(" "))
                    } else {
                        None
                    };
                    
                    history.push(RebootRecord {
                        timestamp,
                        reason,
                    });
                }
            }
        }
    }
    
    // journalctl'den de reboot geçmişini al (systemd sistemlerde)
    if Command::new("which").arg("journalctl").output().map(|o| o.status.success()).unwrap_or(false) {
        if let Ok(output) = Command::new("journalctl")
            .args(["--list-boots", "--no-pager"])
            .output() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            
            for line in output_str.lines().skip(1) { // İlk satırı atla (başlık)
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 {
                    let timestamp = format!("{} {}", parts[1], parts[2]);
                    
                    // Bu boot için kapanış nedenini bul
                    let reason = if let Ok(reason_output) = Command::new("journalctl")
                        .args(["-b", parts[0], "--no-pager"])
                        .output() {
                        let reason_str = String::from_utf8_lossy(&reason_output.stdout);
                        if let Some(last_line) = reason_str.lines().last() {
                            if last_line.contains("shutdown") || last_line.contains("reboot") {
                                Some(last_line.to_string())
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    };
                    
                    history.push(RebootRecord {
                        timestamp,
                        reason,
                    });
                }
            }
        }
    }
    
    history
}

fn get_system_info() -> SystemInfo {
    let mut sys = System::new_all();
    sys.refresh_all();

    // Hostname
    let hostname = sys.host_name().unwrap_or_else(|| "Unknown".to_string());

    // Kernel version
    let kernel_version = sys.kernel_version().unwrap_or_else(|| "Unknown".to_string());

    // OS version
    let os_version = sys.long_os_version().unwrap_or_else(|| "Unknown".to_string());

    // CPU Info
    let cpu_info = CpuInfo {
        usage_percent: sys.global_cpu_info().cpu_usage(),
        temperature_celsius: None, // Sıcaklık bilgisi için ek kütüphane gerekebilir
        frequency_mhz: sys.global_cpu_info().frequency() as f32,
    };

    // Memory Info
    let memory_info = MemoryInfo {
        total_mb: sys.total_memory() / 1024 / 1024,
        used_mb: sys.used_memory() / 1024 / 1024,
        free_mb: (sys.total_memory() - sys.used_memory()) / 1024 / 1024,
    };

    // Disk Info
    let disks = get_disk_info(&sys);

    // Network Info
    let network_info = get_network_info(&sys);

    // User Access
    let user_access = get_user_access();

    // Services
    let services = get_services();

    // Security Info
    let security_info = get_security_info();

    // Hardware Info
    let hardware_info = get_hardware_info();

    // Uptime Info
    let uptime_info = get_uptime_info();

    // Process List
    let mut process_list = Vec::new();
    for (pid, process) in sys.processes() {
        process_list.push(ProcessInfo {
            pid: *pid,
            name: process.name().to_string(),
            cpu_usage: process.cpu_usage(),
            memory_usage: process.memory(),
            status: process.status().to_string(),
            user: process.user_id().unwrap_or(0).to_string(),
            command: process.cmd().join(" "),
        });
    }

    SystemInfo {
        cpu: cpu_info,
        memory: memory_info,
        load_avg: get_load_average(),
        disks,
        network: network_info,
        user_access,
        services,
        security: security_info,
        hardware: hardware_info,
        system_uptime: uptime_info,
        hostname,
        kernel_version,
        os_version,
        process_list,
        timestamp: chrono::Local::now().to_rfc3339(),
    }
}

fn save_to_json(info: &SystemInfo) -> io::Result<()> {
    let json = serde_json::to_string_pretty(info)?;
    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S").to_string();
    let filename = format!("system_info_{}.json", timestamp);
    
    let mut file = fs::File::create(&filename)?;
    file.write_all(json.as_bytes())?;
    
    println!("Sistem bilgileri {} dosyasına kaydedildi.", filename);
    Ok(())
}

fn run_monitor() -> Result<()> {
    info!("StaffMon başlatılıyor...");
    
    // API yapılandırmasını yükle
    let api_config = match config::ApiConfig::load() {
        Ok(config) => {
            info!("API yapılandırması yüklendi");
            config
        }
        Err(e) => {
            error!("API yapılandırması yüklenemedi: {}", e);
            return Err(e.into());
        }
    };

    let api_client = match api::ApiClient::new(api_config) {
        Ok(client) => {
            info!("API istemcisi oluşturuldu");
            client
        }
        Err(e) => {
            error!("API istemcisi oluşturulamadı: {}", e);
            return Err(e.into());
        }
    };

    info!("Sistem izleme başlatıldı");
    
    loop {
        let system_info = get_system_info();
        
        // JSON dosyasına kaydet
        if let Err(e) = save_to_json(&system_info) {
            error!("JSON dosyasına kaydedilemedi: {}", e);
        } else {
            info!("Sistem bilgileri JSON dosyasına kaydedildi");
        }

        // API'ye gönder
        if let Err(e) = api_client.send_system_info(&system_info) {
            error!("API'ye gönderilemedi: {}", e);
        } else {
            info!("Sistem bilgileri API'ye gönderildi");
        }

        // 2 saniye bekle
        thread::sleep(Duration::from_secs(2));
    }
}

fn main() -> Result<()> {
    // Log sistemini başlat
    log_config::init_logger()?;
    
    info!("StaffMon başlatılıyor...");

    // Daemon olarak çalıştır
    let daemonize = Daemonize::new()
        .pid_file("/tmp/staffmon.pid")
        .chown_pid_file(true)
        .working_directory(".")
        .user("nobody")
        .group("nobody")
        .umask(0o027);

    match daemonize.start() {
        Ok(_) => {
            info!("StaffMon arka planda başlatıldı");
            run_monitor()
        }
        Err(e) => {
            error!("Daemon başlatılamadı: {}", e);
            Err(e.into())
        }
    }
} 