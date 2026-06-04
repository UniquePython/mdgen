/// The level of a Markdown heading, from `H1` (largest) to `H6` (smallest).
///
/// Used with [`Document::heading`] to specify how prominent a heading should be.
///
/// # Examples
///
/// ```
/// use mdgen::HeadingLevel;
///
/// let level = HeadingLevel::H1;
/// assert_eq!(level.marker(), "#");
/// ```
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
    /// Returns the Markdown prefix string for this heading level.
    ///
    /// # Examples
    ///
    /// ```
    /// use mdgen::HeadingLevel;
    ///
    /// assert_eq!(HeadingLevel::H3.marker(), "###");
    /// ```
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
