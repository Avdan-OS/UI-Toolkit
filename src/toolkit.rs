use eframe::egui::{self, Response, Widget};
use eframe::egui::special_emojis::GITHUB;
use eframe::egui::style::WidgetVisuals;

use crate::egui::Ui;
use crate::fonts::setup_font;

#[derive(Default)]

pub struct UIToolkitDemo{
    boolean: bool,
    radio: Enum,
    scalar: f32,
    color: egui::Color32,
    animate_progress_bar: bool,
    text_input:String, 
}


impl UIToolkitDemo {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_font(&cc.egui_ctx);
        Self::default()
    }
}


#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
enum Enum {
    First,
    Second,
    Third,
}

impl Default for Enum {
    fn default() -> Self {
        Self::First
    }
}
pub struct WidgetGallery {
    enabled: bool,
    visible: bool,
    boolean: bool,
    radio: Enum,
    scalar: f32,
    string: String,
    color: egui::Color32,
    animate_progress_bar: bool,
}

impl Default for WidgetGallery {
    fn default() -> Self {
        Self {
            enabled: true,
            visible: true,
            boolean: false,
            radio: Enum::First,
            scalar: 42.0,
            string: Default::default(),
            color: egui::Color32::LIGHT_BLUE.linear_multiply(0.5),
            animate_progress_bar: true,
        }
    }
}

fn doc_link_label<'a>(title: &'a str, search_term: &'a str) -> impl egui::Widget + 'a {
    // hyperlink label helper function
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
impl eframe::App for UIToolkitDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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
            }); // buttons

            ui.horizontal(|ui: &mut Ui| { // sliders 
                ui.label("Sliders :");
                ui.label("AvdanOS Coolness : ");
                ui.add(egui::Slider::new(&mut 0, 0..=1000),);
                ui.label("Rust Coolness");
                ui.add(egui::Slider::new(&mut 0, 0..=1000));
            }); // sliders 

            ui.horizontal(|ui: &mut Ui| { // checkboxes
                ui.label("Checkboxes :");
                ui.checkbox(&mut true, "AvdanOS is cool !");
                ui.checkbox(&mut true, "We need more developers !");
            }); // sliders

            ui.horizontal(|ui: &mut Ui| { // drag values
                ui.label("Dragvalue :");
                ui.add(egui::DragValue::new(&mut 100));
                ui.add(egui::DragValue::new(&mut 100));
            }); // drag values

            ui.separator();

            ui.collapsing("Click to see what is hidden!", |ui| {
                ui.label("Not much, as it turns out");
            });
            ui.end_row();

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
            let progress = self.scalar / 360.0;
            let progress_bar = egui::ProgressBar::new(progress)
                .show_percentage()
                .animate(self.animate_progress_bar);
            
            self.animate_progress_bar = ui
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
            ui.add(doc_link_label(
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
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.radio, Enum::First, "First");
                    ui.selectable_value(&mut self.radio, Enum::Second, "Second");
                    ui.selectable_value(&mut self.radio, Enum::Third, "Third");
                });
            ui.end_row();

            // Collapsing Header + Spinner 
            ui.add(doc_link_label("CollapsingHeader", "collapsing"));
            ui.collapsing("Click to see what is hidden!", |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
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