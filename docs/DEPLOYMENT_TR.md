# Kurulum ve Dağıtım Kılavuzu

## Genel Bakış

Bu kılavuz, StaffLinuxMonitor'ün tek sunucu kurulumlarından büyük ölçekli üretim dağıtımlarına kadar çeşitli ortamlarda kurulumunu kapsar.

## Ön Gereksinimler

### Sistem Gereksinimleri

- **İşletim Sistemi**: Linux (Ubuntu 18.04+, CentOS 7+, RHEL 7+, Debian 9+)
- **Mimari**: x86_64, ARM64
- **Bellek**: Minimum 512MB RAM, Önerilen 2GB+
- **Depolama**: Minimum 100MB disk alanı, Önerilen 10GB+
- **Ağ**: Güncellemeler ve API iletişimi için internet erişimi

### Yazılım Bağımlılıkları

- **Rust Runtime**: 1.70+ (kaynaktan derleme için)
- **Sistem Kütüphaneleri**: libc, libssl, libcurl
- **Sistem Araçları**: systemctl, journalctl (systemd entegrasyonu için)

## Kurulum Yöntemleri

### Yöntem 1: Binary Kurulumu (Tavsiye Edilen)

#### Adım 1: Binary İndir

```bash
# Kurulum dizini oluştur
sudo mkdir -p /opt/staffmon
cd /opt/staffmon

# Son sürümü indir
wget https://github.com/forniya/StaffLinuxMonitor/releases/latest/download/staffmon

# Çalıştırılabilir yap
chmod +x staffmon

# Sembolik bağlantı oluştur
sudo ln -sf /opt/staffmon/staffmon /usr/local/bin/staffmon
```

#### Adım 2: Yapılandırma Oluştur

```bash
# Yapılandırma dizini oluştur
sudo mkdir -p /etc/staffmon

# Yapılandırma dosyası oluştur
sudo tee /etc/staffmon/config.toml > /dev/null <<EOF
[monitoring]
update_interval = 2
enable_daemon = true
enable_foreground = false
enable_json_output = true
enable_api = true

[logging]
level = "info"
file = "/var/log/staffmon.log"
max_size = "100MB"
backup_count = 5
enable_console = false
enable_syslog = true

[api]
enabled = true
host = "127.0.0.1"
port = 8080
api_key = "$(openssl rand -hex 32)"
timeout_seconds = 30
retry_count = 3
rate_limit = 100

[output]
json_file = "/var/log/staffmon_data.json"
enable_compression = true
max_file_size = "1GB"
retention_days = 30

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
EOF
```

#### Adım 3: Sistem Kullanıcısı Oluştur

```bash
# Sistem kullanıcısı oluştur
sudo useradd -r -s /bin/false -d /opt/staffmon staffmon

# Sahipliği ayarla
sudo chown -R staffmon:staffmon /opt/staffmon
sudo chown -R staffmon:staffmon /etc/staffmon
```

#### Adım 4: Log Dizini Oluştur

```bash
# Log dizini oluştur
sudo mkdir -p /var/log/staffmon
sudo chown staffmon:staffmon /var/log/staffmon
```

### Yöntem 2: Kaynaktan Derleme

#### Adım 1: Rust Kur

```bash
# Rust kur
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Derleme bağımlılıklarını kur
sudo apt-get update
sudo apt-get install -y build-essential pkg-config libssl-dev libcurl4-openssl-dev
```

#### Adım 2: Projeyi Derle

```bash
# Repository'yi klonla
git clone https://github.com/forniya/StaffLinuxMonitor.git
cd StaffLinuxMonitor

# Release versiyonunu derle
cargo build --release

# Binary'yi kur
sudo cp target/release/staffmon /usr/local/bin/
sudo chmod +x /usr/local/bin/staffmon
```

## Systemd Servis Kurulumu

### Servis Dosyası Oluştur

