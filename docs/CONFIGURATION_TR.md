# Yapılandırma Kılavuzu

## Genel Bakış

StaffLinuxMonitor, yapılandırma için TOML (Tom's Obvious, Minimal Language) kullanır. Yapılandırma dosyası ile izleme davranışını, API ayarlarını, loglama ve çıktı formatlarını özelleştirebilirsiniz.

## Yapılandırma Dosyası Konumu

Araç yapılandırma dosyalarını şu sırayla arar:

1. Komut satırında belirtilen dosya (`--config` seçeneği)
2. `./config.toml` (mevcut dizin)
3. `~/.config/staffmon/config.toml` (kullanıcı yapılandırma dizini)
4. `/etc/staffmon/config.toml` (sistem yapılandırma dizini)

## Yapılandırma Yapısı

### Temel Yapılandırma

```toml
# StaffLinuxMonitor Yapılandırma Dosyası
# Sürüm: 1.0.2

# İzleme Ayarları
[monitoring]
update_interval = 2                    # Güncelleme aralığı (saniye) (1-60)
enable_daemon = false                  # Daemon süreci olarak çalıştır
enable_foreground = true               # Ön planda çıktı göster
enable_json_output = true              # Veriyi JSON dosyasına kaydet
enable_api = true                      # REST API sunucusunu etkinleştir

# Loglama Yapılandırması
[logging]
level = "info"                         # Log seviyesi: trace, debug, info, warn, error
file = "/var/log/staffmon.log"         # Log dosyası yolu
max_size = "100MB"                     # Maksimum log dosyası boyutu
backup_count = 5                       # Saklanacak yedek dosya sayısı
enable_console = true                  # Logları konsola yazdır
enable_syslog = false                  # Logları syslog'a yazdır

# API Yapılandırması
[api]
enabled = true                         # API sunucusunu etkinleştir
host = "127.0.0.1"                    # API sunucu host'u
port = 8080                           # API sunucu portu
api_key = "güvenli-api-anahtarınız"   # API kimlik doğrulama anahtarı
timeout_seconds = 30                  # İstek zaman aşımı
retry_count = 3                       # Yeniden deneme sayısı
rate_limit = 100                      # Dakikada maksimum istek sayısı
enable_cors = false                   # Web istemcileri için CORS etkinleştir
enable_ssl = false                    # HTTPS etkinleştir
ssl_cert = "/path/to/cert.pem"        # SSL sertifika yolu
ssl_key = "/path/to/key.pem"          # SSL özel anahtar yolu

# Çıktı Yapılandırması
[output]
json_file = "/var/log/staffmon_data.json"  # JSON çıktı dosyası yolu
csv_file = ""                              # CSV çıktı dosyası yolu (isteğe bağlı)
prometheus_file = ""                        # Prometheus metrik dosyası (isteğe bağlı)
enable_compression = false                  # Çıktı dosyalarını sıkıştır
max_file_size = "1GB"                      # Maksimum çıktı dosyası boyutu
retention_days = 30                        # Veri saklama süresi

# İzleme Özellikleri
[features]
enable_cpu_monitoring = true           # CPU kullanımı ve sıcaklığını izle
enable_memory_monitoring = true        # Bellek kullanımını izle
enable_disk_monitoring = true          # Disk kullanımını izle
enable_network_monitoring = true       # Ağ arayüzlerini izle
enable_service_monitoring = true       # Sistem servislerini izle
enable_security_monitoring = true      # Güvenlik durumunu izle
enable_process_monitoring = false      # İşlem listesini izle (kaynak yoğun)
enable_hardware_monitoring = true      # Donanım bilgilerini izle
enable_uptime_monitoring = true        # Sistem çalışma süresini izle

# Servis İzleme
[services]
# İzlenecek servislerin listesi (boş = tümünü izle)
monitored_services = [
    "nginx",
    "apache2",
    "mysql",
    "postgresql",
    "redis",
    "docker"
]

# İzlemeden hariç tutulacak servisler
excluded_services = [
    "systemd-timesyncd",
    "systemd-resolved"
]

# Güvenlik İzleme
[security]
enable_firewall_check = true           # Güvenlik duvarı durumunu kontrol et
enable_fail2ban_check = true           # fail2ban durumunu kontrol et
enable_package_updates = true          # Paket güncellemelerini kontrol et
enable_port_scanning = false           # Açık portları tara (root gerekli)
enable_vulnerability_scan = false      # Temel güvenlik açığı taraması
check_ssh_logins = true                # SSH giriş denemelerini izle
check_sudo_usage = true                # sudo kullanımını izle

# Uyarı Yapılandırması
[alerts]
enabled = false                        # Uyarı sistemini etkinleştir
cpu_threshold = 80.0                   # CPU kullanım eşiği (%)
memory_threshold = 85.0                # Bellek kullanım eşiği (%)
disk_threshold = 90.0                  # Disk kullanım eşiği (%)
network_threshold = 1000000            # Ağ kullanım eşiği (byte/s)

# Uyarı Kanalları
[alerts.email]
enabled = false                        # E-posta uyarılarını etkinleştir
smtp_server = "smtp.gmail.com"         # SMTP sunucusu
smtp_port = 587                        # SMTP portu
username = "e-posta-adresiniz@gmail.com"      # SMTP kullanıcı adı
password = "uygulama-şifreniz"         # SMTP şifresi
from_address = "uyarilar@domaininiz.com" # Gönderen e-posta adresi
to_addresses = [                       # Alıcı e-posta adresleri
    "admin@domaininiz.com",
    "ops@domaininiz.com"
]

[alerts.slack]
enabled = false                        # Slack uyarılarını etkinleştir
webhook_url = "https://hooks.slack.com/services/..."  # Slack webhook URL'i
channel = "#uyarilar"                  # Slack kanalı
username = "StaffLinuxMonitor"         # Bot kullanıcı adı

[alerts.telegram]
enabled = false                        # Telegram uyarılarını etkinleştir
bot_token = "bot-tokeniniz"            # Telegram bot token'ı
chat_id = "chat-idiniz"                # Telegram chat ID'si

# Veritabanı Yapılandırması (Gelecek Özellik)
[database]
enabled = false                        # Veritabanı depolamayı etkinleştir
type = "sqlite"                        # Veritabanı türü: sqlite, postgresql, mysql
connection_string = "staffmon.db"      # Veritabanı bağlantı dizesi
max_connections = 10                   # Maksimum veritabanı bağlantısı
enable_migrations = true               # Otomatik migrasyonları etkinleştir

# Dış Entegrasyonlar
[integrations]
# Prometheus Entegrasyonu
[integrations.prometheus]
enabled = false                        # Prometheus metriklerini etkinleştir
metrics_path = "/metrics"              # Metrik endpoint yolu
enable_histograms = true               # Histogram metriklerini etkinleştir

# Grafana Entegrasyonu
[integrations.grafana]
enabled = false                        # Grafana entegrasyonunu etkinleştir
api_url = "http://localhost:3000"      # Grafana API URL'i
api_key = "grafana-api-anahtarınız"    # Grafana API anahtarı
dashboard_uid = "staffmon"             # Dashboard UID'si

# Elasticsearch Entegrasyonu
[integrations.elasticsearch]
enabled = false                        # Elasticsearch entegrasyonunu etkinleştir
url = "http://localhost:9200"          # Elasticsearch URL'i
index_prefix = "staffmon"              # İndeks adı öneki
username = ""                          # Elasticsearch kullanıcı adı
password = ""                          # Elasticsearch şifresi

# Performans Ayarları
[performance]
max_threads = 4                        # Maksimum worker thread sayısı
buffer_size = 8192                     # I/O buffer boyutu
enable_caching = true                  # Metrik önbelleğini etkinleştir
cache_ttl = 60                         # Önbellek TTL (saniye)
enable_compression = false             # Yanıt sıkıştırmayı etkinleştir

# Güvenlik Ayarları
[security_settings]
enable_encryption = false              # Veri şifrelemeyi etkinleştir
encryption_key = ""                    # Şifreleme anahtarı (32 byte)
enable_audit_log = false               # Denetim logunu etkinleştir
audit_log_file = "/var/log/staffmon_audit.log"  # Denetim log dosyası
restrict_api_access = false            # API'yi sadece localhost'a kısıtla
allowed_ips = [                        # API erişimi için izin verilen IP'ler
    "127.0.0.1",
    "192.168.1.0/24"
]

# Özel Metrikler
[custom_metrics]
enabled = false                        # Özel metrik toplamayı etkinleştir
# Özel metrikleri burada tanımlayın
# Örnek:
# [custom_metrics.metrics]
# disk_io = "cat /proc/diskstats"
# custom_process_count = "ps aux | wc -l"
```

## Ortam Değişkenleri

StaffLinuxMonitor'ü ortam değişkenleri ile de yapılandırabilirsiniz. Ortam değişkenleri yapılandırma dosyası ayarlarından önceliklidir.

### Mevcut Ortam Değişkenleri

```bash
# İzleme Ayarları
STAFFMON_UPDATE_INTERVAL=2
STAFFMON_ENABLE_DAEMON=false
STAFFMON_ENABLE_FOREGROUND=true
STAFFMON_ENABLE_JSON_OUTPUT=true
STAFFMON_ENABLE_API=true

# Loglama
STAFFMON_LOG_LEVEL=info
STAFFMON_LOG_FILE=/var/log/staffmon.log
STAFFMON_LOG_MAX_SIZE=100MB
STAFFMON_LOG_BACKUP_COUNT=5
STAFFMON_LOG_ENABLE_CONSOLE=true
STAFFMON_LOG_ENABLE_SYSLOG=false

# API Ayarları
STAFFMON_API_ENABLED=true
STAFFMON_API_HOST=127.0.0.1
STAFFMON_API_PORT=8080
STAFFMON_API_KEY=güvenli-api-anahtarınız
STAFFMON_API_TIMEOUT=30
STAFFMON_API_RETRY_COUNT=3
STAFFMON_API_RATE_LIMIT=100

# Çıktı Ayarları
STAFFMON_OUTPUT_JSON_FILE=/var/log/staffmon_data.json
STAFFMON_OUTPUT_CSV_FILE=
STAFFMON_OUTPUT_PROMETHEUS_FILE=
STAFFMON_OUTPUT_ENABLE_COMPRESSION=false
STAFFMON_OUTPUT_MAX_FILE_SIZE=1GB
STAFFMON_OUTPUT_RETENTION_DAYS=30

# Özellik Bayrakları
STAFFMON_FEATURES_ENABLE_CPU_MONITORING=true
STAFFMON_FEATURES_ENABLE_MEMORY_MONITORING=true
STAFFMON_FEATURES_ENABLE_DISK_MONITORING=true
STAFFMON_FEATURES_ENABLE_NETWORK_MONITORING=true
STAFFMON_FEATURES_ENABLE_SERVICE_MONITORING=true
STAFFMON_FEATURES_ENABLE_SECURITY_MONITORING=true
STAFFMON_FEATURES_ENABLE_PROCESS_MONITORING=false
STAFFMON_FEATURES_ENABLE_HARDWARE_MONITORING=true
STAFFMON_FEATURES_ENABLE_UPTIME_MONITORING=true

# Uyarılar
STAFFMON_ALERTS_ENABLED=false
STAFFMON_ALERTS_CPU_THRESHOLD=80.0
STAFFMON_ALERTS_MEMORY_THRESHOLD=85.0
STAFFMON_ALERTS_DISK_THRESHOLD=90.0
STAFFMON_ALERTS_NETWORK_THRESHOLD=1000000
```

## Yapılandırma Örnekleri

### Minimal Yapılandırma

```toml
# Temel izleme için minimal yapılandırma
[monitoring]
update_interval = 5
enable_daemon = false

[logging]
level = "info"
file = "/var/log/staffmon.log"

[api]
enabled = false
```

### Üretim Yapılandırması

```toml
# Üretim için hazır yapılandırma
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
api_key = "üretim-api-anahtarınız"
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
smtp_server = "smtp.şirketiniz.com"
smtp_port = 587
username = "uyarilar@şirketiniz.com"
password = "güvenli-şifre"
from_address = "uyarilar@şirketiniz.com"
to_addresses = ["ops@şirketiniz.com", "admin@şirketiniz.com"]

[security_settings]
enable_encryption = true
encryption_key = "32-byte-şifreleme-anahtarınız"
enable_audit_log = true
restrict_api_access = true
allowed_ips = ["10.0.0.0/8", "192.168.1.0/24"]
```

### Geliştirme Yapılandırması

```toml
# Geliştirme için debug loglama ile yapılandırma
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
api_key = "dev-api-anahtarı"
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

## Yapılandırma Doğrulama

Araç başlangıçta yapılandırma dosyalarını doğrular. Yaygın doğrulama hataları:

- Geçersiz dosya yolları
- Geçersiz port numaraları
- Geçersiz log seviyeleri
- Eksik gerekli alanlar
- Geçersiz boolean değerler

### Doğrulama Komutları

```bash
# Yapılandırma dosyasını doğrula
./staffmon --validate-config /path/to/config.toml

# Mevcut yapılandırmayı göster
./staffmon --show-config

# Varsayılan yapılandırma oluştur
./staffmon --generate-config > config.toml
```

## Yapılandırma Yeniden Yükleme

Şu anda yapılandırma değişiklikleri daemon'u yeniden başlatmayı gerektirir. Gelecek sürümlerde yapılandırma değişikliklerinin sıcak yeniden yüklenmesi desteklenecektir.

```bash
# Yapılandırma değişikliklerini uygulamak için daemon'u yeniden başlat
sudo systemctl restart staffmon

# Veya manuel olarak yeniden başlat
./staffmon --stop
./staffmon --start
```

## En İyi Uygulamalar

### Güvenlik

1. **Güçlü API anahtarları kullanın**: Kriptografik olarak güvenli API anahtarları oluşturun
2. **SSL/TLS etkinleştirin**: Üretimde API iletişimi için HTTPS kullanın
3. **API erişimini kısıtlayın**: API erişimini güvenilir IP adresleriyle sınırlayın
4. **Denetim logunu etkinleştirin**: Tüm yapılandırma değişikliklerini ve API erişimlerini loglayın
5. **Ortam değişkenlerini kullanın**: Hassas yapılandırmayı ortam değişkenlerinde saklayın

### Performans

1. **Güncelleme aralıklarını optimize edin**: İzleme sıklığını sistem kaynaklarıyla dengeleyin
2. **Sıkıştırmayı etkinleştirin**: Disk alanından tasarruf etmek için çıktı dosyalarını sıkıştırın
3. **Saklama süresini yapılandırın**: Uygun veri saklama süreleri belirleyin
4. **Daemon modunu kullanın**: Üretim ortamları için arka planda çalıştırın
5. **Kaynak kullanımını izleyin**: Kullanılmayan izleme özelliklerini devre dışı bırakın

### İzleme

1. **Uygun eşikler belirleyin**: Sisteminize göre uyarı eşiklerini yapılandırın
2. **Kritik servisleri izleyin**: İzlemede temel servisleri dahil edin
3. **Güvenlik izlemeyi etkinleştirin**: Güvenlikle ilgili metrikleri izleyin
4. **Uyarıları yapılandırın**: Yedeklilik için birden fazla uyarı kanalı kurun
5. **Düzenli bakım yapın**: Yapılandırmayı düzenli olarak gözden geçirin ve güncelleyin

## Yapılandırma Sorunlarını Giderme

### Yaygın Sorunlar

1. **İzin reddedildi hataları**: Uygun dosya izinlerini sağlayın
2. **Geçersiz yapılandırma**: TOML sözdizimi ve doğrulamayı kontrol edin
3. **Eksik bağımlılıklar**: Gerekli sistem paketlerini yükleyin
4. **Port çakışmaları**: API portunun kullanımda olmadığından emin olun
5. **Disk alanı**: Çıktı dosyası boyutlarını ve saklama süresini izleyin

### Hata Ayıklama Komutları

```bash
# Yapılandırma sözdizimini kontrol et
./staffmon --validate-config config.toml

# Debug loglama ile çalıştır
RUST_LOG=debug ./staffmon --config config.toml

# API bağlantısını test et
curl -H "Authorization: Bearer api-anahtarınız" http://localhost:8080/api/v1/system/info

# Log dosyalarını kontrol et
tail -f /var/log/staffmon.log
``` 