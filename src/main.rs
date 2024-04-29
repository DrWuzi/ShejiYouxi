#![allow(dead_code, unused)]

mod api;
mod error;
mod screen;
mod stylesheet;

use api::clients::valorant_api_local::AsyncValorantApiLocal;
use api::valorant_lockfile::Lockfile;
use error::Result;
use iced::widget::{button, column, container, row, Column, Text};
use iced::{executor, Length, Padding};
use iced::{window, Alignment, Application, Command, Element, Settings, Size, Theme};
use screen::home::{self, home_screen, HomeScreen};
use screen::settings::{self, settings_screen, SettingsScreen};
use screen::{Action, Screen};
use stylesheet::sidebar;

pub fn main() -> Result<()> {
    let settings: Settings<()> = Settings {
        window: window::Settings {
            size: Size::new(600.0, 300.0),
            ..window::Settings::default()
        },
        ..Default::default()
    };

    Ok(App::run(settings)?)
}

struct App {
    theme: Theme,
    api: AsyncValorantApiLocal,

    screen: Screen,
    home_screen: HomeScreen,
    settings_screen: SettingsScreen,
}

#[derive(Debug, Clone)]
enum Message {
    Action(Action),

    HomeScreen(home::Message),
    SettingsScreen(settings::Message),
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let client = reqwest::Client::new();
        #[cfg(target_os = "windows")]
        let lockfile = Lockfile::new_from_lockfile().expect("Failed to read lockfile."); // TODO: Handle error correctly
        #[cfg(not(target_os = "windows"))]
        // Just a testing lockfile so that it doesn't crash during development
        let lockfile = Lockfile::new(
            "test".to_string(),
            100,
            16000,
            "password".into(),
            "https".to_string(),
        );
        let api = AsyncValorantApiLocal::new(client, lockfile);

        (
            Self {
                theme: Theme::Dracula,
                api,

                screen: Screen::Home,
                home_screen: home_screen(),
                settings_screen: settings_screen(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Sheji Youxi")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Action(action) => {
                match action {
                    Action::SwitchScreen(screen) => self.screen = screen,
                    Action::ChangeTheme(theme) => self.theme = theme,
                };

                Command::none()
            }
            Message::HomeScreen(message) => {
                if let Screen::Home = &mut self.screen {
                    if let home::Message::Action(action) = message.clone() {
                        match action {
                            Action::SwitchScreen(screen) => self.screen = screen,
                            Action::ChangeTheme(theme) => self.theme = theme,
                        }
                    }

                    self.home_screen.update(message);
                }

                Command::none()
            }
            Message::SettingsScreen(message) => {
                if let Screen::Settings = &mut self.screen {
                    self.settings_screen.update(message);
                }

                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let screen = match &self.screen {
            Screen::Home => self.home_screen.view().map(Message::HomeScreen),
            Screen::Settings => self.settings_screen.view().map(Message::SettingsScreen),
        };
        row!(
            container(
                column!(
                    menu_entry("A", Screen::Home),
                    menu_entry("B", Screen::Settings),
                )
                .height(Length::Fill)
                .width(Length::Fill)
                .align_items(Alignment::Center)
                .spacing(10)
            )
            .style(sidebar::container::StyleSheet::new())
            .padding(Padding::from(10))
            .height(Length::Fill)
            .width(60),
            container(screen).height(Length::Fill).width(Length::Fill)
        )
        .height(Length::Fill)
        .width(Length::Fill)
        .into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}

fn menu_entry(
    content: impl Into<Element<'static, Message>>,
    screen: Screen,
) -> Element<'static, Message> {
    button(content)
        .height(40)
        .width(40)
        .on_press(Message::Action(Action::SwitchScreen(screen)))
        .into()
}
