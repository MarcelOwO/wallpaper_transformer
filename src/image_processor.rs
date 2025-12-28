use image::*;
use std::path::{Path, PathBuf};
use webp::*;

use std::fs;

pub(crate) fn process_image(image_path: &PathBuf, out_path: &PathBuf, (w, h): (u32, u32)) {
    if !image_path.is_file() {
        println!("{} is not a file", image_path.display());
        return;
    }

    let Some(file_name_with_extension) = image_path.file_name() else {
        println!("Could not get name of {}", image_path.display());
        return;
    };

    let file_name_as_path = PathBuf::from(file_name_with_extension);

    let file_name = file_name_as_path.file_stem().unwrap();

    let mut img: DynamicImage = match image::open(image_path) {
        Ok(value) => value,
        Err(_) => {
            println!("Could not open {} as image", image_path.display());
            return;
        }
    };

    let resized = img.resize_exact(w, h, imageops::FilterType::Triangle);

    let rgb8_img = DynamicImage::ImageRgb8(resized.to_rgb8());

    let encoder: Encoder = match Encoder::from_image(&rgb8_img) {
        Ok(value) => value,
        Err(_) => {
            println!("Couldn't open encoder for {}", image_path.display());
            return;
        }
    };

    let webp: WebPMemory = encoder.encode(90f32);

    let mut output_path = out_path.clone().join(file_name);
    output_path.set_extension("webp");

    if let Err(e) = fs::write(&output_path, &*webp) {
        println!("Failed to write {} : {}", output_path.display(), e);
        return;
    }
}
