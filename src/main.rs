#![allow(unused_imports)]
use eframe::*;

mod fonts;
mod toolkit;

fn main() {
    let native_options = eframe::NativeOptions::default();

    eframe::run_native (
        "AvdanOS toolkit",
        native_options,
        Box::new(|cc| Box::new(toolkit::UIToolkit::new(cc)))
    );
}
