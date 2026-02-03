use iced::{
    application::IntoBoot,
    widget::{Column, column, text, text_input},
};

use crate::{
    plugins::text_search::TextSearch,
    queriable::{QueryPlugin, QueryResult},
};

mod plugins;
mod queriable;

pub fn main() -> iced::Result {
    iced::application(|| State::new(), State::update, State::view).run()
}

struct State {
    plugins: Vec<Box<dyn QueryPlugin>>,
    query: String,
}

impl State {
    fn new() -> Self {
        Self {
            plugins: vec![Box::new(TextSearch)],
            query: String::new(),
        }
    }

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