```bash
sudo tee /etc/systemd/system/staffmon.service > /dev/null <<EOF
[Unit]
Description=StaffLinuxMonitor Sistem İzleme Aracı
Documentation=https://github.com/forniya/StaffLinuxMonitor
After=network.target

[Service]
Type=simple
User=staffmon
Group=staffmon
ExecStart=/usr/local/bin/staffmon --config /etc/staffmon/config.toml
ExecReload=/bin/kill -HUP \$MAINPID
Restart=always
RestartSec=10
StandardOutput=journal
StandardError=journal
SyslogIdentifier=staffmon

# Güvenlik ayarları
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=/var/log/staffmon /etc/staffmon

# Kaynak limitleri
LimitNOFILE=65536
LimitNPROC=4096

[Install]
WantedBy=multi-user.target
EOF
```

### Servisi Etkinleştir ve Başlat

```bash
# systemd'yi yeniden yükle
sudo systemctl daemon-reload

# Servisi etkinleştir
sudo systemctl enable staffmon

# Servisi başlat
sudo systemctl start staffmon

# Durumu kontrol et
sudo systemctl status staffmon
```

## Docker Dağıtımı

### Dockerfile

```dockerfile
FROM rust:1.70-alpine as builder

# Derleme bağımlılıklarını kur
RUN apk add --no-cache musl-dev openssl-dev

# Kaynak kodu kopyala
WORKDIR /app
COPY . .

# Uygulamayı derle
RUN cargo build --release

# Çalışma zamanı aşaması
FROM alpine:latest

# Çalışma zamanı bağımlılıklarını kur
RUN apk add --no-cache libc6-compat openssl

# Kullanıcı oluştur
RUN addgroup -g 1001 -S staffmon && \
    adduser -S staffmon -u 1001

# Binary'yi kopyala
COPY --from=builder /app/target/release/staffmon /usr/local/bin/
RUN chmod +x /usr/local/bin/staffmon

# Dizinleri oluştur
RUN mkdir -p /etc/staffmon /var/log/staffmon && \
    chown -R staffmon:staffmon /etc/staffmon /var/log/staffmon

# Root olmayan kullanıcıya geç
USER staffmon

# API portunu aç
EXPOSE 8080

# Sağlık kontrolü
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD wget --no-verbose --tries=1 --spider http://localhost:8080/api/v1/health || exit 1

# Uygulamayı çalıştır
CMD ["/usr/local/bin/staffmon", "--config", "/etc/staffmon/config.toml"]
```

### Docker Compose

```yaml
version: '3.8'

services:
  staffmon:
    build: .
    container_name: staffmon
    restart: unless-stopped
    ports:
      - "8080:8080"
    volumes:
      - ./config.toml:/etc/staffmon/config.toml:ro
      - staffmon_logs:/var/log/staffmon
      - /proc:/host/proc:ro
      - /sys:/host/sys:ro
      - /etc:/host/etc:ro
    environment:
      - RUST_LOG=info
    cap_add:
      - SYS_ADMIN
    security_opt:
      - seccomp:unconfined
    network_mode: host

volumes:
  staffmon_logs:
```

### Docker ile Çalıştır

```bash
# Derle ve çalıştır
docker-compose up -d

# Logları kontrol et
docker-compose logs -f staffmon

# API'ye eriş
curl http://localhost:8080/api/v1/system/info
```

## Kubernetes Dağıtımı

### ConfigMap

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: staffmon-config
  namespace: monitoring
