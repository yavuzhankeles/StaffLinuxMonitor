# Sürüm Notları

## v1.0.2 (2024-03-19)
- Arka plan (daemon) desteği eklendi
- Loglama sistemi eklendi
- Hata yönetimi geliştirildi
- PID dosyası desteği

### Yeni Özellikler
- Arka planda çalışma
- Detaylı loglama
- Hata logları
- PID dosyası yönetimi
- Güvenli daemon başlatma

### Teknik İyileştirmeler
- Log4rs entegrasyonu
- Daemonize desteği
- Hata yakalama ve loglama
- Güvenlik iyileştirmeleri

## v1.0.1 (2024-03-19)
- API entegrasyonu eklendi
- REST API desteği
- JSON formatında veri alışverişi
- HTTP/HTTPS desteği

### Yeni Özellikler
- API endpoint'leri
- Veri gönderme/alma
- API kimlik doğrulama
- Rate limiting
- Hata yönetimi

### Teknik İyileştirmeler
- API istemci kütüphanesi
- HTTP istek yönetimi
- JSON serileştirme/deserileştirme
- API yanıt işleme
- Bağlantı yönetimi

## v1.0.0 (2024-03-19)
- İlk kararlı sürüm
- Temel sistem izleme özellikleri eklendi
- 2 saniyede bir otomatik güncelleme
- JSON formatında raporlama
- GitHub Actions ile otomatik derleme
- Statik binary desteği (musl)

### Yeni Özellikler
- CPU kullanımı izleme
- Bellek kullanımı izleme
- Disk kullanımı izleme
- Ağ arayüzleri izleme
- Servis durumu izleme
- Güvenlik durumu izleme
- Donanım bilgileri
- Sistem yükü izleme
- Kullanıcı erişim izleme
- Otomatik JSON raporlama

### Teknik İyileştirmeler
- GitHub Actions entegrasyonu
- Statik binary derleme (musl)
- CI/CD pipeline geliştirmeleri
- Performans optimizasyonları
- Hata yönetimi geliştirmeleri

### Bilinen Sorunlar
- CPU sıcaklık bilgisi bazı sistemlerde alınamayabilir
- Bazı servis durumları sistem yapılandırmasına bağlı olarak alınamayabilir

### Gelecek Planları
- Web arayüzü geliştirme
- E-posta bildirimleri
- Telegram/Discord entegrasyonu
- Daha detaylı raporlama
- Grafik arayüzü
- Docker desteği
- Windows desteği 