use iced::widget::{container, text};
use iced::{Command, Element, Renderer, Theme};

#[derive(Debug, Clone, Copy)]
pub enum Message {}

#[derive(Debug, Clone, Default)]
pub struct SettingsScreen {}

pub fn settings_screen() -> SettingsScreen {
    SettingsScreen::new()
}

impl SettingsScreen {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, _message: Message) -> Option<Command<Message>> {
        None
    }

    pub fn view(&self) -> Element<'_, Message, Theme, Renderer> {
        container(text("Settings Screen!")).into()
    }
}
