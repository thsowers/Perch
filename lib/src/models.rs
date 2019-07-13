#[derive(Serialize, Deserialize, Debug)]
pub struct Poems {
    pub poems: Vec<Poem>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Poem {
    pub author: String,
    pub title: String,
    pub text: Vec<String>,
}