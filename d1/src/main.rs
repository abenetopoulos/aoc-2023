use std::fs;
use std::io::{self, BufRead};
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short('f'), long)]
    pub input_file_path: Option<PathBuf>,
}

const DIGITS: [(&str, u8); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn parse_first_digit(line: &String) -> String {
    let digit_idx = match line.find(|c| char::is_digit(c, 10)) {
        Some(d) => d,
        _ => line.len(),
    };

    let mut substr_idx = line.len();
    let mut substr_digit = 0;
    for (digit_str, digit) in DIGITS {
        match line.find(digit_str) {
            Some(d) => {
                if d > substr_idx {
                    continue;
                }
                substr_idx = d;
                substr_digit = digit;
            }
            None => (),
        }
    }

    match substr_idx < digit_idx {
        true => substr_digit.to_string(),
        false => (line.as_bytes()[digit_idx] - 48).to_string(),
    }
}

fn parse_last_digit(line: &String) -> String {
    let digit_idx = match line.rfind(|c| char::is_digit(c, 10)) {
        Some(d) => d,
        _ => 0,
    };

    let mut substr_idx = 0;
    let mut substr_digit = 0;
    for (digit_str, digit) in DIGITS {
        match line.rfind(digit_str) {
            Some(d) => {
                if d < substr_idx {
                    continue;
                }
                substr_idx = d;
                substr_digit = digit;
            }
            None => (),
        }
    }

    match substr_idx > digit_idx {
        true => substr_digit.to_string(),
        false => (line.as_bytes()[digit_idx] - 48).to_string(),
    }
}

fn main() {
    let args = Args::parse();
    let Some(p) = args.input_file_path else {
        panic!("missing input file");
    };

    let f = fs::File::open(p).unwrap();
    let file_reader = io::BufReader::new(f);
    let mut sum: usize = 0;
    for line in file_reader.lines() {
        let Ok(l) = line else {
            panic!("invalid line");
        };

        let first_digit_str = parse_first_digit(&l);
        let second_digit_str = parse_last_digit(&l);

        #[cfg(debug_assertions)]
        println!(
            "digits of {} are {} and {}",
            l, first_digit_str, second_digit_str
        );
        sum += (first_digit_str + &second_digit_str)
            .parse::<usize>()
            .unwrap();
    }
    println!("{sum}");
}
