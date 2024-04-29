use iced::theme::Container;
use iced::widget::container;
use iced::widget::container::Appearance;
use iced::{Background, Color};

pub struct StyleSheet;

impl StyleSheet {
    pub fn new() -> Container {
        Container::Custom(Box::new(StyleSheet))
    }
}

impl container::StyleSheet for StyleSheet {
    type Style = iced::Theme;

    fn appearance(&self, style: &Self::Style) -> Appearance {
        let palette = style.palette();
        let fg = palette.text;
        let bg = Color::from_rgb(
            palette.background.r * 0.9,
            palette.background.g * 0.9,
            palette.background.b * 0.9,
        );
        Appearance {
            text_color: Some(fg),
            background: Some(Background::Color(bg)),
            ..Default::default()
        }
    }
}
