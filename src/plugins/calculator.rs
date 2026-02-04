use crate::queriable::{QueryPlugin, QueryResult};

pub struct Calculator;

impl QueryPlugin for Calculator {
    fn search(&self, query: &str) -> Vec<QueryResult> {
        let mut results = Vec::new();
        let mut context = fend_core::Context::new();
        if let Ok(result) = fend_core::evaluate(query, &mut context)
            && let text = result.get_main_result()
            && !text.is_empty()
        {
            dbg!(text);
            results.push(QueryResult {
                text: text.into(),
                id: String::new(),
                score: 0.9,
            });
        }

        results
    }

    fn activate(&self, id: &str) {
        println!("selected {id}");
    }
}
