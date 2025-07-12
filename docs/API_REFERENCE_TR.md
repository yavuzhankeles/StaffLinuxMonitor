# StaffMon API Referans Dokümantasyonu

## İçindekiler

1. [Genel Bakış](#genel-bakış)
2. [Veri Yapıları](#veri-yapıları)
3. [Temel Fonksiyonlar](#temel-fonksiyonlar)
4. [API İstemcisi](#api-istemcisi)
5. [Yapılandırma](#yapılandırma)
6. [Günlük Kaydı](#günlük-kaydı)
7. [Örnekler](#örnekler)
8. [Hata Yönetimi](#hata-yönetimi)

## Genel Bakış

StaffMon, Rust dilinde yazılmış kapsamlı bir Linux sistem izleme aracıdır. Hem yerel JSON çıktısı hem de REST API entegrasyonu aracılığıyla gerçek zamanlı sistem bilgisi toplama, izleme ve raporlama yetenekleri sağlar.

### Temel Özellikler

- **Sistem Bilgisi Toplama**: CPU, bellek, disk, ağ ve süreç izleme
- **Servis Yönetimi**: Servis durumu ve sürüm algılama
- **Güvenlik İzleme**: Güvenlik duvarı durumu, açık portlar ve paket güncellemeleri
- **Kullanıcı Erişim Takibi**: SSH girişleri, aktif kullanıcılar ve sudo erişimi
- **Donanım Bilgisi**: Detaylı donanım özellikleri
- **Çalışma Süresi ve Yeniden Başlatma Geçmişi**: Sistem çalışma süresi takibi ve yeniden başlatma analizi
- **API Entegrasyonu**: Uzaktan izleme için REST API istemcisi
- **Daemon Modu**: Arka plan servis çalışması
- **Kapsamlı Günlük Kaydı**: Dosya çıktısı ile yapılandırılmış günlük kaydı

## Veri Yapıları

### SystemInfo

Tüm sistem bilgilerini içeren ana veri yapısı.

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

**Alanlar:**
- `cpu`: CPU kullanımı ve performans bilgisi
- `memory`: Bellek kullanım istatistikleri
- `load_avg`: Sistem yük ortalaması (1, 5, 15 dakika ortalamaları)
- `disks`: Disk bölümleri ve kullanım listesi
- `network`: Ağ arayüzü bilgisi
- `user_access`: Kullanıcı giriş ve erişim bilgisi
- `services`: Sistem servisleri durumu
- `security`: Güvenlikle ilgili bilgiler
- `hardware`: Donanım özellikleri
- `system_uptime`: Sistem çalışma süresi ve yeniden başlatma geçmişi
- `hostname`: Sistem host adı
- `kernel_version`: Linux çekirdek sürümü
- `os_version`: İşletim sistemi sürümü
- `process_list`: Çalışan süreçler listesi
- `timestamp`: Veri toplama zamanı (ISO 8601 formatında)

### CpuInfo

CPU performansı ve kullanım bilgisi.

```rust
#[derive(Debug, Serialize)]
struct CpuInfo {
    usage_percent: f32,
    temperature_celsius: Option<f32>,
    frequency_mhz: f32,
}
```

**Alanlar:**
- `usage_percent`: CPU kullanım yüzdesi (0.0 - 100.0)
- `temperature_celsius`: CPU sıcaklığı (Celsius, mevcutsa)
- `frequency_mhz`: Mevcut CPU frekansı (MHz)

### MemoryInfo

Bellek kullanım istatistikleri.

```rust
#[derive(Debug, Serialize)]
struct MemoryInfo {
    total_mb: u64,
    used_mb: u64,
    free_mb: u64,
}
```

**Alanlar:**
- `total_mb`: Toplam bellek (MB)
- `used_mb`: Kullanılan bellek (MB)
- `free_mb`: Boş bellek (MB)

### LoadAverage

Sistem yük ortalaması bilgisi.

```rust
#[derive(Debug, Serialize)]
struct LoadAverage {
    one: f64,
    five: f64,
    fifteen: f64,
}
```

**Alanlar:**
- `one`: 1 dakikalık yük ortalaması
- `five`: 5 dakikalık yük ortalaması
- `fifteen`: 15 dakikalık yük ortalaması

### DiskInfo

Disk bölümü bilgisi.

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

**Alanlar:**
- `name`: Cihaz adı (örn. "/dev/sda1")
- `total_gb`: Toplam disk alanı (GB)
- `used_gb`: Kullanılan disk alanı (GB)
- `free_gb`: Boş disk alanı (GB)
- `mount_point`: Bağlama noktası yolu

### NetworkInfo

Ağ arayüzü bilgisi.

```rust
#[derive(Debug, Serialize)]
struct NetworkInfo {
    interfaces: Vec<NetworkInterface>,
}
```

### NetworkInterface

Tekil ağ arayüzü detayları.

```rust
#[derive(Debug, Serialize)]
struct NetworkInterface {
    name: String,
    ip_addresses: Vec<String>,
    rx_bytes: u64,
    tx_bytes: u64,
}
```

**Alanlar:**
- `name`: Arayüz adı (örn. "eth0", "wlan0")
- `ip_addresses`: IP adresleri listesi (IPv4 ve IPv6)
- `rx_bytes`: Alınan bayt sayısı
- `tx_bytes`: Gönderilen bayt sayısı

### UserAccess

Kullanıcı erişimi ve giriş bilgisi.

```rust
#[derive(Debug, Serialize)]
struct UserAccess {
    last_ssh_logins: Vec<String>,
    active_users: Vec<String>,
    sudo_users: Vec<String>,
}
```

**Alanlar:**
- `last_ssh_logins`: Son SSH giriş kayıtları
- `active_users`: Şu anda giriş yapmış kullanıcılar
- `sudo_users`: Sudo ayrıcalığına sahip kullanıcılar

### ServiceInfo

Sistem servisi bilgisi.

```rust
#[derive(Debug, Serialize)]
struct ServiceInfo {
    name: String,
    active: bool,
    enabled: bool,
    version: Option<String>,
}
```

**Alanlar:**
- `name`: Servis adı
- `active`: Servisin şu anda çalışıp çalışmadığı
- `enabled`: Servisin açılışta başlatılmaya ayarlı olup olmadığı
- `version`: Servis sürümü (mevcutsa)

### SecurityInfo

Güvenlikle ilgili sistem bilgisi.

```rust
#[derive(Debug, Serialize)]
struct SecurityInfo {
    firewall_enabled: bool,
    fail2ban_active: bool,
    open_ports: Vec<u16>,
    package_updates: Vec<String>,
}
```

**Alanlar:**
- `firewall_enabled`: UFW güvenlik duvarının aktif olup olmadığı
- `fail2ban_active`: fail2ban'ın çalışıp çalışmadığı
- `open_ports`: Açık ağ portları listesi
- `package_updates`: Mevcut paket güncellemeleri

### HardwareInfo

Donanım özellikleri.

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

**Alanlar:**
- `cpu_model`: CPU model adı
- `cores`: CPU çekirdek sayısı
- `total_ram_mb`: Toplam RAM (MB)
- `disk_info`: Disk cihaz bilgisi
- `system_vendor`: Sistem üreticisi
- `system_model`: Sistem modeli

### UptimeInfo

Sistem çalışma süresi ve yeniden başlatma bilgisi.

```rust
#[derive(Debug, Serialize)]
struct UptimeInfo {
    current_uptime: String,
    last_boot_time: String,
    reboot_history: Vec<RebootRecord>,
}
```

**Alanlar:**
- `current_uptime`: Mevcut sistem çalışma süresi dizisi
- `last_boot_time`: Son sistem açılış zamanı
- `reboot_history`: Sistem yeniden başlatma geçmişi

### RebootRecord

Tekil yeniden başlatma kaydı.

```rust
#[derive(Debug, Serialize)]
struct RebootRecord {
    timestamp: String,
    reason: Option<String>,
}
```

**Alanlar:**
- `timestamp`: Yeniden başlatma zaman damgası
- `reason`: Yeniden başlatma nedeni (mevcutsa)

### ProcessInfo

Süreç bilgisi (kullanımdan çıkarılmış).

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

**Alanlar:**
- `pid`: Süreç ID'si
- `name`: Süreç adı
- `cpu_usage`: CPU kullanım yüzdesi
- `memory_usage`: Bellek kullanımı (bayt)
- `status`: Süreç durumu
- `user`: Süreç sahibi
- `command`: Tam komut satırı

### PackageManager

Desteklenen paket yöneticileri.

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

## Temel Fonksiyonlar

### Sistem Bilgisi Toplama

#### `get_system_info() -> SystemInfo`

Tüm mevcut kaynaklardan kapsamlı sistem bilgisi toplar.

**Döndürür:** Tam sistem bilgisi yapısı

**Örnek:**
```rust
let system_info = get_system_info();
println!("Host Adı: {}", system_info.hostname);
println!("CPU Kullanımı: {:.1}%", system_info.cpu.usage_percent);
```

#### `get_cpu_info(sys: &System) -> CpuInfo`

CPU'ya özel bilgi toplar.

**Parametreler:**
- `sys`: sysinfo crate'inden System örneği

**Döndürür:** CPU bilgisi yapısı

**Örnek:**
```rust
let mut sys = System::new_all();
sys.refresh_all();
let cpu_info = get_cpu_info(&sys);
println!("CPU Frekansı: {} MHz", cpu_info.frequency_mhz);
```

#### `get_memory_info(sys: &System) -> MemoryInfo`

Bellek kullanım bilgisi toplar.

**Parametreler:**
- `sys`: sysinfo crate'inden System örneği

**Döndürür:** Bellek bilgisi yapısı

**Örnek:**
```rust
let mut sys = System::new_all();
sys.refresh_all();
let mem_info = get_memory_info(&sys);
println!("Bellek Kullanımı: {}/{} MB", mem_info.used_mb, mem_info.total_mb);
```

#### `get_load_average() -> LoadAverage`

Sistem yük ortalaması bilgisini alır.

**Döndürür:** Yük ortalaması yapısı

**Örnek:**
```rust
let load_avg = get_load_average();
println!("1 dakika yük: {:.2}", load_avg.one);
```

#### `get_disk_info(sys: &System) -> Vec<DiskInfo>`

Disk bölümü bilgisi toplar.

**Parametreler:**
- `sys`: sysinfo crate'inden System örneği

**Döndürür:** Disk bilgisi yapıları vektörü

**Örnek:**
```rust
let mut sys = System::new_all();
sys.refresh_all();
let disks = get_disk_info(&sys);
for disk in &disks {
    println!("{}: {:.1}GB kullanılan / {:.1}GB toplam", disk.name, disk.used_gb, disk.total_gb);
}
```

#### `get_network_info(sys: &System) -> NetworkInfo`

Ağ arayüzü bilgisi toplar.

**Parametreler:**
- `sys`: sysinfo crate'inden System örneği

**Döndürür:** Ağ bilgisi yapısı

**Örnek:**
```rust
let mut sys = System::new_all();
sys.refresh_all();
let net_info = get_network_info(&sys);
for interface in &net_info.interfaces {
    println!("{}: {:?}", interface.name, interface.ip_addresses);
}
```

### Servis Yönetimi

#### `get_services() -> Vec<ServiceInfo>`

Sistem servisleri hakkında bilgi toplar.

**Döndürür:** Servis bilgisi yapıları vektörü

**Örnek:**
```rust
let services = get_services();
for service in &services {
    if service.active {
        println!("{} çalışıyor", service.name);
    }
}
```

#### `get_service_status(service: &str) -> (bool, bool)`

Belirli bir servisin durumunu kontrol eder.

**Parametreler:**
- `service`: Kontrol edilecek servis adı

**Döndürür:** (aktif_mi, etkin_mi) tuple'ı

**Örnek:**
```rust
let (active, enabled) = get_service_status("nginx");
println!("Nginx aktif: {}, etkin: {}", active, enabled);
```

#### `get_service_version(service: &str) -> Option<String>`

Belirli bir servisin sürümünü alır.

**Parametreler:**
- `service`: Servis adı

**Döndürür:** Servis sürüm dizisi veya None

**Örnek:**
```rust
if let Some(version) = get_service_version("nginx") {
    println!("Nginx sürümü: {}", version);
}
```

### Güvenlik Fonksiyonları

#### `get_security_info() -> SecurityInfo`

Güvenlikle ilgili sistem bilgisi toplar.

**Döndürür:** Güvenlik bilgisi yapısı

**Örnek:**
```rust
let security = get_security_info();
println!("Güvenlik duvarı etkin: {}", security.firewall_enabled);
println!("Açık portlar: {:?}", security.open_ports);
```

#### `get_package_updates() -> Vec<String>`

Mevcut paket güncellemelerini alır.

**Döndürür:** Güncelleme mevcut paket adları vektörü

**Örnek:**
```rust
let updates = get_package_updates();
println!("Mevcut güncellemeler: {}", updates.len());
for update in &updates {
    println!("  {}", update);
}
```

#### `detect_package_manager() -> PackageManager`

Sistemin paket yöneticisini algılar.

**Döndürür:** Paket yöneticisi numaralandırması

**Örnek:**
```rust
match detect_package_manager() {
    PackageManager::Apt => println!("APT paket yöneticisi kullanılıyor"),
    PackageManager::Yum => println!("YUM paket yöneticisi kullanılıyor"),
    PackageManager::Dnf => println!("DNF paket yöneticisi kullanılıyor"),
    PackageManager::Pacman => println!("Pacman paket yöneticisi kullanılıyor"),
    PackageManager::Zypper => println!("Zypper paket yöneticisi kullanılıyor"),
    PackageManager::Unknown => println!("Bilinmeyen paket yöneticisi"),
}
```

### Kullanıcı ve Erişim Fonksiyonları

#### `get_user_access() -> UserAccess`

Kullanıcı erişimi ve giriş bilgisi toplar.

**Döndürür:** Kullanıcı erişim bilgisi yapısı

**Örnek:**
```rust
let user_access = get_user_access();
println!("Aktif kullanıcılar: {:?}", user_access.active_users);
println!("Son SSH girişleri: {:?}", user_access.last_ssh_logins);
```

### Donanım Fonksiyonları

#### `get_hardware_info() -> HardwareInfo`

Donanım özelliklerini toplar.

**Döndürür:** Donanım bilgisi yapısı

**Örnek:**
```rust
let hardware = get_hardware_info();
println!("CPU: {}", hardware.cpu_model);
println!("Çekirdekler: {}", hardware.cores);
println!("RAM: {} MB", hardware.total_ram_mb);
```

### Çalışma Süresi Fonksiyonları

#### `get_uptime_info() -> UptimeInfo`

Sistem çalışma süresi bilgisini toplar.

**Döndürür:** Çalışma süresi bilgisi yapısı

**Örnek:**
```rust
let uptime = get_uptime_info();
println!("Mevcut çalışma süresi: {}", uptime.current_uptime);
println!("Son açılış: {}", uptime.last_boot_time);
```

#### `get_reboot_history() -> Vec<RebootRecord>`

Sistem yeniden başlatma geçmişini alır.

**Döndürür:** Yeniden başlatma kayıtları vektörü

**Örnek:**
```rust
let reboots = get_reboot_history();
for reboot in &reboots {
    println!("Yeniden başlatma {}: {:?}", reboot.timestamp, reboot.reason);
}
```

### Yardımcı Fonksiyonlar

#### `save_to_json(info: &SystemInfo) -> io::Result<()>`

Sistem bilgisini JSON dosyasına kaydeder.

**Parametreler:**
- `info`: Kaydedilecek sistem bilgisi

**Döndürür:** IO sonucu

**Örnek:**
```rust
let system_info = get_system_info();
if let Err(e) = save_to_json(&system_info) {
    eprintln!("JSON kaydedilemedi: {}", e);
}
```

#### `run_monitor() -> Result<()>`

Ana izleme döngüsünü çalıştırır.

**Döndürür:** Başarı veya başarısızlık gösteren sonuç

**Örnek:**
```rust
if let Err(e) = run_monitor() {
    eprintln!("İzleme başarısız: {}", e);
}
```

## API İstemcisi

### ApiClient

Sistem bilgisini uzak sunuculara göndermek için REST API istemcisi.

#### `ApiClient::new(config: ApiConfig) -> Result<Self>`

Yeni bir API istemcisi örneği oluşturur.

**Parametreler:**
- `config`: API yapılandırması

**Döndürür:** API istemcisi örneği veya hata

**Örnek:**
```rust
let config = ApiConfig::load()?;
let client = ApiClient::new(config)?;
```

#### `send_system_info(&self, info: &SystemInfo) -> Result<()>`

Sistem bilgisini API sunucusuna gönderir.

**Parametreler:**
- `info`: Gönderilecek sistem bilgisi

**Döndürür:** Başarı veya başarısızlık gösteren sonuç

**Örnek:**
```rust
let system_info = get_system_info();
client.send_system_info(&system_info)?;
```

#### `get_system_info(&self) -> Result<SystemInfo>`

API sunucusundan sistem bilgisini alır.

**Döndürür:** Sistem bilgisi veya hata

**Örnek:**
```rust
let system_info = client.get_system_info()?;
println!("Alınan sistem bilgisi: {}", system_info.hostname);
```

## Yapılandırma

### ApiConfig

API istemcisi yapılandırma yapısı.

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

**Alanlar:**
- `base_url`: API sunucusu temel URL'si
- `api_key`: Kimlik doğrulama API anahtarı
- `timeout_seconds`: İstek zaman aşımı (saniye)
- `retry_count`: Yeniden deneme sayısı
- `rate_limit`: Dakikada sınırlı istek sayısı

#### `ApiConfig::default() -> Self`

Varsayılan API yapılandırması oluşturur.

**Döndürür:** Varsayılan yapılandırma

**Örnek:**
```rust
let config = ApiConfig::default();
// base_url: "http://localhost:8080"
// timeout_seconds: 30
// retry_count: 3
// rate_limit: 100
```

#### `ApiConfig::load() -> Result<Self>`

Dosya ve ortam değişkenlerinden yapılandırmayı yükler.

**Döndürür:** Yüklenen yapılandırma veya hata

**Yapılandırma Kaynakları:**
1. `config.toml` dosyası
2. `STAFFMON_` öneki ile ortam değişkenleri
3. `.env` dosyası
4. Varsayılan değerler

**Örnek:**
```rust
let config = ApiConfig::load()?;
println!("API URL: {}", config.base_url);
```

**Yapılandırma Dosyası Örneği (`config.toml`):**
```toml
base_url = "https://api.example.com"
api_key = "your-api-key-here"
timeout_seconds = 30
retry_count = 3
rate_limit = 100
```

**Ortam Değişkenleri:**
```bash
export STAFFMON_BASE_URL="https://api.example.com"
export STAFFMON_API_KEY="your-api-key-here"
export STAFFMON_TIMEOUT_SECONDS=30
export STAFFMON_RETRY_COUNT=3
export STAFFMON_RATE_LIMIT=100
```

## Günlük Kaydı

### `init_logger() -> Result<(), Box<dyn std::error::Error>>`

Günlük kaydı sistemini başlatır.

**Döndürür:** Başarı veya başarısızlık gösteren sonuç

**Özellikler:**
- `logs/` dizinini yoksa oluşturur
- `logs/staffmon.log` dosyasına günlük kaydı yapar
- Desen kullanır: `{d(%Y-%m-%d %H:%M:%S)} [{l}] {m}{n}`
- Günlük seviyesi: Info

**Örnek:**
```rust
if let Err(e) = log_config::init_logger() {
    eprintln!("Günlük kaydı başlatılamadı: {}", e);
}
```

**Günlük Formatı:**
```
2024-01-15 10:30:45 [INFO] StaffMon başlatılıyor...
2024-01-15 10:30:45 [INFO] API yapılandırması yüklendi
2024-01-15 10:30:45 [INFO] API istemcisi oluşturuldu
2024-01-15 10:30:45 [INFO] Sistem izleme başlatıldı
```

## Örnekler

### Temel Sistem Bilgisi Toplama

```rust
use staffmon::{get_system_info, save_to_json};

fn main() -> Result<()> {
    // Sistem bilgisini topla
    let system_info = get_system_info();
    
    // Temel bilgileri yazdır
    println!("Host Adı: {}", system_info.hostname);
    println!("İşletim Sistemi: {}", system_info.os_version);
    println!("Çekirdek: {}", system_info.kernel_version);
    println!("CPU Kullanımı: {:.1}%", system_info.cpu.usage_percent);
    println!("Bellek Kullanımı: {}/{} MB", 
             system_info.memory.used_mb, 
             system_info.memory.total_mb);
    
    // JSON dosyasına kaydet
    save_to_json(&system_info)?;
    
    Ok(())
}
```

### Servis İzleme

```rust
use staffmon::{get_services, get_service_status};

fn main() {
    // Tüm servisleri al
    let services = get_services();
    
    println!("Sistem Servisleri:");
    for service in &services {
        let status = if service.active { "Çalışıyor" } else { "Durdu" };
        let enabled = if service.enabled { "Etkin" } else { "Devre Dışı" };
        println!("  {}: {} ({})", service.name, status, enabled);
        
        if let Some(version) = &service.version {
            println!("    Sürüm: {}", version);
        }
    }
    
    // Belirli servisi kontrol et
    let (active, enabled) = get_service_status("nginx");
    println!("Nginx - Aktif: {}, Etkin: {}", active, enabled);
}
```

### Ağ İzleme

```rust
use staffmon::{get_system_info, System};

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();
    
    let system_info = get_system_info();
    
    println!("Ağ Arayüzleri:");
    for interface in &system_info.network.interfaces {
        println!("  {}:", interface.name);
        println!("    IP Adresleri: {:?}", interface.ip_addresses);
        println!("    RX: {} bayt", interface.rx_bytes);
        println!("    TX: {} bayt", interface.tx_bytes);
    }
}
```

### Güvenlik İzleme

```rust
use staffmon::get_security_info;

fn main() {
    let security = get_security_info();
    
    println!("Güvenlik Durumu:");
    println!("  Güvenlik Duvarı (UFW): {}", 
             if security.firewall_enabled { "Etkin" } else { "Devre Dışı" });
    println!("  Fail2ban: {}", 
             if security.fail2ban_active { "Aktif" } else { "Pasif" });
    println!("  Açık Portlar: {:?}", security.open_ports);
    println!("  Mevcut Güncellemeler: {}", security.package_updates.len());
    
    for update in &security.package_updates {
        println!("    {}", update);
    }
}
```

### API Entegrasyonu

```rust
use staffmon::{ApiClient, ApiConfig, get_system_info};

fn main() -> Result<()> {
    // Yapılandırmayı yükle
    let config = ApiConfig::load()?;
    
    // API istemcisi oluştur
    let client = ApiClient::new(config)?;
    
    // Sistem bilgisini topla ve gönder
    let system_info = get_system_info();
    client.send_system_info(&system_info)?;
    
    println!("Sistem bilgisi API'ye başarıyla gönderildi");
    
    Ok(())
}
```

### Sürekli İzleme

```rust
use staffmon::{run_monitor, log_config};
use std::time::Duration;
use std::thread;

fn main() -> Result<()> {
    // Günlük kaydını başlat
    log_config::init_logger()?;
    
    // İzleme döngüsünü çalıştır
    run_monitor()?;
    
    Ok(())
}
```

### Özel İzleme Döngüsü

```rust
use staffmon::{get_system_info, save_to_json, ApiClient, ApiConfig};
use std::time::Duration;
use std::thread;

fn main() -> Result<()> {
    let config = ApiConfig::load()?;
    let client = ApiClient::new(config)?;
    
    loop {
        // Sistem bilgisini topla
        let system_info = get_system_info();
        
        // Yerel JSON dosyasına kaydet
        if let Err(e) = save_to_json(&system_info) {
            eprintln!("JSON kaydedilemedi: {}", e);
        }
        
        // API'ye gönder
        if let Err(e) = client.send_system_info(&system_info) {
            eprintln!("API'ye gönderilemedi: {}", e);
        }
        
        // Sonraki toplama öncesi bekle
        thread::sleep(Duration::from_secs(60)); // 1 dakika aralık
    }
}
```

## Hata Yönetimi

StaffMon, çeşitli hata türlerini işleyebilen birleşik bir hata türü (`Result<T>`) sağlayan `anyhow` crate'ini kullanır.

### Yaygın Hata Desenleri

```rust
use anyhow::Result;

// Başarısız olabilecek fonksiyon
fn process_system_info() -> Result<()> {
    let system_info = get_system_info();
    
    // Potansiyel hataları işle
    save_to_json(&system_info)?;
    
    let config = ApiConfig::load()?;
    let client = ApiClient::new(config)?;
    client.send_system_info(&system_info)?;
    
    Ok(())
}

// Ana fonksiyonda hata işleme
fn main() {
    if let Err(e) = process_system_info() {
        eprintln!("Hata: {}", e);
        std::process::exit(1);
    }
}
```

### Hata Türleri

- **IO Hataları**: Dosya işlemleri, ağ istekleri
- **Yapılandırma Hataları**: Eksik veya geçersiz yapılandırma dosyaları
- **API Hataları**: Ağ zaman aşımları, kimlik doğrulama başarısızlıkları
- **Sistem Hataları**: Komut yürütme başarısızlıkları, izin sorunları

### Hata Günlük Kaydı

```rust
use log::{error, info};

fn monitor_system() -> Result<()> {
    info!("Sistem izleme başlatılıyor");
    
    let system_info = get_system_info();
    
    match save_to_json(&system_info) {
        Ok(_) => info!("Sistem bilgisi başarıyla kaydedildi"),
        Err(e) => error!("Sistem bilgisi kaydedilemedi: {}", e),
    }
    
    match ApiConfig::load() {
        Ok(config) => {
            match ApiClient::new(config) {
                Ok(client) => {
                    match client.send_system_info(&system_info) {
                        Ok(_) => info!("Sistem bilgisi API'ye gönderildi"),
                        Err(e) => error!("API'ye gönderilemedi: {}", e),
                    }
                }
                Err(e) => error!("API istemcisi oluşturulamadı: {}", e),
            }
        }
        Err(e) => error!("API yapılandırması yüklenemedi: {}", e),
    }
    
    Ok(())
}
```

## En İyi Uygulamalar

### 1. Hata Yönetimi
Sistem bilgisi toplama fonksiyonlarından gelen potansiyel hataları her zaman işleyin.

### 2. Kaynak Yönetimi
API çağrıları için uygun zaman aşımları ve yeniden deneme mekanizmaları kullanın.

### 3. Günlük Kaydı
Hata ayıklama ve izleme için kapsamlı günlük kaydı uygulayın.

### 4. Yapılandırma
API anahtarları gibi hassas yapılandırma için ortam değişkenleri kullanın.

### 5. Performans
Sistem etkisini önlemek için veri toplama sıklığını göz önünde bulundurun.

### 6. Güvenlik
Harici API'lere göndermeden önce tüm sistem bilgilerini doğrulayın ve temizleyin.

## Bağımlılıklar

StaffMon aşağıdaki temel bağımlılıklara dayanır:

- **sysinfo**: Sistem bilgisi toplama
- **serde**: Serileştirme/serileştirme çözme
- **reqwest**: API iletişimi için HTTP istemcisi
- **tokio**: Asenkron çalışma zamanı
- **log**: Günlük kaydı çerçevesi
- **anyhow**: Hata yönetimi
- **chrono**: Tarih/zaman işleme
- **daemonize**: Daemon süreç yönetimi

## Lisans

Bu proje MIT Lisansı altında lisanslanmıştır. Detaylar için LICENSE dosyasına bakın.