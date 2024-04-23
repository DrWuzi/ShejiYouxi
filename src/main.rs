mod error;
mod layout;
mod api;

use api::clients::valorant_api_local::AsyncValorantApiLocal;
use api::valorant_lockfile::Lockfile;
use error::Result;
use iced::executor;
use iced::widget::{Column, Text};
use iced::{window, Alignment, Application, Command, Element, Settings, Size, Theme};
use crate::api::endpoints::local::friends::Friends;
use crate::api::query::AsyncQuery;
use crate::api::types::local::friends::Friend;
use crate::layout::Layout;

#[tokio::main]
pub async fn main() -> Result<()> {
    if !cfg!(target_os = "windows") {
        panic!("This app can only be run on Windows.");
    }

    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .use_rustls_tls()
        .build().expect("failed to build client");
    let lockfile = Lockfile::new_from_lockfile().expect("Failed to read lockfile."); // TODO: Handle error correctly
    let api = AsyncValorantApiLocal::new(client, lockfile);

    let friends = Friends::new().query_async(&api).await.expect("failed to query friends");
    println!("{:?}", friends);

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
    is_loading: bool,
    layout: Layout,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                theme: Theme::Dracula,
                is_loading: false,
                layout: Layout::new(600, 300, 20, 20, 20),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Sheji Youxi")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        if self.is_loading {
            return Command::none();
        }
        self.layout.update(message);

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        if self.is_loading {
            Column::new()
                .push(Text::new("Loading..."))
                .align_items(Alignment::Center)
                .into()
        } else {
            self.layout.view()
        }
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}
