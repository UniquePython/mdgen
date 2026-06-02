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

#[derive(Debug)]
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

    pub fn heading(mut self, level: HeadingLevel, text: impl Into<String>) -> Self {
        self.blocks.push(Block::Heading {
            level,
            text: text.into(),
        });
        self
    }

    pub fn bullet_list<I, S>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.blocks.push(Block::BulletList {
            items: items.into_iter().map(|s| s.into()).collect(),
        });

        self
    }

    pub fn paragraph(mut self, text: impl Into<String>) -> Self {
        self.blocks.push(Block::Paragraph { text: text.into() });
        self
    }

    pub fn render(&self) -> String {
        let mut parts: Vec<String> = Vec::new();

        for block in &self.blocks {
            let rendered: String = match block {
                Block::Heading { level, text } => {
                    format!("{} {}", level.marker(), text)
                }

                Block::Paragraph { text } => text.clone(),

                Block::BulletList { items } => items
                    .iter()
                    .map(|item| format!("- {}", item))
                    .collect::<Vec<_>>()
                    .join("\n"),
            };

            parts.push(rendered);
        }

        let mut result: String = parts.join("\n\n");
        result.push('\n');
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_document_is_empty() {
        let doc: Document = Document::new();

        assert_eq!(doc.blocks.len(), 0);
    }

    #[test]
    fn heading_adds_heading_block() {
        let doc: Document = Document::new().heading(HeadingLevel::H1, "Title");

        let block: &Block = &doc.blocks[0];

        match block {
            Block::Heading { level, text } => {
                assert_eq!(*level, HeadingLevel::H1);
                assert_eq!(text, "Title");
            }

            other => panic!("expected heading block, got {:#?}", other),
        }
    }

    #[test]
    fn bullet_list_adds_bullet_list_block() {
        let doc: Document = Document::new().bullet_list(vec!["A", "B"]);

        let block: &Block = &doc.blocks[0];

        match block {
            Block::BulletList { items } => {
                assert_eq!(*items, vec!["A", "B"]);
            }

            other => panic!("expected bullet list block, got {:#?}", other),
        }
    }

    #[test]
    fn paragraph_adds_paragraph_block() {
        let doc: Document = Document::new().paragraph("text");

        let block: &Block = &doc.blocks[0];

        match block {
            Block::Paragraph { text } => {
                assert_eq!(*text, "text");
            }

            other => panic!("expected paragraph block, got {:#?}", other),
        }
    }

    #[test]
    fn render_basic_document() {
        let doc = Document::new()
            .heading(HeadingLevel::H1, "Title")
            .paragraph("Hello world")
            .bullet_list(vec!["A", "B"]);

        let output = doc.render();

        let expected = "# Title\n\nHello world\n\n- A\n- B\n";

        assert_eq!(output, expected);
    }
}
