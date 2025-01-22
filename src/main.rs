use std::collections::HashSet;

use clap::Parser;

/// Simple program to encrypt plaintext using playfair cipher
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// plaintext
    #[arg(short, long)]
    plaintext: String,

    /// Keyword for playfair cipher
    #[arg(short, long)]
    keyword: String,
}

fn get_keyword_grid(keyword_input: String) -> [[char; 5]; 5] {
    let mut seen = HashSet::new();
    // Remove duplicates and empty character
    let keyword: String = keyword_input
        .chars()
        .filter(|&ch| ch != ' ' && seen.insert(ch))
        .collect();

    let alphabets_without_j = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
        'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];
    let alphabet_len = alphabets_without_j.len();

    let uppercase_keyword: String = keyword.to_uppercase();
    let keyword_chars: Vec<char> = uppercase_keyword.chars().collect();
    let mut used_alphabet: HashSet<char> = HashSet::new();

    let mut grid: [[char; 5]; 5] = [[' '; 5]; 5];

    let keyword_length = uppercase_keyword.len();
    let mut used_keyword_count: usize = 0;
    let mut used_alphabet_index: usize = 0;

    for row in 0..5 {
        for col in 0..5 {
            if used_keyword_count < keyword_length {
                let keyword_char = keyword_chars[used_keyword_count];
                grid[row][col] = keyword_char;
                used_alphabet.insert(keyword_char);
                used_keyword_count = used_keyword_count + 1;
            } else {
                let mut char_for_keyword = ' ';
                for alphabet_index in used_alphabet_index..alphabet_len {
                    let alphabet = alphabets_without_j[alphabet_index];
                    let alphabet_is_used = used_alphabet.contains(&alphabet);
                    if alphabet_is_used {
                        continue;
                    } else {
                        char_for_keyword = alphabet;
                        used_alphabet.insert(char_for_keyword);
                        break;
                    }
                }
                grid[row][col] = char_for_keyword;
                used_alphabet_index = used_alphabet_index + 1;
            }
        }
    }
    println!("Keyword matrix:");
    for row in grid.iter() {
        for &ch in row.iter() {
            print!("{} ", ch);
        }
        println!();
    }
    return grid;
}

fn main() {
    let args = Args::parse();
    let plaintext = args.plaintext;
    let keyword = args.keyword;

    let keyword_grid = get_keyword_grid(keyword);
    // for (index, character) in plaintext.chars().enumerate() {}

    // for (index, character) in plaintext.chars().enumerate() {}
}
