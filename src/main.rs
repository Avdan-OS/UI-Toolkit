use eframe::egui;

mod fonts;

fn main() {
    let native_options = eframe::NativeOptions::default();

    eframe::run_native (
        "AvdanOS toolkit",
        native_options,
        Box::new(|cc| Box::new(UIToolkit::new(cc)))
    );
}

#[derive(Default)]
struct UIToolkit {}

impl UIToolkit {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        fonts::setup_font(&cc.egui_ctx);

        Self::default()
    }
}

impl eframe::App for UIToolkit {
   fn update(&mut self,
             ctx:    &egui::Context,
             _frame: &mut eframe::Frame) {
       egui::CentralPanel::default().show(ctx, |ui| {
           ui.heading("Hello, World!");
       });
   }
}
