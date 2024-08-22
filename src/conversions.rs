use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use image::ImageFormat;

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
