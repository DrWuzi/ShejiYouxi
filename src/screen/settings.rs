use iced::widget::{column, container, text, toggler};
use iced::{Command, Element, Renderer, Subscription, Theme};

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Toggled(bool),
}

#[derive(Debug, Clone, Default)]
pub struct SettingsScreen {
    pub toggle: bool,
}

impl SettingsScreen {
    pub fn new() -> (Self, Command<Message>) {
        (
            Self::default(),
            Command::none()
        )
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

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }
}
