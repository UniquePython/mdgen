use crate::heading::HeadingLevel;

#[derive(Debug)]
pub(crate) enum Block {
    Heading { level: HeadingLevel, text: String },

    Paragraph { text: String },

    BulletList { items: Vec<String> },
    NumberedList { items: Vec<String> },
}
