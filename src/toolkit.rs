use eframe::egui;
use eframe::egui::style::WidgetVisuals;

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
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui: &mut Ui| {
            ui.heading(r#"AvdanOS UI Toolkit Demo"#);
            ui.end_row();
            
            ui.horizontal(|ui: &mut Ui| {
                ui.label("Hello, AvdanOS Community!");
                ui.label("Check out our GitHub !");
            });
            ui.hyperlink("https://github.com/Avdan-OS");
            ui.end_row();
            
            ui.separator();

            ui.collapsing("Click to see buttons !", |ui| { // buttons
                if ui.add(egui::Button::new("Dark mode!")).clicked() {
                    ctx.set_visuals(egui::Visuals::dark());
                }
                if ui.add(egui::Button::new("Light mode!")).clicked() {
                    ctx.set_visuals(egui::Visuals::light());
                }
            }); // buttons

            ui.collapsing("Click to see sliders !", |ui| { // sliders 
                ui.label("AvdanOS Coolness : ");
                ui.add(egui::Slider::new(&mut 0, 0..=1000),);
                ui.label("Rust Coolness : ");
                ui.add(egui::Slider::new(&mut 0, 0..=1000));
            }); // sliders 

            ui.collapsing("Click to see checkboxes !", |ui| { // checkboxes
                ui.checkbox(&mut true, "AvdanOS is cool !");
                ui.checkbox(&mut true, "We need more developers !");
            }); // sliders

            ui.collapsing("Click to see a dragvalue !", |ui| { // drag values
                ui.label("Dragvalues :");
                ui.add(egui::DragValue::new(&mut 1).speed(0.1));
                ui.add(egui::DragValue::new(&mut 10).speed(1.5));
            }); // drag values

            
            ui.collapsing("Click to see what is hidden!", |ui| { // hidden thing
                ui.label("Not much, as it turns out");
            }); // hidden thing
            
            ui.collapsing("Counting Thing!", |ui| {
                let counter = &mut 1;
                ui.label("Count and stuff :");
                ui.horizontal(|ui| {
                    if ui.button("-").clicked() {
                        *counter -= 1;
                    }
                    ui.label(counter.to_string());
                    if ui.button("+").clicked() {
                        *counter += 1;
                    }
                }); // ui horizontal
            }); // ui collapsing
            ui.separator();
        }); // central panel
    }
    // those are the default members don't touch them (they're pretty much helper functions so)
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}

    fn on_exit_event(&mut self) -> bool {
        true
    }

    fn on_exit(&mut self, _gl: &eframe::glow::Context) {}

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }

    fn max_size_points(&self) -> egui::Vec2 {
        egui::Vec2::INFINITY
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> egui::Rgba {
        // NOTE: a bright gray makes the shadows of the windows look weird.
        // We use a bit of transparency so that if the user switches on the
        // `transparent()` option they get immediate results.
        egui::Color32::from_rgba_unmultiplied(12, 12, 12, 180).into()

        // _visuals.window_fill() would also be a natural choice
    }

    fn persist_native_window(&self) -> bool {
        true
    }

    fn persist_egui_memory(&self) -> bool {
        true
    }

    fn warm_up_enabled(&self) -> bool {
        false
    } // central panel
}