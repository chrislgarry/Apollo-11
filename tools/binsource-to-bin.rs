//! Convert a Virtual AGC binsource file (octal text) to raw binary.
//!
//! Usage: binsource-to-bin <input.binsource> <output.bin>
//!
//! The binsource format consists of:
//! - Bank headers: "BANK=NN" where NN is the bank number (octal or decimal)
//! - Data lines: 8 octal words per line, e.g., "50026 27064 30027 30026 50025 27066 30025 24553"
//! - Bugger words: checksum at end of each bank (last word)
//! - Comments: lines starting with ';'
//! - Blank lines: ignored
//!
//! Output: raw binary file with 16-bit big-endian words, banks in order.
//!
//! The AGC has 36 fixed-memory banks (banks 0-35), each containing 1024 words.
//! Total rope size: 36 * 1024 * 2 bytes = 73728 bytes.

use std::env;
use std::fs;
use std::io::Write;
use std::process;

const WORDS_PER_BANK: usize = 1024;
const NUM_BANKS: usize = 36;
const TOTAL_WORDS: usize = WORDS_PER_BANK * NUM_BANKS;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <input.binsource> <output.bin>", args[0]);
        process::exit(1);
    }

    let input_path = &args[1];
    let output_path = &args[2];

    let content = fs::read_to_string(input_path).unwrap_or_else(|e| {
        eprintln!("Error reading {}: {}", input_path, e);
        process::exit(1);
    });

    let rope = parse_binsource(&content);
    write_binary(&rope, output_path);
}

/// Parse a binsource file into a flat array of 16-bit words.
///
/// TODO: This is a great implementation opportunity. The binsource format
/// requires understanding the bank layout and octal word encoding.
///
/// Key details:
/// - Each line with 8 space-separated octal numbers is a data line
/// - "BANK=NN" lines set the current bank (NN may be octal or decimal)
/// - Lines starting with ';' are comments
/// - Each bank has exactly 1024 words (the last is a bugger/checksum word)
/// - Banks are not necessarily in sequential order in the file
/// - The output should place each bank's words at offset bank_num * 1024
fn parse_binsource(content: &str) -> Vec<u16> {
    let mut rope = vec![0u16; TOTAL_WORDS];
    let mut current_bank: usize = 0;
    let mut word_index: usize = 0;

    for line in content.lines() {
        let trimmed = line.trim();

        // Skip empty lines and comments
        if trimmed.is_empty() || trimmed.starts_with(';') {
            continue;
        }

        // Bank header
        if let Some(bank_str) = trimmed.strip_prefix("BANK=") {
            current_bank = bank_str.trim().parse::<usize>().unwrap_or_else(|_| {
                // Try octal parse
                usize::from_str_radix(bank_str.trim(), 8).unwrap_or_else(|_| {
                    eprintln!("Warning: could not parse bank number '{}'", bank_str);
                    0
                })
            });
            word_index = 0;
            continue;
        }

        // Data line: parse space-separated octal words
        for word_str in trimmed.split_whitespace() {
            if let Ok(word) = u16::from_str_radix(word_str, 8) {
                let offset = current_bank * WORDS_PER_BANK + word_index;
                if offset < TOTAL_WORDS {
                    rope[offset] = word;
                }
                word_index += 1;
            }
        }
    }

    rope
}

/// Write the rope as big-endian 16-bit words.
fn write_binary(rope: &[u16], path: &str) {
    let mut file = fs::File::create(path).unwrap_or_else(|e| {
        eprintln!("Error creating {}: {}", path, e);
        process::exit(1);
    });

    for &word in rope {
        file.write_all(&word.to_be_bytes()).unwrap_or_else(|e| {
            eprintln!("Error writing: {}", e);
            process::exit(1);
        });
    }

    eprintln!(
        "Wrote {} words ({} bytes) to {}",
        rope.len(),
        rope.len() * 2,
        path
    );
}
