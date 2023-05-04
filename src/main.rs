#![allow(unused_imports)] // temporary

use iced::widget::{
    column,
    container,
    text
};

use iced::{
    Alignment,
    Element,
    Length,
    Sandbox,
    Settings
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

struct Toolkit {}

// interactions
#[derive(Debug, Clone, Copy)]
enum Message {}

impl Sandbox for Toolkit {
    type Message = Message;

    // store interactions
    fn new() -> Self {
        Self {}
    }

    // window title
    fn title(&self) -> String {
        String::from("AvdanOS UI Toolkit Demo")
    }

    // update state
    fn update(&mut self, message: Message) {
        match message {}
    }

    // show things on screen
    fn view(&self) -> Element<Message> {
        let title = text("AvdanOS UI Toolkit Demo")
            .font(REGULAR);

        let content = column![title]
            .padding(20)
            .align_items(Alignment::Start);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .into()
    }
}
