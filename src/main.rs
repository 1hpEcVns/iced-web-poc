use iced::widget::{button, column, text};
use iced::Element;

#[derive(Debug, Clone)]
enum Message {
    Increment,
    Decrement,
}

fn update(value: &mut i32, message: Message) {
    match message {
        Message::Increment => *value += 1,
        Message::Decrement => *value -= 1,
    }
}

fn view(value: &i32) -> Element<Message> {
    column![
        text("Counter").size(32),
        text(*value).size(48),
        button("+").on_press(Message::Increment),
        button("-").on_press(Message::Decrement),
    ]
    .padding(20)
    .into()
}

fn main() -> iced::Result {
    iced::application(i32::default, update, view).run()
}
