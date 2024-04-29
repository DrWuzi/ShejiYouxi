use chrono;
use iced::widget::{button, column, Column, container, pick_list, row, Row, text, scrollable};
use iced::{Command, Element, Renderer, Theme};
use iced::theme::Container;
use crate::api::clients::valorant_api_local::AsyncValorantApiLocal;
use crate::api::endpoints::local::friends::Friends;
use crate::api::endpoints::local::presence::Presence;
use crate::api::query::AsyncQuery;
use crate::api::types::local::friends;
use crate::api::types::local::presence::{PresenceDetails, PresenceState};

use super::{Action, Screen};

#[derive(Debug, Clone)]
pub enum ApiError {
    FailedToFetchPresences,
    FailedToFetchFriends,
}

#[derive(Debug, Clone)]
pub enum Message {
    Action(Action),

    Increase,
    Decrease,

    PresencesUpdated(Result<Vec<PresenceDetails>, ApiError>),
    FriendsUpdated(Result<Vec<friends::Friend>, ApiError>),
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
        let presence_api = api.clone();

        (
            Self {
                theme,
                api: api.clone(),
                friends: None,
                ..Default::default()
            },
            Command::batch( vec![
                Command::perform(async move { Presence::new().query_async(&presence_api).await }, |response| {
                    match response {
                        Ok(presences) => Message::PresencesUpdated(Ok(presences.presences)),
                        Err(_) => Message::PresencesUpdated(Err(ApiError::FailedToFetchPresences)),
                    }
                }),
                Command::perform(async move { Friends::new().query_async(&api).await }, |response| {
                    match response {
                        Ok(friends) => Message::FriendsUpdated(Ok(friends.friends)),
                        Err(_) => Message::FriendsUpdated(Err(ApiError::FailedToFetchFriends)),
                    }
                }),
            ])
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
            Message::PresencesUpdated(presences) => {
                // - Push if friend not already in list.
                // - Update to Online if friend is already in list and in presences.
                // - Update to Offline if friend is already in list and not in presences.
                if let Ok(presences) = presences {
                    let mut friends: Vec<Friend> = self.friends.take().unwrap_or_default();

                    for friend in friends.iter_mut() {
                        friend.state = FriendState::Offline;
                    }

                    for presence in presences {
                        let friend = Friend::from(presence.clone());
                        if let Some(index) = friends.iter().position(|f| f.puuid == friend.puuid) {
                            friends[index] = friend;
                        } else {
                            friends.push(friend);
                        }
                    }

                    self.friends = Some(friends);
                }
            }
            Message::FriendsUpdated(friends) => {
                // - Push if friend not already in list.
                if let Ok(friends) = friends {
                    let mut self_friends = self.friends.take().unwrap_or_default();
                    for friend in friends {
                        if !self_friends.iter().any(|f| f.puuid == friend.puuid) {
                            self_friends.push(Friend::from(friend));
                        }
                    }
                    self.friends = Some(self_friends);
                }
            }
        };

        None
    }

    pub fn view(&self) -> Element<'_, Message, Theme, Renderer> {
        let mut friends_column = Column::new();
        if let Some(friends) = &self.friends {
            for friend in friends {
                let state = match &friend.state {
                    FriendState::Offline => "Offline",
                    FriendState::Online(state) => match state {
                        PresenceState::Mobile => "Mobile",
                        PresenceState::Dnd => "Do Not Disturb",
                        PresenceState::Away => "Away",
                        PresenceState::Chat => "Online",
                    }
                }.to_string();

                friends_column = friends_column.push(
                    row!(
                        text(friend.game_name.clone())
                        .width(iced::Length::Fill),
                        text(friend.game_tag.clone())
                        .width(iced::Length::Fill),
                        text(state)
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
                        text("State")
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

#[derive(Debug, Clone)]
pub enum FriendState {
    Offline,
    Online(PresenceState),
}

#[derive(Debug, Clone)]
pub struct Friend {
    pub puuid: String,
    pub game_name: String,
    pub game_tag: String,
    pub state: FriendState,
}

impl From<friends::Friend> for Friend {
    fn from(friend: friends::Friend) -> Self {
        Self {
            puuid: friend.puuid,
            game_name: friend.game_name,
            game_tag: friend.game_tag,
            state: FriendState::Offline,
        }
    }
}

impl From<PresenceDetails> for Friend {
    fn from(presence: PresenceDetails) -> Self {
        Self {
            puuid: presence.puuid,
            game_name: presence.game_name,
            game_tag: presence.game_tag,
            state: FriendState::Online(presence.state),
        }
    }
}