use iced::widget::{Column, column, text, text_input};

const TERMS: [&'static str; 11] = [
    "cheese", "potato", "carrot", "house", "bear", "clobber", "turkey", "mouse", "crab", "curtain",
    "zeus",
];

mod queriable;

pub fn main() -> iced::Result {
    iced::run(Counter::update, Counter::view)
}

#[derive(Default)]
struct Counter {
    value: i64,
    query: String,
}

impl Counter {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => self.value += 1,
            Message::Decrement => self.value -= 1,
            Message::ContendChanged(content) => self.query = content,
        }
    }

    fn view(&self) -> Column<Message> {
        let mut column =
            column![text_input("search...", &self.query).on_input(Message::ContendChanged)];

        for value in TERMS {
            if value.starts_with(&self.query) {
                column = column.push(text(value));
            }
        }

        column
    }
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
    Decrement,
    ContendChanged(String),
}
