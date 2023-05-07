#![allow(unused_imports)] // temporary

use iced::widget::{
    column,
    container,
    radio,
    row,
    text,
};

use iced::{
    Alignment,
    Element,
    Length,
    Sandbox,
    Settings
};

use iced::theme::{
    self,
    Theme
};

mod fonts;

use crate::fonts::{
    BLACK,
    BOLD,
    EXTRA_BOLD,
    EXTRA_LIGHT,
    LIGHT,
    MEDIUM,
    REGULAR,
    SEMI_BOLD,
    THIN
};

pub fn main() -> iced::Result {
    Toolkit::run(Settings::default()) // run program
}

struct Toolkit {
    theme: Theme
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum UITheme {
    Light,
    Dark
}

// interactions
#[derive(Debug, Clone, Copy)]
enum Message {
    ThemeChanged(UITheme)
}

impl Sandbox for Toolkit {
    type Message = Message;

    // store interactions
    fn new() -> Self {
        Toolkit { theme: Theme::Light }
    }

    // window title
    fn title(&self) -> String {
        String::from("AvdanOS UI Toolkit Demo")
    }

    // update state
    fn update(&mut self, message: Message) {
        match message {
            Message::ThemeChanged(theme) => {
                self.theme = match theme {
                    UITheme::Light => Theme::Light,
                    UITheme::Dark  => Theme::Dark
                }
            }
        }
    }

    // show things on screen
    fn view(&self) -> Element<Message> {
        let title = text("AvdanOS UI Toolkit Demo")
            .font(REGULAR);

        let choose_theme =
            [UITheme::Light, UITheme::Dark]
                .iter()
                .fold (
                    row![text("Choose a theme:")].spacing(10),
                    |row, theme| {
                        row.push(radio (
                            format!("{theme:?}"),
                            *theme,
                            Some(match self.theme {
                                Theme::Light => UITheme::Light,
                                Theme::Dark  => UITheme::Dark,
                                Theme::Custom(..) => todo!()
                            }),
                            Message::ThemeChanged)
                        )
                    });

        let content = column![
            title,
            choose_theme
        ]
            .spacing(20)
            .padding(20)
            .align_items(Alignment::Start);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}
