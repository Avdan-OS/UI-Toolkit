#![allow(unused_imports)]
use eframe::*;
use toolkit::UIToolkitDemo;

mod fonts;
mod toolkit;

fn main() {
    let native_options: NativeOptions = eframe::NativeOptions::default();

    eframe::run_native (
        "AvdanOS UI Toolkit Demo",
        native_options,
        Box::new(|cc: &CreationContext| Box::new(UIToolkitDemo::new(cc)))
    );
}
