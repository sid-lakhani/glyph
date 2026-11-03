use clap::{Parser, ValueEnum};

/// Text cleaning and formatting modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, ValueEnum)]
pub enum CleanMode {
    /// Unwrap prose line-breaks, re-join hyphenated words, preserve paragraphs.
    #[default]
    Prose,
    /// Preserve leading indentation, brackets, quotes, and code structure.
    Code,
    /// Strip all line-breaks into a single continuous line.
    Line,
}

#[derive(Parser, Debug)]
#[command(
    name = "glyph",
    author,
    version,
    about = "✦ Blazing-fast, minimal Wayland screen OCR & text sniper",
    long_about = "Drag a region on screen to recognize text or QR codes instantly,\ncopying the cleaned result to your clipboard with desktop notification."
)]
pub struct Cli {
    /// Preserve indentation and code structure (Code Mode)
    #[arg(short = 'c', long, conflicts_with = "line")]
    pub code: bool,

    /// Collapse text into a single continuous line (Line Mode)
    #[arg(short = 'l', long, conflicts_with = "code")]
    pub line: bool,

    /// Suppress desktop notifications
    #[arg(short = 's', long)]
    pub silent: bool,

    /// Output recognized text directly to stdout (pipe-friendly)
    #[arg(long)]
    pub stdout: bool,

    /// OCR language code (e.g. 'eng', 'deu', 'fra', 'jpn')
    #[arg(long, default_value = "eng")]
    pub lang: String,
}

impl Cli {
    /// Returns the selected text cleaning mode based on CLI flags.
    #[must_use]
    pub fn clean_mode(&self) -> CleanMode {
        if self.code {
            CleanMode::Code
        } else if self.line {
            CleanMode::Line
        } else {
            CleanMode::Prose
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_mode_selection() {
        let mut cli = Cli::parse_from(["glyph"]);
        assert_eq!(cli.clean_mode(), CleanMode::Prose);

        cli = Cli::parse_from(["glyph", "--code"]);
        assert_eq!(cli.clean_mode(), CleanMode::Code);

        cli = Cli::parse_from(["glyph", "-l"]);
        assert_eq!(cli.clean_mode(), CleanMode::Line);
    }
}
