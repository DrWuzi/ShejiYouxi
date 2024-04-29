use iced::widget::{button, column, container, pick_list, row, text};
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
    pub theme: Theme,
}

pub fn home_screen(theme: Theme) -> HomeScreen {
    HomeScreen::new(theme)
}

impl HomeScreen {
    pub fn new(theme: Theme) -> Self {
        Self { count: 0, theme }
    }

    pub fn update(&mut self, message: Message) -> Option<Command<Message>> {
        match message {
            Message::Action(action) => match action {
                Action::ChangeTheme(theme) => self.theme = theme,
                _ => {}
            },
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
        container(
            column!(
                text("Home Screen!"),
                row!(
                    button(text("Increase")).on_press(Message::Increase),
                    text(self.count.to_string()),
                    button(text("Decrease")).on_press(Message::Decrease),
                )
                .align_items(iced::Alignment::Center)
                .spacing(10),
                button(text("Set theme: Dracula"))
                    .on_press(Message::Action(Action::ChangeTheme(Theme::Dracula))),
                button(text("Set theme: Catppuccin Mocha"))
                    .on_press(Message::Action(Action::ChangeTheme(Theme::CatppuccinMocha))),
                button(text("Quick Jump to Settings Screen"))
                    .on_press(Message::Action(Action::SwitchScreen(Screen::Settings))),
                pick_list(Theme::ALL, Some(&self.theme), |theme| {
                    Message::Action(Action::ChangeTheme(theme))
                }),
            )
            .spacing(10),
        )
        .into()
    }
}
