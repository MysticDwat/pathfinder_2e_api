#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Keyword {
    pub id: i32,
    pub name: String,
    pub description: String
}

impl Keyword {
    pub fn new(id: i32, name: String, description: String) -> Self {
        Self {
            id,
            name,
            description
        }
    }
}