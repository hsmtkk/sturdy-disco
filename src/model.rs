use askama::Template;

pub struct TodoEntry {
    pub id: u32,
    pub text: String,
}

#[derive(Template)]
#[template(path="index.html")]
pub struct IndexTemplate {
    pub entries: Vec<TodoEntry>,
}
