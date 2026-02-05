use crate::queriable::{QueryPlugin, QueryResult};

pub struct Calculator;

impl QueryPlugin for Calculator {
    fn search(&self, query: &str) -> Vec<QueryResult> {
        let mut results = Vec::new();
        let mut context = fend_core::Context::new();

        let eval_str = query.trim_matches(['=', ' ']);
        if let Ok(result) = fend_core::evaluate(eval_str, &mut context)
            && let text = result.get_main_result()
            && !text.is_empty()
        {
            let score = if query.starts_with('=') { 1.0 } else { 0.5 };

            results.push(QueryResult {
                text: text.into(),
                id: String::new(),
                score: score,
            });
        }

        results
    }

    fn activate(&self, id: &str) {
        println!("selected {id}");
    }
}
