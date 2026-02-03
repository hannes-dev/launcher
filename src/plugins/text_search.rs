use crate::queriable::{QueryPlugin, QueryResult};

const TERMS: [&'static str; 11] = [
    "cheese", "potato", "carrot", "house", "bear", "clobber", "turkey", "mouse", "crab", "curtain",
    "zeus",
];

pub struct TextSearch;

impl TextSearch {
    fn make_match(text: &str, query: &str) -> Option<QueryResult> {
        if text.starts_with(&query) {
            Some(QueryResult {
                text: text.to_string(),
                id: text.to_string(),
                score: 1.,
            })
        } else {
            None
        }
    }
}

impl QueryPlugin for TextSearch {
    fn search(&self, query: &str) -> Vec<QueryResult> {
        TERMS
            .into_iter()
            .filter_map(|s| Self::make_match(s, query))
            .collect()
    }

    fn activate(&self, id: &str) {
        println!("selected {id}");
    }
}
