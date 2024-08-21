mod components;

use components::{convert_button, toggle_button};
use eframe::egui;

#[derive(Default)]
struct ImageConverterApp {
    is_batch_mode: bool, // State variable to track the conversion mode
    selected_file: String,
    image_thumbnail: Option<egui::TextureHandle>,
}

impl eframe::App for ImageConverterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("RustEZ Conversion");

            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                        // Handle open file
                    }
                    if ui.button("Exit").clicked() {
                        // Handle exit
                    }
                });
            });

            ui.separator();

            ui.horizontal(|ui| {
                if toggle_button(ui, !self.is_batch_mode, "Single Conversion") {
                    self.is_batch_mode = false;
                }
                if toggle_button(ui, self.is_batch_mode, "Batch Conversion") {
                    self.is_batch_mode = true;
                }
            });

            ui.separator();

            if self.is_batch_mode {
                // Batch Conversion UI
                ui.vertical(|ui| {
                    ui.label("Batch Image Conversion:");
                    if ui.button("Add Files").clicked() {
                        // Handle adding multiple files
                    }

                    ui.group(|ui| {
                        // Display a list of selected files
                        ui.label("Selected Files:");
                        // List file names here
                    });

                    if ui.button("Convert All").clicked() {
                        // Handle batch conversion
                    }
                });
            } else {
                // Single Conversion UI
                ui.horizontal(|ui| {
                    ui.label("Convert a Single Image:");
                    if ui.button("Choose File").clicked() {
                        if let Some(file) = rfd::FileDialog::new()
                            .add_filter("Image Files", &["jpg", "jpeg", "png", "webp"])
                            .pick_file()
                        {
                            self.selected_file = file.display().to_string();
                            // Load the image and create a thumbnail
                            if let Ok(image) = image::open(&self.selected_file) {
                                let thumbnail = image.thumbnail(300, 300); // Resize to 100x100 pixels
                                
                                // Convert the image to an egui texture
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
                    ui.label("Output Format:");
                    egui::ComboBox::from_label("")
                        .selected_text("JPG".to_string()) // Convert to String
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut String::from("JPG"), "JPG".to_string(), "JPG");
                            ui.selectable_value(&mut String::from("PNG"), "PNG".to_string(), "PNG");
                            ui.selectable_value(&mut String::from("WebP"), "WebP".to_string(), "WebP");
                        });

                    convert_button(ui, "Convert");
                });

                ui.separator();

                // Display the selected file path
                ui.label("File path:");
                ui.text_edit_singleline(&mut self.selected_file);
                ui.label("Preview:");

                if !self.selected_file.is_empty() {
                    // Display the image thumbnail
                    if let Some(thumbnail) = &self.image_thumbnail {
                        ui.image(thumbnail); // Display the thumbnail
                    }
                }
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
