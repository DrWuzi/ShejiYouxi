use iced::{Alignment, Element};
use iced::widget::{button, Column, Row, Text};
use crate::Message;

pub struct Layout {
    pub width: u32,
    pub height: u32,
    pub margin: u32,
    pub padding: u32,
    pub border: u32,
    pub counter: i32,
}

impl Layout {
    pub fn new(width: u32, height: u32, margin: u32, padding: u32, border: u32) -> Self {
        Self {
            width,
            height,
            margin,
            padding,
            border,
            counter: 0,
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.counter += 1;
            }
            Message::DecrementPressed => {
                self.counter -= 1;
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        Row::new()
            .push(
                // Sidebar
                Column::new()
                    .push(button("Increment").on_press(Message::IncrementPressed))
                    .push(Text::new(self.counter.to_string()))
                    .push(button("Decrement").on_press(Message::DecrementPressed)),
            )
            .push(
                // Content bar
                Column::new().push(Text::new("Content goes here")),
            )
            .padding(20)
            .align_items(Alignment::Center)
            .into()
        }
}
