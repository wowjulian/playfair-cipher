use clap::Parser;

/// Simple program to encrypt plaintext using playfair cipher
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// plaintext
    #[arg(short, long)]
    plaintext: String,

    /// Keyword for playfair cipher.
    #[arg(short, long)]
    keyword: String,
}

fn main() {
    println!("Hello, world!");
}
