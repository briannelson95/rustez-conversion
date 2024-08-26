use eframe::egui;

pub fn toggle_button(ui: &mut egui::Ui, is_active: bool, label: &str) -> bool {
    let text = egui::RichText::new(label).color(if is_active {
        egui::Color32::WHITE // Active text color
    } else {
        egui::Color32::BLACK // Inactive text color
    });

    ui.add_sized(
        [80.0, 25.0], // Button size
        egui::Button::new(text)
            .fill(if is_active {
                egui::Color32::from_rgb(100, 150, 255) // Active color
            } else {
                egui::Color32::from_rgb(200, 200, 200) // Inactive color
            })
    ).clicked()
}

pub fn convert_button(ui: &mut egui::Ui) -> egui::Response {
    let text = egui::RichText::new("▶ Convert").color(egui::Color32::WHITE);
    ui.add_sized(
        [80.0, 25.0],
        egui::Button::new(text).fill(egui::Color32::from_rgb(60, 100, 205)))
}

pub fn open_image_button(ui: &mut egui::Ui) -> egui::Response {
    let text = egui::RichText::new("🖼️ Open Image").color(egui::Color32::WHITE);
    ui.add_sized(
        [80.0, 25.0], 
        egui::Button::new(text)
    )
}