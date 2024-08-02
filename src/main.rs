// src/main.rs

use lib::{FileSize, Sizes};

fn main() {
    let size = 24000000;
    let filesize = match size {
        0..=999 => FileSize::Bytes(size),
        1000..=999_999 => FileSize::KiloBytes(size / 1000),
        1_000_000..=999_999_999 => FileSize::MegaBytes(size / 1_000_000),
        _ => FileSize::GigaBytes(size / 1_000_000_000),
    };
    println!("{}", filesize.format_size());

    let sizes = Sizes::new(size);
    println!("{:?}", sizes);

    println!("In KB: {:.2}", sizes.to_kilobytes());
    println!("In MB: {:.2}", sizes.to_megabytes());
    println!("In GB: {:.2}", sizes.to_gigabytes());
}

