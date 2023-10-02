use clap::Parser;
use strum::EnumString;

/// title
///
/// Display a fancy title in the terminal
#[derive(Parser)]
pub struct Args {
    /// Character style to use
    #[arg(short, long, default_value = "unicode")]
    pub style: Style,

    /// Override terminal width
    #[arg(short, long)]
    pub width: Option<usize>,

    /// Text to display
    pub text: Vec<String>,
}

#[derive(Debug, EnumString, Clone, Copy)]
pub enum Style {
    #[strum(serialize = "u", serialize = "unicode", serialize = "utf8")]
    Unicode,
    #[strum(serialize = "a", serialize = "ascii")]
    Ascii,
    #[strum(serialize = "b", serialize = "borderless")]
    Borderless,
}
