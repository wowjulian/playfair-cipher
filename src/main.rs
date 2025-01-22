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

fn get_second_letter(isOverflow: bool) -> char {
    return ' ';
}
fn main() {
    let args = Args::parse();
    let plaintext_input = args.plaintext;
    let keyword_input = args.keyword;

    let keyword_grid = get_keyword_grid(keyword_input);

    let plaintext: String = plaintext_input
        .chars()
        .filter(|&ch| ch != ' ' && ch != ',' && ch != '.')
        .map(|ch| ch.to_ascii_uppercase())
        .collect();

    println!("plaintext: {}", plaintext);
    // for (index, character) in plaintext.chars().enumerate() {}
    // for (index, character) in plaintext.chars().enumerate() {}

    let plaintext_chars: Vec<char> = plaintext.chars().collect();

    let mut first_letter_index = 0;
    while first_letter_index < plaintext.len() {
        let second_letter_index: usize = first_letter_index + 1;
        let first_letter = plaintext_chars[first_letter_index];
        let second_letter_exists = second_letter_index < plaintext.len();
        let same_first_and_second_letter = if second_letter_exists {
            first_letter == plaintext_chars[second_letter_index]
        } else {
            false
        };
        let diagraph_first_letter = first_letter;

        let second_letter = if second_letter_exists {
            plaintext_chars[second_letter_index]
        } else {
            'X'
        };
        let diagraph_second_letter = if same_first_and_second_letter {
            'X'
        } else {
            second_letter
        };

        if same_first_and_second_letter {
            first_letter_index = first_letter_index + 1;
        } else {
            first_letter_index = first_letter_index + 2;
        }

        print!("{}", diagraph_first_letter);
        print!("{}", diagraph_second_letter);
        print!(" ");
    }
}
