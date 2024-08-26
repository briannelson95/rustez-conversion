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
                            let thumbnail = image.thumbnail(300, 300);
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
                        let output_file = format!("{}/{}.png", self.output_path, self.output_filename);
                        match conversions::convert_jpg_to_png(&self.selected_file, &output_file) {
                            Ok(_) => {
                                self.conversion_status = Some("Conversion successful!".to_string());
                            }
                            Err(err) => {
                                self.conversion_status = Some(format!("Error: {}", err));
                            }
                        }
                    }
                });
            });

            ui.separator();

            // Source Path
            ui.horizontal(|ui| {
                ui.label("Source:");
                ui.add(egui::TextEdit::singleline(&mut self.selected_file).desired_width(f32::INFINITY));
            });

            ui.separator();

            // Conversion Frame
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    // Conversion options (left column)
                    ui.label("Format:");
                    egui::ComboBox::from_label("")
                        .width(250.0)
                        .selected_text("PNG".to_string())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut String::from("PNG"), "PNG".to_string(), "PNG");
                            ui.selectable_value(&mut String::from("JPG"), "JPG".to_string(), "JPG");
                            ui.selectable_value(&mut String::from("WebP"), "WebP".to_string(), "WebP");
                        });

                    // Future settings space
                });

                ui.vertical(|ui| {
                    // Image preview (right column)
                    ui.label("Preview:");
                    if let Some(thumbnail) = &self.image_thumbnail {
                        ui.image(thumbnail); // Display the thumbnail
                    }
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
