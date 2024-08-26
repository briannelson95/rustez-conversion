use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use image::{ImageError, ImageFormat};

pub fn convert_jpg_to_png(input_path: &str, output_path: &str) -> Result<(), String> {
    // Open the input image file
    let img = image::open(&Path::new(input_path))
        .map_err(|e| format!("Failed to open image: {}", e))?;

    // Convert the image to PNG and save it to the output path
    let output_file = File::create(&Path::new(output_path))
        .map_err(|e| format!("Failed to create output file: {}", e))?;
    
    let mut writer = BufWriter::new(output_file);

    img.write_to(&mut writer, ImageFormat::Png)
        .map_err(|e| format!("Failed to convert image: {}", e))?;

    Ok(())
}

/// Convert JPG to WebP
pub fn convert_jpg_to_webp(input_path: &str, output_path: &str) -> Result<(), ImageError> {
    let img = image::open(input_path)?;
    let mut output_file = std::fs::File::create(output_path)?;
    img.write_to(&mut output_file, ImageFormat::WebP)?;
    Ok(())
}

/// Convert PNG to JPG
pub fn convert_png_to_jpg(input_path: &str, output_path: &str) -> Result<(), ImageError> {
    let img = image::open(input_path)?;
    let mut output_file = std::fs::File::create(output_path)?;
    img.write_to(&mut output_file, ImageFormat::Jpeg)?; // Quality set to 80
    Ok(())
}

/// Convert PNG to WebP
pub fn convert_png_to_webp(input_path: &str, output_path: &str) -> Result<(), ImageError> {
    let img = image::open(input_path)?;
    let mut output_file = std::fs::File::create(output_path)?;
    img.write_to(&mut output_file, ImageFormat::WebP)?;
    Ok(())
}

/// Convert WebP to JPG
pub fn convert_webp_to_jpg(input_path: &str, output_path: &str) -> Result<(), ImageError> {
    let img = image::open(input_path)?;
    let mut output_file = std::fs::File::create(output_path)?;
    img.write_to(&mut output_file, ImageFormat::Jpeg)?; // Quality set to 80
    Ok(())
}

/// Convert WebP to PNG
pub fn convert_webp_to_png(input_path: &str, output_path: &str) -> Result<(), ImageError> {
    let img = image::open(input_path)?;
    let mut output_file = std::fs::File::create(output_path)?;
    img.write_to(&mut output_file, ImageFormat::Png)?;
    Ok(())
}
