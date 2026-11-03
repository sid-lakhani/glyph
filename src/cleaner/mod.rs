use crate::cli::CleanMode;

/// Cleans and formats recognized OCR text according to the selected mode.
#[must_use]
pub fn clean_text(raw: &str, mode: CleanMode) -> String {
    match mode {
        CleanMode::Prose => clean_prose(raw),
        CleanMode::Code => clean_code(raw),
        CleanMode::Line => clean_line(raw),
    }
}

/// Prose cleaner:
/// - Re-joins hyphenated word wraps ("multi-\nple" -> "multiple").
/// - Preserves blank lines (paragraph breaks).
/// - Joins single line-breaks within the same paragraph with a space.
/// - Trims excessive whitespace while keeping paragraph flow natural.
fn clean_prose(raw: &str) -> String {
    let normalized = raw.replace("\r\n", "\n").replace('\r', "\n");
    let paragraphs = normalized.split("\n\n");

    let mut cleaned_paragraphs = Vec::new();

    for para in paragraphs {
        let lines: Vec<&str> = para
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty())
            .collect();
        if lines.is_empty() {
            continue;
        }

        let mut para_text = String::new();
        for (i, line) in lines.iter().enumerate() {
            if i > 0 {
                // If previous line ended with a hyphen and letter, join word directly
                if para_text.ends_with('-')
                    && para_text
                        .chars()
                        .nth_back(1)
                        .is_some_and(|c| c.is_alphabetic())
                    && line.chars().next().is_some_and(|c| c.is_alphabetic())
                {
                    para_text.pop(); // Remove trailing hyphen
                } else {
                    para_text.push(' ');
                }
            }
            para_text.push_str(line);
        }

        let trimmed_para = para_text.trim();
        if !trimmed_para.is_empty() {
            cleaned_paragraphs.push(trimmed_para.to_string());
        }
    }

    cleaned_paragraphs.join("\n\n")
}

/// Code cleaner:
/// - Preserves leading indentation (spaces and tabs).
/// - Normalizes curly/smart quotes to standard ASCII quotes.
/// - Trims trailing whitespace on each line.
/// - Preserves empty lines and code structure.
fn clean_code(raw: &str) -> String {
    let normalized = raw
        .replace("\r\n", "\n")
        .replace('\r', "\n")
        .replace(['“', '”'], "\"")
        .replace(['‘', '’'], "'");

    let lines: Vec<String> = normalized
        .lines()
        .map(|l| l.trim_end().to_string())
        .collect();

    // Trim leading and trailing empty lines from overall snippet
    let start = lines.iter().position(|l| !l.is_empty()).unwrap_or(0);
    let end = lines
        .iter()
        .rposition(|l| !l.is_empty())
        .map_or(0, |p| p + 1);

    if start >= end {
        return String::new();
    }

    lines[start..end].join("\n")
}

/// Line cleaner:
/// - Collapses all newlines, carriage returns, and tabs into single spaces.
/// - Collapses consecutive spaces into a single space.
/// - Trims start and end.
fn clean_line(raw: &str) -> String {
    raw.split_whitespace().collect::<Vec<&str>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prose_rejoins_hyphenation() {
        let input = "This is a demon-\nstration of hyphen split.";
        let cleaned = clean_text(input, CleanMode::Prose);
        assert_eq!(cleaned, "This is a demonstration of hyphen split.");
    }

    #[test]
    fn test_prose_preserves_paragraphs() {
        let input = "Paragraph one line 1\nline 2.\n\nParagraph two line 1\nline 2.";
        let cleaned = clean_text(input, CleanMode::Prose);
        assert_eq!(
            cleaned,
            "Paragraph one line 1 line 2.\n\nParagraph two line 1 line 2."
        );
    }

    #[test]
    fn test_code_preserves_indentation_and_normalizes_quotes() {
        let input = "  fn main() {\n    let s = “hello world”;\n  }\n";
        let cleaned = clean_text(input, CleanMode::Code);
        assert_eq!(cleaned, "  fn main() {\n    let s = \"hello world\";\n  }");
    }

    #[test]
    fn test_line_mode_collapses_all() {
        let input = "First line\n   Second line   with\t tabs \nand spaces.";
        let cleaned = clean_text(input, CleanMode::Line);
        assert_eq!(cleaned, "First line Second line with tabs and spaces.");
    }
}
