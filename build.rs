use std::{env, error::Error, fs::File, io::BufWriter};

use ico::{IconDir, IconDirEntry, IconImage};
use image::GenericImageView;
use winresource::WindowsResource;

fn convert_png_to_icon(png_path: &str, icon_path: &str) -> Result<(), Box<dyn Error>> {
    let png = image::open(png_path)?;
    let (width, height) = png.dimensions();

    let rgba_data = png.to_rgba8();
    let icon_image = IconImage::from_rgba_data(width, height, rgba_data.into_raw());
    let mut icon_dir = IconDir::new(ico::ResourceType::Icon);
    icon_dir.add_entry(IconDirEntry::encode(&icon_image)?);
    let file = File::create(icon_path)?;
    let mut writer = BufWriter::new(file);
    icon_dir.write(&mut writer)?;
    Ok(())

}

fn main() {
    //普通图片转换成icon
    convert_png_to_icon("./ui/images/logo.png", "./ui/images/logo.ico").expect("icon_set_fail1");
    if env::var("CARGO_CFG_WINDOWS").is_ok() {
        WindowsResource::new()
            .set_icon("./ui/images/logo.ico")
            .compile()
            .expect("icon_set_fail");
    }
    //icon图标嵌入二进制文件
    slint_build::compile("ui/app-window.slint").expect("Slint build failed");
}
