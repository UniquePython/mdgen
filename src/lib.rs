mod block;
mod document;
mod heading;
mod inline;

pub use document::Document;
pub use heading::HeadingLevel;
pub use inline::{bold, code, italic, strikethrough};
