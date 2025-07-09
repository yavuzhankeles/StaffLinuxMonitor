use log4rs::{
    append::file::FileAppender,
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
};
use std::path::Path;

pub fn init_logger() -> Result<(), Box<dyn std::error::Error>> {
    // Log dizinini oluştur
    let log_dir = Path::new("logs");
    if !log_dir.exists() {
        std::fs::create_dir(log_dir)?;
    }

    // Log dosyası yolunu oluştur
    let log_file = log_dir.join("staffmon.log");

    // Log appender'ı oluştur
    let file_appender = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S)} [{l}] {m}{n}")))
        .build(log_file)?;

    // Log yapılandırmasını oluştur
    let config = Config::builder()
        .appender(Appender::builder().build("file", Box::new(file_appender)))
        .build(Root::builder().appender("file").build(log::LevelFilter::Info))?;

    // Log sistemini başlat
    log4rs::init_config(config)?;

    Ok(())
} 