use chrono;
use iced::widget::{button, column, Column, container, pick_list, row, Row, text, scrollable};
use iced::{Command, Element, Renderer, Theme};
use iced::theme::Container;
use crate::api::clients::valorant_api_local::AsyncValorantApiLocal;
use crate::api::endpoints::local::friends::Friends;
use crate::api::query::AsyncQuery;
use crate::api::types::local::friends;
use crate::api::types::local::friends::Friend;

use super::{Action, Screen};

#[derive(Debug, Clone)]
pub enum FriendsError {
    FailedToFetchFriends,
}

#[derive(Debug, Clone)]
pub enum Message {
    Action(Action),

    Increase,
    Decrease,

    FriendsUpdated(Result<Vec<Friend>, FriendsError>)
}

#[derive(Debug, Clone, Default)]
pub struct HomeScreen {
    pub count: i32,
    pub theme: Theme,
    pub api: AsyncValorantApiLocal,
    pub friends: Option<Vec<Friend>>,
}

impl HomeScreen {
    pub fn new(theme: Theme, api: AsyncValorantApiLocal) -> (Self, Command<Message>) {
        (
            Self {
                theme,
                api: api.clone(),
                friends: None,
                ..Default::default()
            },
            Command::perform(async move { Friends::new().query_async(&api).await }, |result| {
                println!("{:?}", result);
                match result {
                    Ok(friends) => Message::FriendsUpdated(Ok(friends.friends)),
                    Err(_) => Message::FriendsUpdated(Err(FriendsError::FailedToFetchFriends)),
                }
            })
        )
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
            Message::FriendsUpdated(friends) => {
                self.friends = friends.ok();
            }
        };

        None
    }

    pub fn view(&self) -> Element<'_, Message, Theme, Renderer> {
        let mut friends_column = Column::new();
        if let Some(friends) = &self.friends {
            for friend in friends {
                // From unix timestamp to human readable date
                // If Null it means the friend is online
                let last_online = match friend.last_online_ts {
                    Some(ts) => "Online".to_string(),
                    None => "Offline".to_string(),
                };

                friends_column = friends_column.push(
                    row!(
                        text(friend.game_name.clone())
                        .width(iced::Length::Fill),
                        text(friend.game_tag.clone())
                        .width(iced::Length::Fill),
                        text(last_online)
                        .width(iced::Length::Fill),
                    )
                );
            }
        }

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

                column!(
                    row!(
                        text("Game Name")
                        .width(iced::Length::Fill),
                        text("Game Tag")
                        .width(iced::Length::Fill),
                        text("Last Online")
                        .width(iced::Length::Fill),
                    ),
                    scrollable(
                        friends_column
                    )
                )
                .width(iced::Length::Fill)
                .height(iced::Length::Fill)
            )
            .spacing(10),
        )
        .into()
    }
}
