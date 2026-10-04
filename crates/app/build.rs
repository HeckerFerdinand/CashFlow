use std::path::PathBuf;

fn main() {
    let config = slint_build::CompilerConfiguration::new().with_style("fluent".into());
    slint_build::compile_with_config("ui/app.slint", config).expect("Slint build failed");

    // Windows: embed the application icon (generated from assets/logo.png) and
    // version info into the .exe.
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        let icon = PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("cashflow.ico");
        write_icon(&icon);
        #[cfg(windows)]
        {
            let mut resource = winresource::WindowsResource::new();
            resource.set_icon(icon.to_str().unwrap());
            resource.set("ProductName", "CashFlow");
            resource.set("FileDescription", "CashFlow – Lohnabrechnung");
            resource.compile().expect("Windows resource");
        }
    }
    println!("cargo:rerun-if-changed=../../assets/logo.png");
}

/// Square, multi-size .ico built from the logo (keeps the logo the single source).
fn write_icon(path: &std::path::Path) {
    use image::{ImageBuffer, Rgba, imageops};
    let logo = image::open("../../assets/logo.png").expect("assets/logo.png").to_rgba8();
    let side = logo.width().max(logo.height());
    let mut square = ImageBuffer::from_pixel(side, side, Rgba([0, 0, 0, 0]));
    imageops::overlay(&mut square, &logo, ((side - logo.width()) / 2) as i64, ((side - logo.height()) / 2) as i64);
    let icon = imageops::resize(&square, 256, 256, imageops::FilterType::Lanczos3);
    icon.save_with_format(path, image::ImageFormat::Ico).expect("write icon");
}
