pub mod home;
pub mod settings;

use iced::Theme;

#[derive(Debug, Clone, Copy)]
pub enum Screen {
    Home,
    Settings,
}

#[derive(Debug, Clone)]
pub enum Action {
    SwitchScreen(Screen),
    ChangeTheme(Theme),
}
