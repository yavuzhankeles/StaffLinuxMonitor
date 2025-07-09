# Sorun Giderme Kılavuzu

## Genel Bakış

Bu kılavuz, StaffLinuxMonitor ile karşılaşabileceğiniz yaygın sorunları ve çözümlerini içerir. Sorunları sistematik olarak teşhis etmek ve çözmek için adım adım yaklaşımlar sunar.

## Sorun Teşhis Süreci

### 1. Temel Kontroller

Herhangi bir sorunla karşılaştığınızda, önce bu temel kontrolleri yapın:

```bash
# Servis durumunu kontrol et
sudo systemctl status staffmon

# İşlemin çalışıp çalışmadığını kontrol et
ps aux | grep staffmon

# Port dinleme durumunu kontrol et
sudo netstat -tlnp | grep 8080
# veya
sudo ss -tlnp | grep 8080

# Log dosyalarını kontrol et
sudo tail -f /var/log/staffmon.log
```

### 2. Yapılandırma Doğrulama

```bash
# Yapılandırma dosyasını doğrula
sudo staffmon --validate-config /etc/staffmon/config.toml

# Yapılandırmayı göster
sudo staffmon --show-config

# Test modunda çalıştır
sudo staffmon --config /etc/staffmon/config.toml --test
```

## Yaygın Sorunlar ve Çözümleri

### Servis Başlamıyor

#### Belirtiler
- `systemctl start staffmon` komutu başarısız oluyor
- Servis durumu "failed" gösteriyor
- Log dosyalarında hata mesajları var

#### Olası Nedenler ve Çözümler

**1. İzin Sorunları**

```bash
# Dosya izinlerini kontrol et
ls -la /opt/staffmon/staffmon
ls -la /etc/staffmon/
ls -la /var/log/staffmon/

# İzinleri düzelt
sudo chown staffmon:staffmon /opt/staffmon/staffmon
sudo chmod +x /opt/staffmon/staffmon
sudo chown -R staffmon:staffmon /etc/staffmon
sudo chown -R staffmon:staffmon /var/log/staffmon
```

**2. Yapılandırma Hatası**

```bash
# Yapılandırma sözdizimini kontrol et
sudo staffmon --validate-config /etc/staffmon/config.toml

# Geçersiz değerleri kontrol et
grep -E "(true|false)" /etc/staffmon/config.toml
grep -E "[0-9]+" /etc/staffmon/config.toml
```

**3. Bağımlılık Sorunları**

```bash
# Gerekli kütüphaneleri kontrol et
ldd /opt/staffmon/staffmon

# Eksik kütüphaneleri yükle
sudo apt-get install libssl1.1 libc6
# veya CentOS/RHEL için
sudo yum install openssl-libs glibc
```

**4. Sistem Limitleri**

```bash
# Dosya tanımlayıcı limitlerini kontrol et
ulimit -n

# Limitleri artır
echo "staffmon soft nofile 65536" | sudo tee -a /etc/security/limits.conf
echo "staffmon hard nofile 65536" | sudo tee -a /etc/security/limits.conf
```

### API Erişilemiyor

#### Belirtiler
- `curl` komutları zaman aşımına uğruyor
- Web tarayıcısında bağlantı hatası
- "Connection refused" mesajları

#### Olası Nedenler ve Çözümler

**1. Servis Çalışmıyor**

```bash
# Servis durumunu kontrol et
sudo systemctl status staffmon

# Servisi yeniden başlat
sudo systemctl restart staffmon

# Logları kontrol et
sudo journalctl -u staffmon -f
```

**2. Yanlış Port Yapılandırması**

```bash
# Dinlenen portları kontrol et
sudo netstat -tlnp | grep staffmon

# Yapılandırmadaki portu kontrol et
grep "port" /etc/staffmon/config.toml

# Port çakışmasını kontrol et
sudo lsof -i :8080
```

**3. Güvenlik Duvarı Sorunu**

```bash
# UFW durumunu kontrol et (Ubuntu)
sudo ufw status

# UFW'de portu aç
sudo ufw allow 8080/tcp

# iptables durumunu kontrol et
sudo iptables -L | grep 8080

# iptables kuralı ekle
sudo iptables -A INPUT -p tcp --dport 8080 -j ACCEPT
```

**4. Host Bağlama Sorunu**

```bash
# Yapılandırmadaki host ayarını kontrol et
grep "host" /etc/staffmon/config.toml

# localhost yerine 0.0.0.0 kullan
sudo sed -i 's/host = "127.0.0.1"/host = "0.0.0.0"/' /etc/staffmon/config.toml

# Servisi yeniden başlat
sudo systemctl restart staffmon
```

### Yüksek CPU/Bellek Kullanımı

#### Belirtiler
- Sistem yavaş çalışıyor
- `top` komutunda staffmon yüksek kaynak kullanıyor
- Sistem donuyor veya yanıt vermiyor

#### Olası Nedenler ve Çözümler

**1. Çok Sık Güncelleme**

