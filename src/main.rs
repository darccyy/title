use clap::Parser;

use title::{print_title, Args};

fn main() {
    let args = Args::parse();

    let width = args.width.unwrap_or_else(|| {
        term_size::dimensions()
            .expect("Failed to get terminal columns. Try running with `--width` argument")
            .0
    });

    let text = args.text.join(" ");

    print_title(&text, args.style, width);
}