data:
  config.toml: |
    [monitoring]
    update_interval = 2
    enable_daemon = false
    enable_foreground = true
    enable_json_output = true
    enable_api = true

    [logging]
    level = "info"
    file = "/var/log/staffmon.log"
    enable_console = true

    [api]
    enabled = true
    host = "0.0.0.0"
    port = 8080
    api_key = "api-anahtarınız"

    [output]
    json_file = "/var/log/staffmon_data.json"
    enable_compression = true
    max_file_size = "100MB"
    retention_days = 7

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
```

### Deployment

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: staffmon
  namespace: monitoring
  labels:
    app: staffmon
spec:
  replicas: 1
  selector:
    matchLabels:
      app: staffmon
  template:
    metadata:
      labels:
        app: staffmon
    spec:
      serviceAccountName: staffmon-sa
      containers:
      - name: staffmon
        image: staffmon:latest
        imagePullPolicy: Always
        ports:
        - containerPort: 8080
          name: api
        volumeMounts:
        - name: config
          mountPath: /etc/staffmon
          readOnly: true
        - name: logs
          mountPath: /var/log/staffmon
        - name: proc
          mountPath: /host/proc
          readOnly: true
        - name: sys
          mountPath: /host/sys
          readOnly: true
        - name: etc
          mountPath: /host/etc
          readOnly: true
        env:
        - name: RUST_LOG
          value: "info"
        resources:
          requests:
            memory: "128Mi"
            cpu: "100m"
          limits:
            memory: "512Mi"
            cpu: "500m"
        livenessProbe:
          httpGet:
            path: /api/v1/health
            port: 8080
          initialDelaySeconds: 30
          periodSeconds: 30
        readinessProbe:
          httpGet:
            path: /api/v1/health
            port: 8080
          initialDelaySeconds: 5
          periodSeconds: 10
      volumes:
      - name: config
        configMap:
          name: staffmon-config
      - name: logs
        emptyDir: {}
      - name: proc
        hostPath:
          path: /proc
      - name: sys
        hostPath:
          path: /sys
      - name: etc
        hostPath:
          path: /etc
```

### Service

```yaml
apiVersion: v1
kind: Service
metadata:
  name: staffmon-service
  namespace: monitoring
spec:
  selector:
    app: staffmon
  ports:
  - name: api
    port: 8080
    targetPort: 8080
  type: ClusterIP
```

### ServiceAccount

```yaml
apiVersion: v1
kind: ServiceAccount
metadata:
  name: staffmon-sa
  namespace: monitoring
```

### RBAC (İsteğe Bağlı)

```yaml
apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRole
metadata:
  name: staffmon-role
rules:
- apiGroups: [""]
  resources: ["nodes", "pods", "services"]
  verbs: ["get", "list", "watch"]
- apiGroups: [""]
  resources: ["nodes/proxy"]
  verbs: ["get", "list", "watch"]

---
apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRoleBinding
metadata:
  name: staffmon-role-binding
roleRef:
  apiGroup: rbac.authorization.k8s.io
  kind: ClusterRole
  name: staffmon-role
subjects:
- kind: ServiceAccount
  name: staffmon-sa
  namespace: monitoring
```

## Üretim Dağıtımı

### Yüksek Erişilebilirlik Kurulumu

#### Yük Dengeleyici Yapılandırması

```nginx
upstream staffmon_backend {
    server 192.168.1.10:8080;
    server 192.168.1.11:8080;
    server 192.168.1.12:8080;
}

server {
    listen 80;
    server_name staffmon.domaininiz.com;

    location / {
        proxy_pass http://staffmon_backend;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

#### SSL/TLS Yapılandırması

```bash
# SSL sertifikası oluştur
sudo certbot certonly --standalone -d staffmon.domaininiz.com

# Yapılandırmayı güncelle
sudo tee -a /etc/staffmon/config.toml <<EOF

[api]
enable_ssl = true
ssl_cert = "/etc/letsencrypt/live/staffmon.domaininiz.com/fullchain.pem"
ssl_key = "/etc/letsencrypt/live/staffmon.domaininiz.com/privkey.pem"
EOF
```

### İzleme ve Uyarı

#### Prometheus Yapılandırması

```yaml
# prometheus.yml
global:
  scrape_interval: 15s

scrape_configs:
  - job_name: 'staffmon'
    static_configs:
      - targets: ['localhost:8080']
    metrics_path: '/api/v1/metrics'
    scheme: 'http'
    scrape_interval: 30s
