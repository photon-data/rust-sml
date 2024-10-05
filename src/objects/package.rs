// src/objects/catalog.rs

pub struct Catalog {
    pub unique_name: String,
    pub items: Vec<String>,
}

impl Catalog {
    pub fn new(name: &str) -> Self {
        Catalog {
            name: name.to_string(),
            items: Vec::new(),
        }
    }

    pub fn add_item(&mut self, item: String) {
        self.items.push(item);
    }
}
