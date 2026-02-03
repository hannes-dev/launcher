pub trait QueryPlugin {
    fn search(&self, query: &str) -> Vec<QueryResult>;
    fn activate(&self, id: &str);
}

pub struct QueryResult {
    // icon
    pub text: String,
    pub id: String,
    pub score: f32,
}
