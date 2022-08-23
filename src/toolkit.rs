use eframe::egui;

use crate::egui::Ui;
use crate::fonts::setup_font;

#[derive(Default)]
pub struct UIToolkitDemo;

impl UIToolkitDemo {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_font(&cc.egui_ctx);

        Self::default()
    }
}

impl eframe::App for UIToolkitDemo {
    fn update(&mut self,
            ctx:    &egui::Context,
            _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui: &mut Ui| {
            ui.heading(r#"AvdanOS UI Toolkit Demo"#);
            ui.end_row();

            ui.separator();

            ui.horizontal(|ui: &mut Ui| {
                ui.label("Label: ");
                ui.label("Hello, World!");
            });
            ui.end_row();
            
            ui.separator();

            ui.horizontal(|ui: &mut Ui| {
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
