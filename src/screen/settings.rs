use iced::widget::{column, container, text, toggler};
use iced::{Command, Element, Renderer, Theme};

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Toggled(bool),
}

#[derive(Debug, Clone, Default)]
pub struct SettingsScreen {
    pub toggle: bool,
}

pub fn settings_screen() -> SettingsScreen {
    SettingsScreen::new()
}

impl SettingsScreen {
    pub fn new() -> Self {
        Self { toggle: false }
    }

    pub fn update(&mut self, message: Message) -> Option<Command<Message>> {
        match message {
            Message::Toggled(value) => self.toggle = value,
        }

        None
    }

    pub fn view(&self) -> Element<'_, Message, Theme, Renderer> {
        container(column!(
            text("Settings Screen!"),
            toggler("Toggle me!".to_string(), self.toggle, Message::Toggled),
        ))
        .into()
    }
}
