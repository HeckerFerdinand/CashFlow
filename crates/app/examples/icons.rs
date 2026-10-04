//! Regenerates the application icons from `assets/logo.png`:
//! `assets/icon.png` (window icon, 256×256) and `assets/icon.ico` (Windows,
//! several sizes). Run after changing the logo:
//!
//!     cargo run -p cashflow-app --example icons

use image::codecs::ico::{IcoEncoder, IcoFrame};
use image::{ExtendedColorType, ImageBuffer, Rgba, RgbaImage, imageops};
use std::path::PathBuf;

fn main() {
    let assets = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets");
    let logo = image::open(assets.join("logo.png")).expect("assets/logo.png").to_rgba8();

    // Square canvas with a white rounded tile behind the line-art logo, so the
    // icon stays visible on dark task bars.
    let size = 1024u32;
    let mut canvas: RgbaImage = ImageBuffer::from_pixel(size, size, Rgba([0, 0, 0, 0]));
    let radius = 200.0f32;
    for y in 0..size {
        for x in 0..size {
            let (fx, fy) = (x as f32 + 0.5, y as f32 + 0.5);
            let cx = fx.clamp(radius, size as f32 - radius);
            let cy = fy.clamp(radius, size as f32 - radius);
            let distance = ((fx - cx).powi(2) + (fy - cy).powi(2)).sqrt();
            let alpha = (radius + 0.5 - distance).clamp(0.0, 1.0);
            if alpha > 0.0 {
                canvas.put_pixel(x, y, Rgba([255, 255, 255, (alpha * 255.0) as u8]));
            }
        }
    }
    let inner = (size as f32 * 0.78) as u32;
    let scale = inner as f32 / logo.width().max(logo.height()) as f32;
    let (w, h) = ((logo.width() as f32 * scale) as u32, (logo.height() as f32 * scale) as u32);
    let scaled = imageops::resize(&logo, w, h, imageops::FilterType::Lanczos3);
    imageops::overlay(&mut canvas, &scaled, ((size - w) / 2) as i64, ((size - h) / 2) as i64);

    let png = imageops::resize(&canvas, 256, 256, imageops::FilterType::Lanczos3);
    png.save(assets.join("icon.png")).unwrap();

    let frames: Vec<IcoFrame> = [16u32, 24, 32, 48, 64, 128, 256]
        .iter()
        .map(|&s| {
            let img = imageops::resize(&canvas, s, s, imageops::FilterType::Lanczos3);
            IcoFrame::as_png(img.as_raw(), s, s, ExtendedColorType::Rgba8).unwrap()
        })
        .collect();
    let file = std::fs::File::create(assets.join("icon.ico")).unwrap();
    IcoEncoder::new(file).encode_images(&frames).unwrap();
    println!("assets/icon.png and assets/icon.ico written");
}
