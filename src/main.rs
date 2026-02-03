use iced::widget::{Column, column, text, text_input};

use crate::{plugins::text_search::TextSearch, queriable::QueryPlugin};

mod plugins;
mod queriable;

pub fn main() -> iced::Result {
    iced::run(State::update, State::view)
}

struct State {
    plugins: Vec<Box<dyn QueryPlugin>>,
    query: String,
}

impl Default for State {
    fn default() -> Self {
        Self {
            plugins: vec![Box::new(TextSearch)],
            query: String::new(),
        }
    }
}

impl State {
    fn update(&mut self, message: Message) {
        match message {
            Message::ContendChanged(content) => self.query = content,
        }
    }

    fn view<'a>(&'a self) -> Column<'a, Message> {
        let mut column =
            column![text_input("search...", &self.query).on_input(Message::ContendChanged)];

        for plugin in &self.plugins {
            for result in plugin.search(&self.query) {
                column = column.push(text(result.text));
            }
        }

        column
    }
}

#[derive(Debug, Clone)]
enum Message {
    ContendChanged(String),
}
