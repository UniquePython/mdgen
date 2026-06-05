use crate::block::Block;
use crate::heading::HeadingLevel;

use std::fmt;
use std::io::{self};

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

    /// Renders the document to any [`std::io::Write`] target.
    ///
    /// This allows streaming output directly to files, stdout,
    /// network streams, or in-memory buffers.
    pub fn render_to<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        let mut first_block: bool = true;

        for block in &self.blocks {
            if !first_block {
                writeln!(writer)?;
                writeln!(writer)?;
            }

            first_block = false;

            match block {
                Block::Heading { level, text } => {
                    write!(writer, "{} {}", level.marker(), text)?;
                }

                Block::Paragraph { text } => {
                    write!(writer, "{}", text)?;
                }

                Block::BulletList { items } => {
                    for (i, item) in items.iter().enumerate() {
                        if i > 0 {
                            writeln!(writer)?;
                        }

                        write!(writer, "- {}", item)?;
                    }
                }

                Block::NumberedList { items } => {
                    for (i, item) in items.iter().enumerate() {
                        if i > 0 {
                            writeln!(writer)?;
                        }

                        write!(writer, "{}. {}", i + 1, item)?;
                    }
                }
            }
        }

        if !self.blocks.is_empty() {
            writeln!(writer)?;
        }

        Ok(())
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
        let mut buf: Vec<u8> = Vec::new();

        self.render_to(&mut buf)
            .expect("writing to Vec<u8> should never fail");

        String::from_utf8(buf).expect("generated markdown should always be valid UTF-8")
    }
}

impl fmt::Display for Document {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first_block: bool = true;

        for block in &self.blocks {
            if !first_block {
                writeln!(f)?;
                writeln!(f)?;
            }

            first_block = false;

            match block {
                Block::Heading { level, text } => {
                    write!(f, "{} {}", level.marker(), text)?;
                }

                Block::Paragraph { text } => {
                    write!(f, "{}", text)?;
                }

                Block::BulletList { items } => {
                    for (i, item) in items.iter().enumerate() {
                        if i > 0 {
                            writeln!(f)?;
                        }

                        write!(f, "- {}", item)?;
                    }
                }

                Block::NumberedList { items } => {
                    for (i, item) in items.iter().enumerate() {
                        if i > 0 {
                            writeln!(f)?;
                        }

                        write!(f, "{}. {}", i + 1, item)?;
                    }
                }
            }
        }

        if !self.blocks.is_empty() {
            writeln!(f)?;
        }

        Ok(())
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

    #[test]
    fn render_to_writer_basic_document() {
        let doc = Document::new()
            .heading(HeadingLevel::H1, "Title")
            .paragraph("Hello world");

        let mut buf = Vec::new();

        doc.render_to(&mut buf).unwrap();

        let output = String::from_utf8(buf).unwrap();

        assert_eq!(output, "# Title\n\nHello world\n");
    }

    #[test]
    fn display_matches_render() {
        let doc = Document::new()
            .heading(HeadingLevel::H1, "Title")
            .paragraph("Hello world");

        assert_eq!(doc.to_string(), doc.render());
    }
}
