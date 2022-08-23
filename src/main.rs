#![allow(unused_imports)]
use eframe::*;

mod fonts;
mod toolkit;

fn main() {
    let native_options: NativeOptions = eframe::NativeOptions::default();

    eframe::run_native (
        "AvdanOS toolkit",
        native_options,
        Box::new(|cc: &CreationContext| Box::new(toolkit::UIToolkitDemo::new(cc)))
    );
}
