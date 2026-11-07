//! zotero model — reference manager.

#[derive(Debug, Clone)]
pub struct Item {
    pub title: String,
    pub type_: String,
    pub creators: Vec<String>,
    pub year: u32,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Collection {
    pub name: String,
    pub items: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub items: Vec<Item>,
    pub collections: Vec<Collection>,
}

impl Default for Model {
    fn default() -> Self { Self { items: vec![], collections: vec![] } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn add_item(&mut self, item: Item) { self.items.push(item); }

    pub fn add_collection(&mut self, collection: Collection) { self.collections.push(collection); }

    pub fn search(&self, query: &str) -> Vec<&Item> {
        self.items.iter().filter(|i| i.title.contains(query)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn item_search() {
        let mut z = Model::new();
        z.add_item(Item { title: "Paper1".into(), type_: "article".into(), creators: vec!["A".into()], year: 2020, tags: vec![] });
        assert_eq!(z.search("Paper").len(), 1);
    }
}
