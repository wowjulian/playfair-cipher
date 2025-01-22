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

fn find_row(grid: [[char; 5]; 5], diagraph: char) -> usize {
    let target = if diagraph == 'J' { 'I' } else { diagraph };
    for row in 0..5 {
        for col in 0..5 {
            if grid[row][col] == target {
                return row;
            }
        }
    }
    panic!("diagraph not found: {} target: {}", diagraph, target);
}

fn find_col(grid: [[char; 5]; 5], diagraph: char) -> usize {
    let target = if diagraph == 'J' { 'I' } else { diagraph };
    for row in 0..5 {
        for col in 0..5 {
            if grid[row][col] == target {
                return col;
            }
        }
    }
    panic!("diagraph not found: {} target: {}", diagraph, target);
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

    let plaintext_chars: Vec<char> = plaintext.chars().collect();

    let mut ciphertext: Vec<char> = Vec::new();
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

        // print!("{}", diagraph_first_letter);
        // print!("{}", diagraph_second_letter);

        let first_letter_row = find_row(keyword_grid, diagraph_first_letter);
        let first_letter_col = find_col(keyword_grid, diagraph_first_letter);
        let second_letter_row = find_row(keyword_grid, diagraph_second_letter);
        let second_letter_col = find_col(keyword_grid, diagraph_second_letter);

        let is_same_row = first_letter_row == second_letter_row;
        if is_same_row {
            let first_letter_ciphertext =
                keyword_grid[first_letter_row][(first_letter_col + 1) % 5];
            let second_letter_ciphertext =
                keyword_grid[second_letter_row][(second_letter_col + 1) % 5];
            ciphertext.push(first_letter_ciphertext);
            ciphertext.push(second_letter_ciphertext);
            continue;
        }
        let is_same_col = first_letter_col == second_letter_col;
        if is_same_col {
            let first_letter_ciphertext =
                keyword_grid[(first_letter_row + 1) % 5][first_letter_col];
            let second_letter_ciphertext =
                keyword_grid[(second_letter_row + 1) % 5][second_letter_col];
            ciphertext.push(first_letter_ciphertext);
            ciphertext.push(second_letter_ciphertext);
            continue;
        }

        let first_letter_ciphertext = keyword_grid[first_letter_row][second_letter_col];
        let second_letter_ciphertext = keyword_grid[second_letter_row][first_letter_col];
        ciphertext.push(first_letter_ciphertext);
        ciphertext.push(second_letter_ciphertext);
    }

    let ciphertext_output: String = ciphertext.into_iter().collect();
    println!("ciphertext: {}", ciphertext_output);
}
