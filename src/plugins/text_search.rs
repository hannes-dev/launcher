use crate::queriable::{QueryPlugin, QueryResult};

const TERMS: [&'static str; 11] = [
    "cheese", "potato", "carrot", "house", "bear", "clobber", "turkey", "mouse", "crab", "curtain",
    "zeus",
];

pub struct TextSearch;

impl TextSearch {
    fn make_match(text: &str, query: &str) -> Option<QueryResult> {
        let mut text_chars = text.chars();
        let mut query_chars = query.chars();
        let mut count = 0;
        while let (Some(tc), Some(qc)) = (text_chars.next(), query_chars.next()) {
            if tc == qc {
                count += 1;
            } else {
                count -= 1;
            }
        }

        if count > 0 {
            Some(QueryResult {
                text: text.into(),
                id: text.into(),
                score: count as f32 / query.len() as f32,
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
