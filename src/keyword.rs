// keyword struct to store keyword data
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Keyword {
    pub name: String,
    pub description: String
}

// keyword functions
impl Keyword {
    // create new keyword
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description
        }
    }
}