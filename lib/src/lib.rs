// src/lib.rs

#[derive(Debug)]
pub struct Sizes {
    pub bytes: String,
    pub kilobytes: String,
    pub megabytes: String,
    pub gigabytes: String,
}

impl Sizes {
    pub fn new(bytes: u64) -> Self {
        Sizes {
            bytes: format!("{} bytes", bytes),
            kilobytes: format!("{} kilobytes", bytes / 1000),
            megabytes: format!("{} megabytes", bytes / 1_000_000),
            gigabytes: format!("{} gigabytes", bytes / 1_000_000_000),
        }
    }
     pub fn to_kilobytes(&self) -> f64 {
        self.bytes.replace(" bytes", "").parse::<u64>().unwrap() as f64 / 1000.0
    }

    pub fn to_megabytes(&self) -> f64 {
        self.bytes.replace(" bytes", "").parse::<u64>().unwrap() as f64 / 1_000_000.0
    }

    pub fn to_gigabytes(&self) -> f64 {
        self.bytes.replace(" bytes", "").parse::<u64>().unwrap() as f64 / 1_000_000_000.0
    }
}

pub enum FileSize {
    Bytes(u64),
    KiloBytes(u64),
    MegaBytes(u64),
    GigaBytes(u64),
}

impl FileSize {
    pub fn format_size(&self) -> String {
        match self {
            FileSize::Bytes(bytes) => format!("{} bytes", bytes),
            FileSize::KiloBytes(kb) => format!("{:.2} KB", *kb as f64 / 1000.0),
            FileSize::MegaBytes(mb) => format!("{:.2} MB", *mb as f64 / 1000.0),
            FileSize::GigaBytes(gb) => format!("{:.2} GB", *gb as f64 / 1000.0),
        }
    }
}
