#[derive(Debug)]
pub struct Label {
    name: String,
    position: usize,
}

impl Label {
    pub fn new(name: String, position: usize) -> Self {
        Self { name, position }
    }
}

#[derive(Debug)]
pub struct LabelTable {
    labels: Vec<Label>,
}

impl LabelTable {
    pub fn new() -> Self {
        Self { labels: Vec::new() }
    }

    pub fn find(&self, label_name: &str) -> Option<usize> {
        let label_name = label_name.to_string();
        for label in &self.labels {
            if label.name == label_name {
                return Some(label.position);
            }
        }
        None
    }

    pub fn push(&mut self, label: Label) {
        self.labels.push(label);
    }
}
