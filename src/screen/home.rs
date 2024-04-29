use iced::widget::{button, column, container, text};
use iced::{Command, Element, Renderer, Theme};

use super::{Action, Screen};

#[derive(Debug, Clone)]
pub enum Message {
    Action(Action),

    Increase,
    Decrease,
}

#[derive(Debug, Clone, Default)]
pub struct HomeScreen {
    pub count: i32,
}

pub fn home_screen() -> HomeScreen {
    HomeScreen::new()
}

impl HomeScreen {
    pub fn new() -> Self {
        Self { count: 0 }
    }

    pub fn update(&mut self, message: Message) -> Option<Command<Message>> {
        match message {
            Message::Action(_) => {}
            Message::Increase => {
                self.count += 1;
            }
            Message::Decrease => {
                self.count -= 1;
            }
        };

        None
    }

    pub fn view(&self) -> Element<'_, Message, Theme, Renderer> {
        container(column!(
            text("Home Screen!"),
            text(self.count.to_string()),
            button(text("Increase")).on_press(Message::Increase),
            button(text("Decrease")).on_press(Message::Decrease),
            button(text("Set theme: Dracula"))
                .on_press(Message::Action(Action::ChangeTheme(Theme::Dracula))),
            button(text("Set theme: Catppuccin Mocha"))
                .on_press(Message::Action(Action::ChangeTheme(Theme::CatppuccinMocha))),
            button(text("Quick Jump to Settings Screen"))
                .on_press(Message::Action(Action::SwitchScreen(Screen::Settings))),
        ))
        .into()
    }
}
