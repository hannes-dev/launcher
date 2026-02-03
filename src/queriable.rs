pub trait QueryPlugin {
    fn query(&self, query: String) -> Vec<Match>;
    fn activate(&self, id: &str);
}

pub struct Match {
    // icon
    text: String,
    id: String,
    score: f32,
}
