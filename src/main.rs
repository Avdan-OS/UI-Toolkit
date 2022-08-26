use eframe::*;
use toolkit::UIToolkitDemo;

mod fonts; // fonts file
mod toolkit; // the actual app code

pub(crate) fn main() {
    let native_options: NativeOptions = eframe::NativeOptions::default();

    eframe::run_native (
        "AvdanOS UI Toolkit Demo",
        native_options,
        Box::new(|cc: &CreationContext| Box::new(UIToolkitDemo::new(cc)))
    );
}
