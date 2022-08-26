pub(crate) use eframe::egui::{
    FontDefinitions,
    self,
};

pub fn setup_font(ctx: &egui::Context) {
    let mut font: FontDefinitions = egui::FontDefinitions::default();

    font.font_data.insert (
        "inter".to_owned(),
        egui::FontData::from_static(include_bytes!("../assets/fonts/Inter-Regular.ttf")),
    );

    font
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "inter".to_owned());

    font
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("inter".to_owned());

    ctx.set_fonts(font); // sets inter the default font
}
