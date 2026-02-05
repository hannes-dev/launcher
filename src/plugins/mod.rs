use crate::queriable::QueryResult;

pub mod calculator;
pub mod desktop;
pub mod text_search;

fn scored_match(text: &str, query: &str) -> Option<QueryResult> {
    let mut text_chars = text.chars();
    let mut query_chars = query.chars();
    let mut count = 0;
    while let (Some(tc), Some(qc)) = (text_chars.next(), query_chars.next()) {
        if tc.to_ascii_lowercase() == qc.to_ascii_lowercase() {
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