```bash
# Güncelleme aralığını kontrol et
grep "update_interval" /etc/staffmon/config.toml

# Aralığı artır (saniye cinsinden)
sudo sed -i 's/update_interval = 1/update_interval = 5/' /etc/staffmon/config.toml
```

**2. Çok Fazla Özellik Etkin**

```bash
# Etkin özellikleri kontrol et
grep -A 10 "\[features\]" /etc/staffmon/config.toml

# Kullanılmayan özellikleri devre dışı bırak
sudo sed -i 's/enable_process_monitoring = true/enable_process_monitoring = false/' /etc/staffmon/config.toml
```

**3. Büyük Log Dosyaları**

```bash
# Log dosyası boyutlarını kontrol et
du -sh /var/log/staffmon/*

# Log rotasyonunu kontrol et
sudo logrotate -d /etc/logrotate.d/staffmon

# Manuel log temizleme
sudo truncate -s 0 /var/log/staffmon.log
```

**4. Sistem Kaynakları Yetersiz**

```bash
# Sistem kaynaklarını kontrol et
free -h
df -h
nproc

# Yapılandırmayı optimize et
sudo tee -a /etc/staffmon/config.toml <<EOF

[performance]
max_threads = 2
buffer_size = 4096
enable_caching = false
cache_ttl = 30
EOF
```

### Disk Alanı Sorunları

#### Belirtiler
- "No space left on device" hataları
- Log dosyaları çok büyük
- Sistem yavaş çalışıyor

#### Olası Nedenler ve Çözümler

**1. Log Dosyaları Çok Büyük**

```bash
# Disk kullanımını kontrol et
df -h /var/log

# Büyük dosyaları bul
sudo find /var/log/staffmon -type f -size +100M

# Log rotasyonunu yapılandır
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

**2. Veri Dosyaları Çok Büyük**

```bash
# Veri dosyası boyutunu kontrol et
ls -lh /var/log/staffmon_data.json

# Sıkıştırmayı etkinleştir
sudo sed -i 's/enable_compression = false/enable_compression = true/' /etc/staffmon/config.toml

