#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeadingLevel {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

impl HeadingLevel {
    pub fn marker(self) -> &'static str {
        match self {
            HeadingLevel::H1 => "#",
            HeadingLevel::H2 => "##",
            HeadingLevel::H3 => "###",
            HeadingLevel::H4 => "####",
            HeadingLevel::H5 => "#####",
            HeadingLevel::H6 => "######",
        }
    }
}

enum Block {
    Heading { level: HeadingLevel, text: String },

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

    pub fn heading(mut self, level: HeadingLevel, text: &str) -> Self {
        self.blocks.push(Block::Heading {
            level,
            text: text.to_string(),
        });
        self
    }
}
