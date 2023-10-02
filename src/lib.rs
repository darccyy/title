use clap::Parser;
use strum::EnumString;

/// Print fancy title to stdout
pub fn print_title(text: &str, width: usize, style: Style) {
    let chars = match style {
        Style::Unicode => Chars::from(' ', '─', '│', '┌', '┐', '└', '┘'),
        Style::Ascii => Chars::from(' ', '-', ' ', '-', '-', '-', '-'),
        Style::Borderless => Chars::from(' ', ' ', ' ', ' ', ' ', ' ', ' '),
    };

    let width_inner = width.checked_sub(2).unwrap_or(0);
    let padding_total = width_inner.checked_sub(text.len()).unwrap_or(0);
    let padding_left = padding_total / 2;
    let padding_right = padding_total - padding_left;

    print!("{}", chars.top_left);
    print!("{}", chars.horizontal.to_string().repeat(width_inner));
    print!("{}", chars.top_right);
    println!();

    print!("{}", chars.vertical);
    print!("{}", chars.space.to_string().repeat(padding_left));
    print!("{}", text);
    print!("{}", chars.space.to_string().repeat(padding_right));
    print!("{}", chars.vertical);
    println!();

    print!("{}", chars.bottom_left);
    print!("{}", chars.horizontal.to_string().repeat(width_inner));
    print!("{}", chars.bottom_right);
    println!();
}

/// Title
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

/// Character style to use
#[derive(Debug, EnumString, Clone, Copy)]
pub enum Style {
    /// UTF-8 box drawing characters
    #[strum(serialize = "u", serialize = "unicode", serialize = "utf8")]
    Unicode,
    /// Simple ASCII characters
    #[strum(serialize = "a", serialize = "ascii")]
    Ascii,
    /// Do not draw a border
    #[strum(serialize = "b", serialize = "borderless")]
    Borderless,
}

/// Chars used in box drawing around title
struct Chars {
    pub space: char,
    pub horizontal: char,
    pub vertical: char,
    pub top_left: char,
    pub top_right: char,
    pub bottom_left: char,
    pub bottom_right: char,
}

impl Chars {
    /// Wrapper for creating `Chars`
    pub fn from(
        space: char,
        horizontal: char,
        vertical: char,
        top_left: char,
        top_right: char,
        bottom_left: char,
        bottom_right: char,
    ) -> Self {
        Self {
            space,
            horizontal,
            vertical,
            top_left,
            top_right,
            bottom_left,
            bottom_right,
        }
    }
}
