use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Section {
    pub id: String,
    pub project_id: String,
    #[serde(default)]
    pub order: i32,
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Default)]
pub struct Sections {
    pub sections: Vec<Section>,
}

impl Sections {
    pub fn new(items: Vec<Section>) -> Sections {
        Sections { sections: items }
    }
}
