use eframe::egui;

use crate::fonts::setup_font;

/*
 * #[derive(PartialEq)]
 * enum ColorStyle {
 *     LightMode,
 *     DarkMode
 * }
 */

#[derive(Default)]
pub struct UIToolkit {}

impl UIToolkit {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_font(&cc.egui_ctx);

        Self::default()
    }
}

impl eframe::App for UIToolkit {
    fn update(&mut self,
            ctx:    &egui::Context,
            _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("AvdanOS UI Toolkit");
            ui.end_row();

            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Label: ");
                ui.label("Hello, World!");
            });
            ui.end_row();
            
            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Button: ");
                if ui.add(egui::Button::new("Dark mode!")).clicked() {
                    ctx.set_visuals(egui::Visuals::dark());
                }
                if ui.add(egui::Button::new("Light mode!")).clicked() {
                    ctx.set_visuals(egui::Visuals::light());
                }
            });
        });
    }
}
