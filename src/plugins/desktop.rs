use freedesktop_desktop_entry::{Iter, default_paths, get_languages_from_env};

use crate::{
    plugins::scored_match,
    queriable::{QueryPlugin, QueryResult},
};

pub struct Desktop {
    entries: Vec<String>,
}

impl Desktop {
    pub fn new() -> Self {
        let locales = get_languages_from_env();
        let entries = Iter::new(default_paths())
            .entries(Some(&locales))
            .filter_map(|e| e.full_name(&locales).map(String::from))
            .collect::<Vec<_>>();

        Self { entries }
    }
}

impl QueryPlugin for Desktop {
    fn search(&self, query: &str) -> Vec<QueryResult> {
        self.entries
            .iter()
            .filter_map(|s| scored_match(s, query))
            .collect()
    }

    fn activate(&self, id: &str) {
        println!("selected {id}");
    }
}
