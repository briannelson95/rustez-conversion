mod components;
mod conversions;
mod helpers;

use components::{convert_button, open_image_button, toggle_button};
use eframe::egui;
use egui::{Align, Layout};

#[derive(Default)]
struct ImageConverterApp {
    is_batch_mode: bool, // State variable to track the conversion mode
    selected_file: String,
    output_path: String,
    output_filename: String,
    image_thumbnail: Option<egui::TextureHandle>,
    conversion_status: Option<String>,
    selected_conversion: String,
}

impl eframe::App for ImageConverterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Header
            ui.horizontal(|ui| {
                // Conversion mode buttons
                ui.horizontal(|ui| {
                    if toggle_button(ui, !self.is_batch_mode, "Single Conversion") {
                        self.is_batch_mode = false;
                    }
                    if toggle_button(ui, self.is_batch_mode, "Batch Conversion") {
                        self.is_batch_mode = true;
                    }
                });
            });

            ui.horizontal(|ui| {
                // Open Image button with icon
                if open_image_button(ui).clicked() {
                    if let Some(file) = rfd::FileDialog::new()
                        .add_filter("Image Files", &["jpg", "jpeg", "png", "webp"])
                        .pick_file()
                    {
                        self.selected_file = file.display().to_string();
                        self.output_path = file.parent().unwrap().display().to_string();
                        self.output_filename = helpers::get_filename_without_extension(&self.selected_file)
                            .unwrap_or_else(|| String::from("default_filename"));

                        // Load image thumbnail
                        if let Ok(image) = image::open(&self.selected_file) {
                            let thumbnail = image.thumbnail(400, 300);
                            let color_image = egui::ColorImage::from_rgba_unmultiplied(
                                [thumbnail.width() as usize, thumbnail.height() as usize],
                                &thumbnail.to_rgba8(),
                            );
                            self.image_thumbnail = Some(ctx.load_texture(
                                "thumbnail",
                                color_image,
                                egui::TextureOptions::LINEAR,
                            ));
                        }
                    }
                }

                // Convert button with icon, aligned to the right
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if convert_button(ui).clicked() {
                        let output_file = format!("{}/{}.{}", self.output_path, self.output_filename, self.selected_conversion.to_lowercase());
                        
                        match self.selected_conversion.as_str() {
                            "PNG" => {
                                if self.selected_file.ends_with(".jpg") || self.selected_file.ends_with(".jpeg") {
                                    match conversions::convert_jpg_to_png(&self.selected_file, &output_file) {
                                        Ok(_) => self.conversion_status = Some("Conversion successful!".to_string()),
                                        Err(err) => self.conversion_status = Some(format!("Error: {}", err)),
                                    }
                                } else if self.selected_file.ends_with(".webp") {
                                    match conversions::convert_webp_to_png(&self.selected_file, &output_file) {
                                        Ok(_) => self.conversion_status = Some("Conversion successful!".to_string()),
                                        Err(err) => self.conversion_status = Some(format!("Error: {}", err)),
                                    }
                                }
                            }
                            "JPG" => {
                                if self.selected_file.ends_with(".png") {
                                    match conversions::convert_png_to_jpg(&self.selected_file, &output_file) {
                                        Ok(_) => self.conversion_status = Some("Conversion successful!".to_string()),
                                        Err(err) => self.conversion_status = Some(format!("Error: {}", err)),
                                    }
                                } else if self.selected_file.ends_with(".webp") {
                                    match conversions::convert_webp_to_jpg(&self.selected_file, &output_file) {
                                        Ok(_) => self.conversion_status = Some("Conversion successful!".to_string()),
                                        Err(err) => self.conversion_status = Some(format!("Error: {}", err)),
                                    }
                                }
                            }
                            "WebP" => {
                                if self.selected_file.ends_with(".jpg") || self.selected_file.ends_with(".jpeg") {
                                    match conversions::convert_jpg_to_webp(&self.selected_file, &output_file) {
                                        Ok(_) => self.conversion_status = Some("Conversion successful!".to_string()),
                                        Err(err) => self.conversion_status = Some(format!("Error: {}", err)),
                                    }
                                } else if self.selected_file.ends_with(".png") {
                                    match conversions::convert_png_to_webp(&self.selected_file, &output_file) {
                                        Ok(_) => self.conversion_status = Some("Conversion successful!".to_string()),
                                        Err(err) => self.conversion_status = Some(format!("Error: {}", err)),
                                    }
                                }
                            }
                            _ => {
                                self.conversion_status = Some("Unsupported conversion type.".to_string());
                            }
                        }
                    }
                });
            });

            ui.separator();

            // Source Path///////////////////////////
            ui.horizontal(|ui| {
                egui::Frame::none()
                    .outer_margin(8.0)
                    .show(ui, |ui| {
                        ui.label("Source:");
                        ui.add(egui::TextEdit::singleline(&mut self.selected_file).desired_width(f32::INFINITY));
                    });
            });

            ui.separator();

            // Frame for conversion options and preview
            let margin = 8.0;
            let padding = 10.0;
            egui::Frame::none()
                .fill(ui.visuals().extreme_bg_color) // Set the background color of the frame (grey in dark mode, light grey in light mode)
                .rounding(egui::Rounding::same(8.0))
                .outer_margin(margin)
                .inner_margin(padding) // Use the tuple for margins
                .show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.set_min_height(ui.available_height() - 50.0); // Adjust the height of the frame

                        ui.horizontal(|ui| {
                            // Conversion options (left column, 1/3 of the frame)
                            ui.vertical(|ui| {
                                ui.label("Format:");
                                egui::ComboBox::from_label("")
                                    .width(250.0)
                                    .selected_text(self.selected_conversion.clone())
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(&mut self.selected_conversion, "PNG".to_string(), "PNG");
                                        ui.selectable_value(&mut self.selected_conversion, "JPG".to_string(), "JPG");
                                        ui.selectable_value(&mut self.selected_conversion, "WebP".to_string(), "WebP");
                                    });
                            
                                // Future settings space
                            });

                            // Image preview (right column, 2/3 of the frame)
                            ui.vertical(|ui| {
                                ui.label("Preview:");
                                if let Some(thumbnail) = &self.image_thumbnail {
                                    ui.allocate_space(ui.available_size());
                                    ui.image(thumbnail); // Display the thumbnail
                                } else {
                                    ui.allocate_space(ui.available_size()); // Occupy the space even if no image is selected
                                }
                            });
                        });
                    });
                });

            ui.separator();

            // Output Section
            ui.horizontal(|ui| {
                ui.label("Save as:");
                ui.text_edit_singleline(&mut self.output_filename);

                ui.label("To:");
                ui.text_edit_singleline(&mut self.output_path);

                if ui.button("Browse").clicked() {
                    if let Some(output_folder) = rfd::FileDialog::new().pick_folder() {
                        self.output_path = output_folder.display().to_string();
                    }
                }
            });

            ui.separator();

            // Display conversion status
            if let Some(status) = &self.conversion_status {
                ui.label(status);
            }
        });
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "RustEZ Conversion",
        options,
        Box::new(|_cc| Ok(Box::new(ImageConverterApp::default()))),
    );
}