```

#### Grafana Dashboard

```json
{
  "dashboard": {
    "title": "StaffLinuxMonitor Dashboard",
    "panels": [
      {
        "title": "CPU Kullanımı",
        "type": "graph",
        "targets": [
          {
            "expr": "staffmon_cpu_usage_percent",
            "legendFormat": "CPU Kullanımı %"
          }
        ]
      },
      {
        "title": "Bellek Kullanımı",
        "type": "graph",
        "targets": [
          {
            "expr": "staffmon_memory_usage_percent",
            "legendFormat": "Bellek Kullanımı %"
          }
        ]
      }
    ]
  }
}
```

### Yedekleme ve Kurtarma

#### Yedekleme Scripti

```bash
#!/bin/bash
# backup-staffmon.sh

BACKUP_DIR="/backup/staffmon"
DATE=$(date +%Y%m%d_%H%M%S)

# Yedekleme dizini oluştur
mkdir -p "$BACKUP_DIR"

# Yapılandırmayı yedekle
cp -r /etc/staffmon "$BACKUP_DIR/config_$DATE"

# Logları yedekle
tar -czf "$BACKUP_DIR/logs_$DATE.tar.gz" /var/log/staffmon

# Veriyi yedekle
cp /var/log/staffmon_data.json "$BACKUP_DIR/data_$DATE.json"

# Eski yedekleri temizle (son 7 günü tut)
find "$BACKUP_DIR" -type d -mtime +7 -exec rm -rf {} \;
find "$BACKUP_DIR" -name "*.tar.gz" -mtime +7 -delete
find "$BACKUP_DIR" -name "*.json" -mtime +7 -delete

echo "Yedekleme tamamlandı: $BACKUP_DIR"
```

#### Cron Job

```bash
# Crontab'a ekle
0 2 * * * /usr/local/bin/backup-staffmon.sh
```

## Güvenlik Hususları

### Güvenlik Duvarı Yapılandırması

```bash
# UFW (Ubuntu)
sudo ufw allow 8080/tcp
sudo ufw allow from 192.168.1.0/24 to any port 8080

# iptables
sudo iptables -A INPUT -p tcp --dport 8080 -s 192.168.1.0/24 -j ACCEPT
sudo iptables -A INPUT -p tcp --dport 8080 -j DROP
```

### API Güvenliği

```toml
# Gelişmiş güvenlik yapılandırması
[api]
enabled = true
host = "127.0.0.1"  # Sadece localhost'a bağla
port = 8080
api_key = "çok-güvenli-api-anahtarınız"
enable_ssl = true
enable_cors = false
rate_limit = 60  # Oran limitini azalt

[security_settings]
enable_encryption = true
encryption_key = "32-byte-şifreleme-anahtarınız"
enable_audit_log = true
restrict_api_access = true
allowed_ips = ["127.0.0.1", "192.168.1.0/24"]
```

### SELinux Yapılandırması (RHEL/CentOS)

```bash
# SELinux politikası oluştur
sudo tee staffmon.te <<EOF
module staffmon 1.0;

require {
    type staffmon_t;
    type staffmon_exec_t;
    type var_log_t;
    type etc_t;
    class file { read write execute };
    class dir { read write };
}

allow staffmon_t var_log_t:file { read write };
allow staffmon_t etc_t:file { read };
EOF

# Politikayı derle ve yükle
sudo make -f /usr/share/selinux/devel/Makefile staffmon.pp
sudo semodule -i staffmon.pp
```

## Sorun Giderme

### Yaygın Sorunlar

#### Servis Başlamıyor

```bash
# Servis durumunu kontrol et
sudo systemctl status staffmon

# Logları kontrol et
sudo journalctl -u staffmon -f

# Yapılandırmayı kontrol et
sudo staffmon --validate-config /etc/staffmon/config.toml

# İzinleri kontrol et
ls -la /opt/staffmon/
ls -la /etc/staffmon/
ls -la /var/log/staffmon/
```

#### API Erişilemiyor

```bash
# Servisin dinlediğini kontrol et
sudo netstat -tlnp | grep 8080

