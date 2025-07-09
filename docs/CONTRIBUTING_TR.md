# Katkıda Bulunma Kılavuzu

## Hoş Geldiniz! 🎉

StaffLinuxMonitor projesine katkıda bulunmak istediğiniz için teşekkür ederiz! Bu kılavuz, projeye nasıl katkıda bulunabileceğinizi açıklar.

## İçindekiler

- [Katkıda Bulunma Yolları](#katkıda-bulunma-yolları)
- [Geliştirme Ortamı Kurulumu](#geliştirme-ortamı-kurulumu)
- [Kod Yazma Kuralları](#kod-yazma-kuralları)
- [Test Yazma](#test-yazma)
- [Pull Request Süreci](#pull-request-süreci)
- [Raporlama ve İletişim](#raporlama-ve-iletişim)
- [Lisans](#lisans)

## Katkıda Bulunma Yolları

### 🐛 Hata Raporlama

Hata bulduğunuzda, lütfen bir [GitHub Issue](https://github.com/forniya/StaffLinuxMonitor/issues) oluşturun. Hata raporunuzda şunları dahil edin:

- **İşletim Sistemi**: Ubuntu 20.04, CentOS 8, vb.
- **StaffLinuxMonitor Sürümü**: `staffmon --version`
- **Hata Açıklaması**: Ne oldu ve ne olması gerekiyordu?
- **Adımlar**: Hatayı yeniden oluşturmak için adımlar
- **Log Dosyaları**: İlgili hata mesajları
- **Ekran Görüntüleri**: Varsa

### 💡 Özellik İstekleri

Yeni özellik önerileriniz için [GitHub Discussions](https://github.com/forniya/StaffLinuxMonitor/discussions) kullanın. Önerinizde şunları açıklayın:

- **Özellik Açıklaması**: Ne yapması gerekiyor?
- **Kullanım Senaryosu**: Hangi durumlarda faydalı olacak?
- **Alternatifler**: Mevcut çözümler neler?
- **Öncelik**: Ne kadar önemli?

### 📚 Dokümantasyon

Dokümantasyon iyileştirmeleri her zaman memnuniyetle karşılanır:

- README güncellemeleri
- API dokümantasyonu
- Kurulum kılavuzları
- Örnek yapılandırmalar
- Çeviri katkıları

### 🔧 Kod Katkıları

Kod katkıları için:

1. [Fork](https://github.com/forniya/StaffLinuxMonitor/fork) oluşturun
2. Feature branch oluşturun
3. Değişikliklerinizi yapın
4. Testlerinizi yazın
5. Pull Request gönderin

## Geliştirme Ortamı Kurulumu

### Ön Gereksinimler

```bash
# Rust (1.70+)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Sistem bağımlılıkları (Ubuntu/Debian)
sudo apt-get update
sudo apt-get install -y build-essential pkg-config libssl-dev libcurl4-openssl-dev

# Sistem bağımlılıkları (CentOS/RHEL)
sudo yum groupinstall "Development Tools"
sudo yum install openssl-devel libcurl-devel
```

### Projeyi Klonlama

```bash
# Repository'yi klonla
git clone https://github.com/forniya/StaffLinuxMonitor.git
cd StaffLinuxMonitor

# Geliştirme bağımlılıklarını yükle
cargo build

# Testleri çalıştır
cargo test
```

### IDE Kurulumu

**VS Code (Tavsiye Edilen)**

```bash
# Rust eklentilerini yükle
code --install-extension rust-lang.rust-analyzer
code --install-extension vadimcn.vscode-lldb
code --install-extension serayuzgur.crates
```

**IntelliJ IDEA / CLion**

- Rust eklentisini yükleyin
- Cargo.toml dosyasını açın
- Projeyi Rust projesi olarak tanımlayın

## Kod Yazma Kuralları

### Rust Kod Stili

Projemiz [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/style/naming/README.html)'a uyar:

```rust
// ✅ Doğru
pub struct SystemInfo {
    hostname: String,
    os_version: String,
    kernel_version: String,
}

impl SystemInfo {
    pub fn new() -> Self {
        Self {
            hostname: String::new(),
            os_version: String::new(),
            kernel_version: String::new(),
        }
    }
}

// ❌ Yanlış
pub struct system_info {
    hostname: String,
    osVersion: String,
    kernel_version: String,
}
```

### Dosya Organizasyonu

```
src/
├── main.rs              # Ana giriş noktası
├── lib.rs               # Kütüphane giriş noktası
├── api/                 # API modülü
│   ├── mod.rs
│   ├── routes.rs
│   └── handlers.rs
├── monitoring/          # İzleme modülü
│   ├── mod.rs
│   ├── cpu.rs
│   ├── memory.rs
│   └── disk.rs
├── config/              # Yapılandırma modülü
│   ├── mod.rs
│   └── settings.rs
└── utils/               # Yardımcı fonksiyonlar
    ├── mod.rs
    └── helpers.rs
```

### Fonksiyon Yazma Kuralları

```rust
/// Sistem bilgilerini toplar ve döndürür
/// 
/// # Arguments
/// * `config` - Yapılandırma ayarları
/// 
/// # Returns
/// * `Result<SystemInfo, Error>` - Sistem bilgileri veya hata
/// 
/// # Examples
/// ```
/// let config = Config::default();
/// let info = collect_system_info(&config)?;
/// ```
pub fn collect_system_info(config: &Config) -> Result<SystemInfo, Error> {
    // İmplementasyon
}
```

### Hata Yönetimi

```rust
// ✅ Doğru - Özel hata türleri kullan
#[derive(Debug, thiserror::Error)]
pub enum MonitoringError {
    #[error("CPU bilgisi alınamadı: {0}")]
    CpuError(String),
    #[error("Bellek bilgisi alınamadı: {0}")]
    MemoryError(String),
    #[error("Disk bilgisi alınamadı: {0}")]
    DiskError(String),
}

// ❌ Yanlış - Generic Error kullanma
pub fn get_cpu_info() -> Result<CpuInfo, Box<dyn std::error::Error>> {
    // ...
}
```

### Loglama

```rust
use log::{info, warn, error, debug};

// ✅ Doğru - Uygun log seviyeleri kullan
pub fn start_monitoring(config: &Config) -> Result<(), Error> {
    info!("İzleme başlatılıyor...");
    
    if config.update_interval < 1 {
        warn!("Güncelleme aralığı çok düşük, 1 saniyeye ayarlanıyor");
    }
    
    match initialize_monitoring(config) {
        Ok(_) => {
            info!("İzleme başarıyla başlatıldı");
            Ok(())
        }
        Err(e) => {
            error!("İzleme başlatılamadı: {}", e);
            Err(e)
        }
    }
}
```

## Test Yazma

### Birim Testleri

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu_usage_calculation() {
        let cpu_info = CpuInfo {
            usage_percent: 50.0,
            temperature: Some(45.0),
            cores: 4,
        };
        
        assert_eq!(cpu_info.usage_percent, 50.0);
        assert_eq!(cpu_info.cores, 4);
        assert!(cpu_info.temperature.is_some());
    }

    #[test]
    fn test_invalid_cpu_usage() {
        let result = CpuInfo::new(-10.0);
        assert!(result.is_err());
    }
}
```

### Entegrasyon Testleri

```rust
// tests/integration_test.rs
use staffmon::api::start_server;
use staffmon::config::Config;

#[tokio::test]
async fn test_api_endpoints() {
    let config = Config::default();
    let server = start_server(config).await.unwrap();
    
    // API endpoint'lerini test et
    let client = reqwest::Client::new();
    let response = client
        .get("http://localhost:8080/api/v1/system/info")
        .send()
        .await
        .unwrap();
    
    assert_eq!(response.status(), 200);
}
```

### Test Çalıştırma

```bash
# Tüm testleri çalıştır
cargo test

# Belirli testleri çalıştır
cargo test test_cpu_usage

# Test coverage (gerekli: cargo-tarpaulin)
cargo tarpaulin --out Html

# Benchmark testleri
cargo bench
```

## Pull Request Süreci

### 1. Issue Oluşturma

Önce bir [Issue](https://github.com/forniya/StaffLinuxMonitor/issues) oluşturun:

- **Bug fix**: Hangi hatayı düzeltiyorsunuz?
- **Feature**: Hangi özelliği ekliyorsunuz?
- **Documentation**: Hangi dokümantasyonu güncelliyorsunuz?

### 2. Branch Oluşturma

```bash
# Ana branch'i güncelle
git checkout main
git pull origin main

# Feature branch oluştur
git checkout -b feature/your-feature-name
# veya
git checkout -b fix/your-bug-fix
```

### 3. Değişiklikleri Yapma

```bash
# Değişikliklerinizi yapın
# ...

# Değişiklikleri stage'le
git add .

# Commit oluştur
git commit -m "feat: CPU sıcaklık izleme özelliği eklendi

- CPU sıcaklık sensörü desteği eklendi
- Sıcaklık uyarıları eklendi
- Dokümantasyon güncellendi

Closes #123"
```

### 4. Commit Mesaj Kuralları

[Conventional Commits](https://www.conventionalcommits.org/) standardını kullanın:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

**Türler:**
- `feat`: Yeni özellik
- `fix`: Hata düzeltmesi
- `docs`: Dokümantasyon değişiklikleri
- `style`: Kod stili değişiklikleri
- `refactor`: Kod refactoring
- `test`: Test ekleme/düzenleme
- `chore`: Bakım görevleri

**Örnekler:**
```
feat(api): REST API endpoint'leri eklendi
fix(monitoring): CPU kullanım hesaplama hatası düzeltildi
docs(readme): Kurulum talimatları güncellendi
test(cpu): CPU izleme testleri eklendi
```

### 5. Pull Request Gönderme

1. GitHub'da Pull Request oluşturun
2. Template'i doldurun:

```markdown
## Değişiklik Türü
- [ ] Bug fix
- [ ] Yeni özellik
- [ ] Dokümantasyon güncellemesi
- [ ] Test ekleme

## Açıklama
Bu PR ne yapıyor?

## Test Edildi
- [ ] Birim testleri geçiyor
- [ ] Entegrasyon testleri geçiyor
- [ ] Manuel test yapıldı

## Ekran Görüntüleri
Varsa ekran görüntüleri ekleyin

## Checklist
- [ ] Kod stili kurallarına uygun
- [ ] Dokümantasyon güncellendi
- [ ] Testler eklendi
- [ ] Commit mesajları conventional commits'e uygun
```

### 6. Code Review

- En az bir maintainer'ın onayı gerekli
- CI/CD testleri geçmeli
- Code coverage düşmemeli

## Raporlama ve İletişim

### İletişim Kanalları

- **GitHub Issues**: Hata raporları ve özellik istekleri
- **GitHub Discussions**: Genel tartışmalar ve sorular

### Topluluk Kuralları

1. **Saygılı olun**: Herkesin fikrine saygı gösterin
2. **Yapıcı olun**: Eleştirilerinizi yapıcı şekilde yapın
3. **Yardımcı olun**: Diğer katkıda bulunanlara yardım edin
4. **Öğrenmeye açık olun**: Yeni fikirler ve yaklaşımlar için açık olun

## Lisans

Bu projeye katkıda bulunarak, katkılarınızın [MIT License](LICENSE) altında lisanslanacağını kabul etmiş olursunuz.

## Teşekkürler

Katkıda bulunan herkese teşekkür ederiz! İsimleriniz [CONTRIBUTORS.md](CONTRIBUTORS.md) dosyasında listelenecektir.

---

**Sorularınız mı var?** [GitHub Discussions](https://github.com/forniya/StaffLinuxMonitor/discussions)'da sorun! 