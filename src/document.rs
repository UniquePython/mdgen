enum Block {
    Heading { level: u8, text: String },

    Paragraph { text: String },

    BulletList { items: Vec<String> },
}

pub struct Document {
    blocks: Vec<Block>,
}

impl Default for Document {
    fn default() -> Self {
        Self { blocks: Vec::new() }
    }
}

impl Document {
    pub fn new() -> Self {
        Self::default()
    }
}
