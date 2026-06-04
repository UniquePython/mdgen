// Empty input produces no output rather than empty markers like `****`,
// which are valid Markdown but meaningless and visually noisy.

// We trust the caller not to pass text that already contains Markdown
// syntax. Users of this crate should not need to write raw Markdown.

fn wrap(text: &str, marker: &str) -> String {
    if text.is_empty() {
        return String::new();
    }
    format!("{}{}{}", marker, text, marker)
}

pub fn bold(text: &str) -> String {
    wrap(text, "**")
}

pub fn italic(text: &str) -> String {
    wrap(text, "*")
}

pub fn code(text: &str) -> String {
    wrap(text, "`")
}

pub fn strikethrough(text: &str) -> String {
    wrap(text, "~~")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bold_normal_input() {
        let bold_text: String = bold("text");

        assert_eq!(bold_text, "**text**");
    }

    #[test]
    fn bold_empty_input() {
        let bold_text: String = bold("");

        assert_eq!(bold_text, "");
    }

    #[test]
    fn italic_normal_input() {
        let italic_text: String = italic("text");

        assert_eq!(italic_text, "*text*");
    }

    #[test]
    fn italic_empty_input() {
        let italic_text: String = italic("");

        assert_eq!(italic_text, "");
    }

    #[test]
    fn code_normal_input() {
        let code_text: String = code("text");

        assert_eq!(code_text, "`text`");
    }

    #[test]
    fn code_empty_input() {
        let code_text: String = code("");

        assert_eq!(code_text, "");
    }

    #[test]
    fn strikethrough_normal_input() {
        let strikethrough_text: String = strikethrough("text");

        assert_eq!(strikethrough_text, "~~text~~");
    }

    #[test]
    fn strikethrough_empty_input() {
        let strikethrough_text: String = strikethrough("");

        assert_eq!(strikethrough_text, "");
    }
}
