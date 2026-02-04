use iced::{
    Color,
    keyboard::{self, key::Named},
    widget::{Column, column, text, text_input},
    window::close,
};

use iced_layershell::{
    Settings, application,
    reexport::{Anchor, Layer},
    settings::{LayerShellSettings, StartMode},
    to_layer_message,
};

use crate::{
    plugins::text_search::TextSearch,
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
            size: Some((0, 400)),
            exclusive_zone: -1,
            anchor: Anchor::Top | Anchor::Left | Anchor::Right,
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
            plugins: vec![Box::new(TextSearch)],
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
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ContendChanged(content) => {
                dbg!(&content);
                self.query = content;
                self.update_results();
            }
            Message::SelectUp => self.selected = self.selected.saturating_sub(1),
            Message::SelectDown => {
                self.selected = self
                    .selected
                    .saturating_add(1)
                    .clamp(0, self.results.len() - 1)
            }
            _ => {}
        }
    }

    fn view<'a>(&'a self) -> Column<'a, Message> {
        let mut column =
            column![text_input("search...", &self.query).on_input(Message::ContendChanged)];

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
            keyboard::Event::KeyPressed {
                key,
                modified_key,
                physical_key,
                location,
                modifiers,
                text,
                repeat,
            } => match key.as_ref() {
                keyboard::Key::Named(Named::ArrowUp) => Some(Message::SelectUp),
                keyboard::Key::Named(Named::ArrowDown) => Some(Message::SelectDown),
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
    ContendChanged(String),
}
