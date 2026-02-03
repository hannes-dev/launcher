use iced::widget::{Column, column, text, text_input};

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
    results: Vec<QueryResult>,
}

impl State {
    fn new() -> Self {
        let mut state = Self {
            plugins: vec![Box::new(TextSearch)],
            query: String::new(),
            results: Vec::new(),
        };
        state.update_results();
        state
    }

    fn update_results(&mut self) {
        self.results = self
            .plugins
            .iter()
            .flat_map(|p| p.search(&self.query))
            .collect();
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ContendChanged(content) => {
                self.query = content;
                self.update_results();
            }
        }
    }

    fn view<'a>(&'a self) -> Column<'a, Message> {
        let mut column =
            column![text_input("search...", &self.query).on_input(Message::ContendChanged)];

        for result in &self.results {
            let text = text(&result.text);
            column = column.push(text)
        }

        column
    }
}

#[derive(Debug, Clone)]
enum Message {
    ContendChanged(String),
}
