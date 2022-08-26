use eframe::egui::{self, Response, Widget};
use eframe::egui::special_emojis::GITHUB;

use crate::egui::Ui;
use crate::fonts::setup_font;

pub struct UIToolkitDemo{
    boolean: bool, // for checklists (true and false)
    radio: Enum, /* radio button options (Enum). similarly to scalar it also syncs the values for the 
    RadioButton, SelectableLabel, and the ComboBox.*/
    scalar: f32, /* fraction from the whole in the ProgressBar and Slider (out of 100%, 360°). also it allows the DragValue, Slider, and 
    ProgressBar values to be synced.*/
    color: egui::Color32, // current color for the ColorPicker
    animate_progress_bar: bool, 
    text_input:String, // current text input from the user in the TextInput field
}

impl UIToolkitDemo {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_font(&cc.egui_ctx);
        Self::default()
    }
}

#[derive(Debug, PartialEq)]
enum Enum {
    First,
    Second,
    Third,
}
impl Default for Enum { // default implementation for the Enum (instead of using the derive macro)
    fn default() -> Self {
        Self::First
    }
}

impl Default for UIToolkitDemo { // default implementation for the UIToolkitDemo struct
    fn default() -> Self {
        Self {
            boolean: false,
            radio: Enum::First,
            scalar: 42.0,
            color: egui::Color32::LIGHT_BLUE.linear_multiply(0.5),
            animate_progress_bar: true,
            text_input: "".to_string(),
        }
    }
}

fn doc_link_label<'a>(title: &'a str, search_term: &'a str) -> impl egui::Widget + 'a {
    // hyperlink label helper function (creates hoverable hyperlinks for labels)
    let label = format!("{}:", title);
    let url = format!("https://docs.rs/egui?search={}", search_term);
    move |ui: &mut egui::Ui| {
        ui.hyperlink_to(label, url).on_hover_ui(|ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label("Search egui docs for");
                ui.code(search_term);
            });
        })
    }
}
impl eframe::App for UIToolkitDemo { // the actual UI 
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) { // updates every frame 
        egui::CentralPanel::default().show(ctx, |ui: &mut Ui| {
            ui.heading("AvdanOS UI Toolkit Demo");
            ui.end_row();
            
            ui.hyperlink_to(
                format!("{} Check us out on GitHub !", GITHUB),
                "https://github.com/Avdan-OS",
            );
            
            ui.vertical_centered(|ui| {
                let tooltip_text = "The full egui documentation.\nYou can also click the different widgets names in the left column.";
                ui.hyperlink("https://docs.rs/egui/").on_hover_text(tooltip_text);
            });
            
            ui.separator();
            ui.end_row();

            // light mode and dark mode buttons
            ui.horizontal(|ui| { 
                ui.label("Dark mode or Light mode ?!");

                if ui.add(egui::Button::new("Dark mode!")).clicked() {
                    ctx.set_visuals(egui::Visuals::dark());
                }
                if ui.add(egui::Button::new("Light mode!")).clicked() {
                    ctx.set_visuals(egui::Visuals::light());
                }
            });

            // Text input box
            ui.add(egui::TextEdit::singleline(&mut self.text_input).hint_text("Write something here"));
            ui.end_row();
            
            // Slider 
            ui.add(doc_link_label("Slider", "Slider"));
            ui.add(egui::Slider::new(&mut self.scalar, 0.0..=360.0).suffix("°"));
            ui.end_row();
            
            // Drag Value
            ui.add(doc_link_label("DragValue", "DragValue"));
            ui.add(egui::DragValue::new(&mut self.scalar).speed(1.0));
            ui.end_row();
            
            // Progress Bar
            ui.add(doc_link_label("ProgressBar", "ProgressBar"));
            let progress = self.scalar / 360.0; // the progress here is a literal fraction of the whole (scalar divided by total)
            let progress_bar = egui::ProgressBar::new(progress) 
                .show_percentage()
                .animate(self.animate_progress_bar);
            
            self.animate_progress_bar = ui // this is the actual ProgressBar UI element
                .add(progress_bar)
                .on_hover_text("The progress bar can be animated!")
                .hovered();
            ui.end_row();

            // Color Picker
            ui.add(doc_link_label("Color picker", "color_edit"));
            ui.color_edit_button_srgba(&mut self.color);
            ui.end_row();

            // Checkbox
            ui.add(doc_link_label("Checkbox", "checkbox"));
            ui.checkbox(&mut self.boolean, "Checkbox");
            ui.end_row();

            // Radio Button 
            ui.add(doc_link_label("RadioButton", "radio"));
            ui.horizontal(|ui| {
                ui.radio_value(&mut self.radio, Enum::First, "First");
                ui.radio_value(&mut self.radio, Enum::Second, "Second");
                ui.radio_value(&mut self.radio, Enum::Third, "Third");
            });
            ui.end_row();

            // Selectable Label
            ui.add(doc_link_label( // the hyperlink components for the SelectableLabel
                "SelectableLabel",
                "selectable_value, SelectableLabel",
            ));
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.radio, Enum::First, "First");
                ui.selectable_value(&mut self.radio, Enum::Second, "Second");
                ui.selectable_value(&mut self.radio, Enum::Third, "Third");
            });
            ui.end_row();
            
            // ComboBox
            ui.add(doc_link_label("ComboBox", "ComboBox"));
            egui::ComboBox::from_label("Take your pick")
                .selected_text(format!("{:?}", &mut self.radio))
                .show_ui(ui, |ui| { // the actual ComboBox with the 3 selectable values
                    ui.selectable_value(&mut self.radio, Enum::First, "First");
                    ui.selectable_value(&mut self.radio, Enum::Second, "Second");
                    ui.selectable_value(&mut self.radio, Enum::Third, "Third");
                });
            ui.end_row();

            // Collapsing Header + Spinner 
            ui.add(doc_link_label("CollapsingHeader", "collapsing"));
            ui.collapsing("Click to see what is hidden!", |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0; // disables item spacing that is enabled by default
                    ui.label("It's a ");
                    ui.add(doc_link_label("Spinner ! ", "spinner"));
                    ui.add_space(4.0);
                    ui.add(egui::Spinner::new());
                });
            });
            ui.end_row();
            ui.separator();
        });
    }
}