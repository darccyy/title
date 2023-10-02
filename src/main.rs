use clap::Parser;
use std::io;
use title::{print_title, Args};

fn main() {
    let args = Args::parse();

    // Use given width, or get from terminal
    let width = args.width.unwrap_or_else(|| {
        term_size::dimensions()
            .expect("Failed to get terminal columns. Try running with `--width` argument")
            .0
    });

    // From argument
    let mut text = args.text.join(" ");

    // From stdin
    if text.trim().is_empty() {
        io::stdin()
            .read_line(&mut text)
            .expect("Failed to read line");
    }

    print_title(text.trim(), width, args.style);
}
