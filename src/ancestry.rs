// modules
use crate::Size;

// ancestry struct to store ancestry data
#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Ancestry {
    pub name:       String, // name of the ancestry
    pub speed:      u8,     // the base speed of the ancestry
    pub hit_points: u8,     // the base hit points of the ancestry
    pub size:       Size,   // base size of the ancestry
}

// ancestry functions
impl Ancestry {
    // function to create ancestry struct from template
    pub fn template(template: &(&str, u8, u8, Size)) -> Self {
        Self {
            name:       template.0.to_string(),
            speed:      template.1,
            hit_points: template.2,
            size:       template.3,
        }
    }
}