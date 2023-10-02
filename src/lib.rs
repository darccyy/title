mod args;

pub use args::{Args, Style};

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
