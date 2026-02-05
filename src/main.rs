use std::process;

use iced::{
    Color, Task,
    keyboard::{self, key::Named},
    widget::{Column, column, text, text_input},
};

use iced_layershell::{
    Settings, application,
    reexport::{Anchor, Layer},
    settings::{LayerShellSettings, StartMode},
    to_layer_message,
};

use crate::{
    plugins::{calculator::Calculator, desktop::Desktop, text_search::TextSearch},
    queriable::{QueryPlugin, QueryResult},
};

mod plugins;
mod queriable;

pub fn main() {
    application(
        || State::new(),
        || String::from("cheese"),
        State::update,
        State::view,
    )
    .subscription(State::subscription)
    .settings(Settings {
        layer_settings: LayerShellSettings {
            size: Some((600, 400)),
            exclusive_zone: -1,
            anchor: Anchor::empty(),
            start_mode: StartMode::Active,
            layer: Layer::Overlay,
            ..Default::default()
        },
        ..Default::default()
    })
    .run()
    .unwrap();
}

#[derive(Default)]
struct State {
    plugins: Vec<Box<dyn QueryPlugin>>,
    query: String,
    results: Vec<QueryResult>,
    selected: usize,
}

impl State {
    fn new() -> Self {
        let mut state = Self {
            plugins: vec![
                Box::new(TextSearch),
                Box::new(Calculator),
                Box::new(Desktop::new()),
            ],
            ..Default::default()
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
        self.results.sort_by(|f, s| {
            s.score
                .partial_cmp(&f.score)
                .unwrap_or(std::cmp::Ordering::Less)
        });
        dbg!(&self.results);
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SearchChanged(content) => {
                if self.query != content {
                    self.query = content;
                    self.update_results();
                }
            }
            Message::SelectUp => self.selected = self.selected.saturating_sub(1),
            Message::SelectDown => {
                self.selected = self
                    .selected
                    .saturating_add(1)
                    .clamp(0, self.results.len().saturating_sub(1))
            }
            _ => {}
        }

        Task::none()
    }

    fn view<'a>(&'a self) -> Column<'a, Message> {
        let mut column =
            column![text_input("search...", &self.query).on_input(Message::SearchChanged)];

        for (i, result) in self.results.iter().enumerate() {
            let mut text = text(&result.text);
            if self.selected == i {
                text = text.color(Color::from_rgb(1., 0., 0.));
            }
            column = column.push(text)
        }

        column
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        keyboard::listen().filter_map(|ev| match ev {
            keyboard::Event::KeyPressed { key, .. } => match key.as_ref() {
                keyboard::Key::Named(Named::ArrowUp) => Some(Message::SelectUp),
                keyboard::Key::Named(Named::ArrowDown) => Some(Message::SelectDown),
                keyboard::Key::Named(Named::Escape) => process::exit(0),
                _ => None,
            },
            _ => None,
        })
    }
}

#[to_layer_message]
#[derive(Debug, Clone)]
enum Message {
    SelectUp,
    SelectDown,
    SearchChanged(String),
}
