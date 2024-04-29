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
        let palette = style.extended_palette();
        let fg = palette.primary.base.text;
        let modifier = if palette.is_dark { 0.8 } else { 0.9 };
        let bg = Color::from_rgb(
            palette.background.base.color.r * modifier,
            palette.background.base.color.g * modifier,
            palette.background.base.color.b * modifier,
        );
        Appearance {
            text_color: Some(fg),
            background: Some(Background::Color(bg)),
            ..Default::default()
        }
    }
}
