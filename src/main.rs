mod settings;

use image::error::ImageResult;
use image::{DynamicImage, Rgba, RgbaImage};
use log::{debug, info};
use settings::SettingData;
use std::env;

fn main() {
    let setting_data = SettingData::load("./settings.toml");

    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    for file in setting_data.get_files() {
        let file_name = file.0;
        let output_file_name = file.1;
        if let Ok(image_data) = load_image(&file_name) {
            // debug!("Hello, world!: {:?}", image_data);
            //
            let rgba_image = image_data.to_rgba8();
            let width = rgba_image.width();
            let height = rgba_image.height();

            let mut output_image = RgbaImage::new(width, height);

            for (x, y, pixel) in rgba_image.enumerate_pixels() {
                output_image.put_pixel(
                    x,
                    y,
                    Rgba([
                        pixel.0[0],
                        pixel.0[1],
                        pixel.0[2],
                        setting_data.get_alpha_value(pixel.0[3]),
                    ]),
                );
            }
            output_image
                .save_with_format(output_file_name, image::ImageFormat::Png)
                .unwrap();
        } else {
            info!("Hello, world!");
        }
    }
}

fn load_image(file_name: &String) -> ImageResult<DynamicImage> {
    let image_data = image::open(file_name)?;
    Ok(image_data)
}
