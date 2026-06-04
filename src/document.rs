use crate::block::Block;
use crate::heading::HeadingLevel;

/// A Markdown document that can be built incrementally and rendered to a string.
///
/// `Document` uses a builder pattern — each method takes ownership of the document,
/// adds a block, and returns the document so calls can be chained.
///
/// # Examples
///
/// ```
/// use mdgen::{Document, HeadingLevel};
///
/// let output = Document::new()
///     .heading(HeadingLevel::H1, "My Document")
///     .paragraph("This is an introduction.")
///     .render();
///
/// assert_eq!(output, "# My Document\n\nThis is an introduction.\n");
/// ```
pub struct Document {
    blocks: Vec<Block>,
}

impl Default for Document {
    fn default() -> Self {
        Self { blocks: Vec::new() }
    }
}

impl Document {
    /// Creates a new empty document with no blocks.
    ///
    /// # Examples
    ///
    /// ```
    /// use mdgen::Document;
    ///
    /// let doc = Document::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Appends a heading block to the document.
    ///
    /// # Examples
    ///
    /// ```
    /// use mdgen::{Document, HeadingLevel};
    ///
    /// let doc = Document::new()
    ///     .heading(HeadingLevel::H1, "Introduction");
    /// ```
    pub fn heading(mut self, level: HeadingLevel, text: impl Into<String>) -> Self {
        self.blocks.push(Block::Heading {
            level,
            text: text.into(),
        });
        self
    }

    /// Appends a bullet list block to the document.
    ///
    /// Accepts any iterable of items that can be converted into a [`String`],
    /// including `Vec<&str>`, `Vec<String>`, and arrays.
    ///
    /// # Examples
    ///
    /// ```
    /// use mdgen::Document;
    ///
    /// let doc = Document::new()
    ///     .bullet_list(vec!["First item", "Second item", "Third item"]);
    /// ```
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

    /// Appends a numbered list block to the document.
    ///
    /// Accepts any iterable of items that can be converted into a [`String`],
    /// including `Vec<&str>`, `Vec<String>`, and arrays.
    ///
    /// # Examples
    ///
    /// ```
    /// use mdgen::Document;
    ///
    /// let doc = Document::new()
    ///     .numbered_list(vec!["First item", "Second item", "Third item"]);
    /// ```
    pub fn numbered_list<I, S>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.blocks.push(Block::NumberedList {
            items: items.into_iter().map(|s| s.into()).collect(),
        });

        self
    }

    /// Appends a paragraph block to the document.
    ///
    /// # Examples
    ///
    /// ```
    /// use mdgen::Document;
    ///
    /// let doc = Document::new()
    ///     .paragraph("This is a paragraph.");
    /// ```
    pub fn paragraph(mut self, text: impl Into<String>) -> Self {
        self.blocks.push(Block::Paragraph { text: text.into() });
        self
    }

    /// Renders the document to a Markdown string.
    ///
    /// Blocks are separated by a blank line. The output always ends with a newline.
    ///
    /// # Examples
    ///
    /// ```
    /// use mdgen::{Document, HeadingLevel};
    ///
    /// let output = Document::new()
    ///     .heading(HeadingLevel::H1, "Hello")
    ///     .paragraph("World")
    ///     .render();
    ///
    /// assert_eq!(output, "# Hello\n\nWorld\n");
    /// ```
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

                Block::NumberedList { items } => items
                    .iter()
                    .enumerate()
                    .map(|(i, item)| format!("{}. {}", i + 1, item))
                    .collect::<Vec<_>>()
                    .join("\n"),
            };

            parts.push(rendered);
        }

        if parts.is_empty() {
            return String::new();
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
    fn numbered_list_adds_numbered_list_block() {
        let doc: Document = Document::new().numbered_list(vec!["A", "B"]);

        let block: &Block = &doc.blocks[0];

        match block {
            Block::NumberedList { items } => {
                assert_eq!(*items, vec!["A", "B"]);
            }

            other => panic!("expected numbered list block, got {:#?}", other),
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
    fn h2_has_correct_marker() {
        assert_eq!(HeadingLevel::H2.marker(), "##");
    }

    #[test]
    fn render_empty_document() {
        let doc: Document = Document::new();

        let output: String = doc.render();

        let expected: &str = "";

        assert_eq!(output, expected);
    }

    #[test]
    fn render_heading_only_document() {
        let doc: Document = Document::new().heading(HeadingLevel::H2, "H2 Heading");

        let output: String = doc.render();

        let expected: &str = "## H2 Heading\n";

        assert_eq!(output, expected);
    }

    #[test]
    fn render_basic_document() {
        let doc: Document = Document::new()
            .heading(HeadingLevel::H1, "Title")
            .paragraph("Hello world")
            .bullet_list(vec!["A", "B"])
            .numbered_list(vec!["C", "D"]);

        let output: String = doc.render();

        let expected: &str = "# Title\n\nHello world\n\n- A\n- B\n\n1. C\n2. D\n";

        assert_eq!(output, expected);
    }
}