# Yerel olarak API'yi test et
curl -H "Authorization: Bearer api-anahtarınız" http://127.0.0.1:8080/api/v1/system/info

# Güvenlik duvarını kontrol et
sudo ufw status
sudo iptables -L
```

#### Yüksek Kaynak Kullanımı

```bash
# İşlem kaynaklarını kontrol et
ps aux | grep staffmon
top -p $(pgrep staffmon)

# Log dosyası boyutlarını kontrol et
du -sh /var/log/staffmon/*

# Yapılandırmayı optimize et
# update_interval'i azalt
# Kullanılmayan özellikleri devre dışı bırak
# Sıkıştırmayı etkinleştir
```

### Performans Optimizasyonu

#### Sistem Limitleri

```bash
# Dosya tanımlayıcı limitlerini artır
echo "staffmon soft nofile 65536" | sudo tee -a /etc/security/limits.conf
echo "staffmon hard nofile 65536" | sudo tee -a /etc/security/limits.conf

# İşlem limitlerini artır
echo "staffmon soft nproc 4096" | sudo tee -a /etc/security/limits.conf
echo "staffmon hard nproc 4096" | sudo tee -a /etc/security/limits.conf
```

#### Kernel Parametreleri

```bash
# İzleme için optimize et
echo "vm.swappiness=10" | sudo tee -a /etc/sysctl.conf
echo "vm.dirty_ratio=15" | sudo tee -a /etc/sysctl.conf
echo "vm.dirty_background_ratio=5" | sudo tee -a /etc/sysctl.conf

# Değişiklikleri uygula
sudo sysctl -p
```

## Bakım

### Düzenli Bakım Görevleri

```bash
#!/bin/bash
# maintenance.sh

# Logları döndür
sudo logrotate /etc/logrotate.d/staffmon

# Eski verileri temizle
find /var/log/staffmon -name "*.json" -mtime +30 -delete

# Sistem paketlerini güncelle
sudo apt-get update && sudo apt-get upgrade -y

# Servisi yeniden başlat
sudo systemctl restart staffmon

# Servis sağlığını kontrol et
curl -f http://localhost:8080/api/v1/health || echo "Servis sağlıksız"
```

### İzleme Scripti

```bash
#!/bin/bash
# monitor-staffmon.sh

# Servisin çalışıp çalışmadığını kontrol et
if ! systemctl is-active --quiet staffmon; then
    echo "StaffLinuxMonitor servisi çalışmıyor!"
    systemctl restart staffmon
    # Uyarı gönder
fi

# API sağlığını kontrol et
if ! curl -f -s http://localhost:8080/api/v1/health > /dev/null; then
    echo "StaffLinuxMonitor API yanıt vermiyor!"
    # Uyarı gönder
fi

# Disk alanını kontrol et
DISK_USAGE=$(df /var/log/staffmon | tail -1 | awk '{print $5}' | sed 's/%//')
if [ "$DISK_USAGE" -gt 90 ]; then
    echo "Disk kullanımı yüksek: ${DISK_USAGE}%"
    # Uyarı gönder
fi
```

## Destek ve Kaynaklar

### Belgeler
- [API Dökümanı](API_TR.md)
- [Yapılandırma Kılavuzu](CONFIGURATION_TR.md)
- [Sorun Giderme Kılavuzu](TROUBLESHOOTING_TR.md)

### Topluluk
- [GitHub Issues](https://github.com/forniya/StaffLinuxMonitor/issues)
- [GitHub Discussions](https://github.com/forniya/StaffLinuxMonitor/discussions)
- [Wiki](https://github.com/forniya/StaffLinuxMonitor/wiki)

### İzleme Araçları
- [Prometheus](https://prometheus.io/)
- [Grafana](https://grafana.com/)
- [AlertManager](https://prometheus.io/docs/alerting/latest/alertmanager/) 