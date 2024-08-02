enum FileSize {
    Btyes(u64),
    KiloBytes(u64),
    MegaBytes(u64),
    GigaBytes(u64),
}

impl FileSize {

fn format_size(&self) -> String {
     match self {     
        FileSize::Btyes(bytes) => format!("{} bytes", bytes),
        FileSize::KiloBytes(kb) => format!("{:.2} KB", *kb as f64/1000.0),
        FileSize::MegaBytes(mb) => format!("{:.2} MB", *mb as f64/1000.0),
        FileSize::GigaBytes(gb) => format!("{:.2} GB", *gb as f64/1000.0),
        }
    }
}

fn main() {
    let size = 4100000000;
    let filesize = match size {
        0..=999 => FileSize::Btyes(size),
        1000..=999_999 => FileSize::KiloBytes(size / 1000),
        1_000_000..=999_999_999 => FileSize::MegaBytes(size / 1_000_000),
        _ => FileSize::GigaBytes(size / 1_000_000_000),
    };

    println!("File size: {}", filesize.format_size());
}
