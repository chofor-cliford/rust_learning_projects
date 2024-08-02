use std::str::FromStr;

#[derive(Debug)]
struct Sizes {
    bytes: String,
    kilobytes: String,
    megabytes: String,
    gigabytes: String,
}

impl Sizes {
    fn new(bytes: u64) -> Self {
        Sizes {
            bytes: format!("{} bytes", bytes),
            kilobytes: format!("{} kilobytes", bytes / 1000),
            megabytes: format!("{} megabytes", bytes / 1_000_000),
            gigabytes: format!("{} gigabytes", bytes / 1_000_000_000),
        }
    }
}

enum FileSize {
    Bytes(u64),
    KiloBytes(u64),
    MegaBytes(u64),
    GigaBytes(u64),
}

impl FileSize {
    fn format_size(&self) -> String {
        match self {
            FileSize::Bytes(bytes) => format!("{} bytes", bytes),
            FileSize::KiloBytes(kb) => format!("{:.2} KB", *kb as f64 / 1000.0),
            FileSize::MegaBytes(mb) => format!("{:.2} MB", *mb as f64 / 1000.0),
            FileSize::GigaBytes(gb) => format!("{:.2} GB", *gb as f64 / 1000.0),
        }
    }
}

fn parse_input(input: &str) -> (u64, &str) {
    let parts: Vec<&str> = input.split_whitespace().collect();
    let size = u64::from_str(parts[0]).expect("Invalid number");
    let unit = parts[1];
    (size, unit)
}

fn main() {
    let input = "4100000000 bytes".to_string();
    let (size, unit) = parse_input(&input);

    let filesize = match unit.to_lowercase().as_str() {
        "bytes" => FileSize::Bytes(size),
        "kb" | "kilobytes" => FileSize::KiloBytes(size * 1000),
        "mb" | "megabytes" => FileSize::MegaBytes(size * 1_000_000),
        "gb" | "gigabytes" => FileSize::GigaBytes(size * 1_000_000_000),
        _ => panic!("Unsupported unit"),
    };

    println!("{}", filesize.format_size());

    let sizes = match filesize {
        FileSize::Bytes(b) => Sizes::new(b),
        FileSize::KiloBytes(kb) => Sizes::new(kb * 1000),
        FileSize::MegaBytes(mb) => Sizes::new(mb * 1_000_000),
        FileSize::GigaBytes(gb) => Sizes::new(gb * 1_000_000_000),
    };

    println!("{:?}", sizes);
}