# Saklama süresini azalt
sudo sed -i 's/retention_days = 30/retention_days = 7/' /etc/staffmon/config.toml
```

**3. Disk Alanı Yetersiz**

```bash
# Disk kullanımını analiz et
sudo du -sh /* | sort -hr | head -10

# Eski dosyaları temizle
sudo find /var/log -name "*.log.*" -mtime +7 -delete
sudo find /tmp -mtime +7 -delete
```

### Ağ Bağlantı Sorunları

#### Belirtiler
- API istekleri zaman aşımına uğruyor
- Ağ metrikleri toplanamıyor
- Dış servislere bağlantı yok

#### Olası Nedenler ve Çözümler

**1. Ağ Arayüzü Sorunları**

```bash
# Ağ arayüzlerini kontrol et
ip addr show

# Ağ bağlantısını test et
ping -c 3 8.8.8.8

# DNS çözümlemesini test et
nslookup google.com
```

**2. Firewall Sorunları**

```bash
# Firewall durumunu kontrol et
sudo ufw status
# veya
sudo iptables -L

# Gerekli portları aç
sudo ufw allow 8080/tcp
sudo ufw allow out 53/tcp
sudo ufw allow out 53/udp
```

**3. Proxy Yapılandırması**

```bash
# Proxy ayarlarını kontrol et
echo $http_proxy
echo $https_proxy

# Proxy ayarlarını yapılandır
export http_proxy="http://proxy.company.com:8080"
export https_proxy="http://proxy.company.com:8080"
```

### Güvenlik Sorunları

#### Belirtiler
- API anahtarı hataları
- Erişim reddedildi mesajları
- SSL/TLS hataları

#### Olası Nedenler ve Çözümler

**1. API Anahtarı Sorunları**

```bash
# API anahtarını kontrol et
grep "api_key" /etc/staffmon/config.toml

# Yeni API anahtarı oluştur
NEW_API_KEY=$(openssl rand -hex 32)
sudo sed -i "s/api_key = \".*\"/api_key = \"$NEW_API_KEY\"/" /etc/staffmon/config.toml

# API anahtarını test et
curl -H "Authorization: Bearer $NEW_API_KEY" http://localhost:8080/api/v1/system/info
```

**2. SSL/TLS Sorunları**

```bash
# SSL sertifikalarını kontrol et
openssl x509 -in /etc/ssl/certs/staffmon.crt -text -noout

# Sertifika süresini kontrol et
openssl x509 -in /etc/ssl/certs/staffmon.crt -noout -dates

# Sertifikaları yenile
sudo certbot renew
```

**3. İzin Sorunları**

```bash
# Dosya izinlerini kontrol et
ls -la /etc/staffmon/
ls -la /var/log/staffmon/

# İzinleri düzelt
sudo chown -R staffmon:staffmon /etc/staffmon
sudo chmod 600 /etc/staffmon/config.toml
sudo chown -R staffmon:staffmon /var/log/staffmon
```

## Debug ve Loglama

### Debug Modunu Etkinleştirme

```bash
# Debug loglama ile çalıştır
RUST_LOG=debug sudo staffmon --config /etc/staffmon/config.toml

# Belirli modülleri debug et
RUST_LOG=staffmon::api=debug,staffmon::monitoring=debug sudo staffmon --config /etc/staffmon/config.toml
```

### Log Seviyelerini Yapılandırma

```toml
[logging]
level = "debug"  # trace, debug, info, warn, error
file = "/var/log/staffmon.log"
enable_console = true
enable_syslog = false
```

### Log Analizi

```bash
# Hata mesajlarını filtrele
sudo grep "ERROR" /var/log/staffmon.log

# Belirli zaman aralığındaki logları görüntüle
sudo journalctl -u staffmon --since "1 hour ago"

# Log dosyasını canlı takip et
sudo tail -f /var/log/staffmon.log | grep -E "(ERROR|WARN)"
```

## Performans Optimizasyonu

### Sistem Kaynaklarını İzleme

```bash
# CPU kullanımını izle
top -p $(pgrep staffmon)

# Bellek kullanımını izle
ps aux | grep staffmon

# Disk I/O'yu izle
iotop -p $(pgrep staffmon)

# Ağ kullanımını izle
iftop -i eth0
```

### Yapılandırma Optimizasyonu

```toml
# Performans için optimize edilmiş yapılandırma
[monitoring]
update_interval = 5  # Daha az sıklıkta güncelle
enable_daemon = true
enable_foreground = false

[performance]
max_threads = 2  # Thread sayısını azalt
buffer_size = 4096  # Buffer boyutunu azalt
enable_caching = true
cache_ttl = 60

[features]
enable_process_monitoring = false  # Kaynak yoğun özelliği devre dışı bırak
enable_security_monitoring = false  # Gereksiz özellikleri kapat
```

## Acil Durum Prosedürleri

### Servis Tamamen Çalışmıyor

```bash
# 1. Servisi durdur
sudo systemctl stop staffmon

# 2. İşlemi zorla sonlandır
sudo pkill -f staffmon

# 3. Log dosyalarını temizle
sudo truncate -s 0 /var/log/staffmon.log

# 4. Yapılandırmayı yedekle
sudo cp /etc/staffmon/config.toml /etc/staffmon/config.toml.backup

# 5. Varsayılan yapılandırma ile başlat
sudo staffmon --config /etc/staffmon/config.toml --daemon

# 6. Servis durumunu kontrol et
sudo systemctl status staffmon
```

### Disk Alanı Kritik

```bash
# 1. Disk kullanımını kontrol et
df -h

# 2. Büyük dosyaları bul ve sil
sudo find /var/log -name "*.log.*" -delete
sudo find /tmp -mtime +1 -delete

# 3. Log dosyalarını sıkıştır
sudo gzip /var/log/staffmon.log.*

# 4. Eski veri dosyalarını sil
sudo find /var/log/staffmon -name "*.json" -mtime +7 -delete

# 5. Servisi yeniden başlat
sudo systemctl restart staffmon
```

### API Güvenlik İhlali

```bash
# 1. API'yi geçici olarak devre dışı bırak
sudo sed -i 's/enabled = true/enabled = false/' /etc/staffmon/config.toml

# 2. Servisi yeniden başlat
sudo systemctl restart staffmon

# 3. Güvenlik loglarını kontrol et
sudo grep -i "unauthorized\|failed" /var/log/staffmon.log

# 4. Yeni API anahtarı oluştur
NEW_KEY=$(openssl rand -hex 32)
sudo sed -i "s/api_key = \".*\"/api_key = \"$NEW_KEY\"/" /etc/staffmon/config.toml

# 5. API'yi yeniden etkinleştir
sudo sed -i 's/enabled = false/enabled = true/' /etc/staffmon/config.toml

# 6. Servisi yeniden başlat
sudo systemctl restart staffmon
```

## Destek Alma

### Sorun Raporlama

Sorun yaşadığınızda, aşağıdaki bilgileri toplayın:

```bash
# Sistem bilgileri
uname -a
cat /etc/os-release
free -h
df -h

# StaffLinuxMonitor bilgileri
staffmon --version
systemctl status staffmon
journalctl -u staffmon --no-pager -n 100

# Yapılandırma (hassas bilgileri gizleyin)
grep -v "api_key\|password" /etc/staffmon/config.toml

# Log dosyaları
tail -n 100 /var/log/staffmon.log
```

### Topluluk Desteği

- [GitHub Issues](https://github.com/forniya/StaffLinuxMonitor/issues)
- [GitHub Discussions](https://github.com/forniya/StaffLinuxMonitor/discussions)
- [Wiki](https://github.com/forniya/StaffLinuxMonitor/wiki)

### Profesyonel Destek

Kurumsal kullanıcılar için profesyonel destek hizmetleri mevcuttur. Detaylar için [destek sayfasını](https://github.com/forniya/StaffLinuxMonitor/wiki/Support) ziyaret edin. 