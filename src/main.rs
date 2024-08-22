mod components;
mod conversions;
mod helpers;

use components::{toggle_button, convert_button};
use eframe::egui;

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
                            self.output_path = file.parent().unwrap().display().to_string();
                            self.output_filename = helpers::get_filename_without_extension(&self.selected_file)
                                .unwrap_or_else(|| String::from("default_filename"));
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

                    // if ui.button("Convert").clicked() {
                    //     if let Err(err) = conversions::convert_jpg_to_png(&self.selected_file) {
                    //         ui.label(format!("Error: {}", err));
                    //     } else {
                    //         ui.label("Conversion successful!");
                    //     }
                    // }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
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

                // Display the selected file path
                ui.label("File path:");
                ui.text_edit_singleline(&mut self.selected_file);

                // Display the output filename, allowing the user to edit it
                ui.label("Output filename:");
                ui.text_edit_singleline(&mut self.output_filename);

                ui.label("Output path:");
                ui.horizontal(|ui| {
                    ui.text_edit_singleline(&mut self.output_path);
                    if ui.button("Choose Output Folder").clicked() {
                        if let Some(output_folder) = rfd::FileDialog::new().pick_folder() {
                            self.output_path = output_folder.display().to_string();
                        }
                    }
                });

                ui.label("Preview:");
                if !self.selected_file.is_empty() {
                    // Display the image thumbnail
                    if let Some(thumbnail) = &self.image_thumbnail {
                        ui.image(thumbnail); // Display the thumbnail
                    }
                }

                // Display conversion status
                if let Some(status) = &self.conversion_status {
                    ui.separator();
                    ui.label(status);
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
