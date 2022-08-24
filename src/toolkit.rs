use eframe::egui;
use eframe::egui::special_emojis::GITHUB;
use eframe::egui::style::WidgetVisuals;

use crate::egui::Ui;
use crate::fonts::setup_font;

#[derive(Default)]
pub struct UIToolkitDemo{
    scalar: f32,
    animate_progress_bar: bool,
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

/// Shows off one example of each major type of widget.
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct WidgetGallery {
    enabled: bool,
    visible: bool,
    boolean: bool,
    radio: Enum,
    scalar: f32,
    string: String,
    color: egui::Color32,
    animate_progress_bar: bool,

    #[cfg(feature = "chrono")]
    #[cfg_attr(feature = "serde", serde(skip))]
    date: Option<chrono::Date<chrono::Utc>>,

    #[cfg_attr(feature = "serde", serde(skip))]
    texture: Option<egui::TextureHandle>,
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
            texture: None,
        }
    }
}

fn doc_link_label<'a>(title: &'a str, search_term: &'a str) -> impl egui::Widget + 'a { // 
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

            ui.separator();

            ui.vertical_centered(|ui| {
                let tooltip_text = "The full egui documentation.\nYou can also click the different widgets names in the left column.";
                ui.hyperlink("https://docs.rs/egui/").on_hover_text(tooltip_text);
            });

            ui.end_row();
            let mut string = "";
            ui.add(egui::TextEdit::singleline(&mut string).hint_text("Write something here"));

            ui.end_row();
            
            ui.separator();

            ui.horizontal(|ui| { // light mode and dark mode buttons
                ui.label("Dark mode or Light mode ?!");
                if ui.add(egui::Button::new("Dark mode!")).clicked() {
                    ctx.set_visuals(egui::Visuals::dark());
                }
                if ui.add(egui::Button::new("Light mode!")).clicked() {
                    ctx.set_visuals(egui::Visuals::light());
                }
            }); // light mode and dark mode buttons

            ui.add(doc_link_label(
                "SelectableLabel",
                "selectable_value, SelectableLabel",
            ));
    
            ui.add(doc_link_label("ComboBox", "ComboBox"));

            #[cfg(feature = "chrono")]
            {
                let date = date.get_or_insert_with(|| chrono::offset::Utc::now().date());
                ui.add(doc_link_label::<'a>("DatePickerButton", "DatePickerButton"));
                ui.add(egui_extras::DatePickerButton::new(date));
                ui.end_row();
            }

            ui.add(doc_link_label("Separator", "separator"));
            ui.separator();
            ui.end_row();

            ui.add(doc_link_label("CollapsingHeader", "collapsing"));
            ui.collapsing("Click to see what is hidden!", |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("It's a ");
                    ui.add(doc_link_label("Spinner", "spinner"));
                    ui.add_space(4.0);
                    ui.add(egui::Spinner::new());
                });
            });
            ui.end_row();

            ui.separator();

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
            }); // central panel
    }
}