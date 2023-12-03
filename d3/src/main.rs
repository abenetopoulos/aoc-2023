#![feature(iter_advance_by)]
use std::fs;
use std::io::{self, BufRead};
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short('f'), long)]
    pub input_file_path: Option<PathBuf>,

    #[arg(short('g'), long, default_value("false"))]
    pub only_gears: bool,
}

macro_rules! is_symbol {
    ($c:ident) => {
        $c != '.' && !char::is_alphanumeric($c)
    };
}

fn parse_number(substring: &str) -> (usize, usize) {
    let end_idx = substring
        .find(|c: char| c == '.' || is_symbol!(c))
        .or_else(|| Some(substring.len()))
        .unwrap();
    (substring[0..end_idx].parse().unwrap(), end_idx)
}

#[derive(Clone, Debug)]
struct UnmatchedNumber {
    number: usize,
    // row, [min-column, max-column)
    position: (usize, (usize, usize)),
}

#[derive(Clone, Debug)]
struct GearInfo {
    position: (usize, (usize, usize)),
    part_numbers: Vec<UnmatchedNumber>,
}

fn sum_symbol_adjacent_numbers(file_reader: io::BufReader<fs::File>, only_gears: bool) {
    let mut all_sum = 0;
    let mut unmatched_numbers = vec![];
    let mut active_symbols = vec![];
    let mut potential_gears = vec![];

    for (line_idx, line) in file_reader.lines().enumerate() {
        unmatched_numbers.retain(|n: &UnmatchedNumber| line_idx - n.position.0 < 2);
        active_symbols.retain(|s: &(usize, (usize, usize))| line_idx - s.0 < 2);
        potential_gears.retain(|g: &GearInfo| g.part_numbers.len() <= 2);

        let Ok(l) = line else {
            panic!("invalid line");
        };

        let mut line_chars = l.chars();
        let mut char_idx = 0;
        loop {
            let character = match line_chars.next() {
                Some(c) => c,
                None => break,
            };

            if character == '.' {
                char_idx += 1;
                continue;
            }

            if is_symbol!(character) {
                let symbol_position = (line_idx, (char_idx, char_idx + 1));
                let mut part_numbers = vec![];
                unmatched_numbers.retain(|n: &UnmatchedNumber| {
                    if symbol_position.0 - n.position.0 > 1 {
                        return true;
                    }

                    if (n.position.1 .0 as isize - symbol_position.1 .0 as isize).abs() < 2
                        || ((n.position.1 .1 - 1) as isize - symbol_position.1 .0 as isize).abs()
                            < 2
                    {
                        all_sum += n.number;
                        part_numbers.push(n.clone());
                        return false;
                    }

                    true
                });
                active_symbols.push(symbol_position);
                char_idx += 1;

                if character == '*' && part_numbers.len() <= 2 {
                    potential_gears.push(GearInfo {
                        position: symbol_position,
                        part_numbers,
                    });
                }

                continue;
            }

            if !character.is_digit(10) {
                char_idx += 1;
                continue;
            }

            let (number, number_str_len) = parse_number(&l[char_idx..]);
            #[cfg(debug_assertions)]
            println!(
                "found number {} on {}, {} with len {}",
                number, line_idx, char_idx, number_str_len
            );

            let current_number = UnmatchedNumber {
                number,
                position: (line_idx, (char_idx, char_idx + number_str_len)),
            };
            let mut matched = false;
            for symbol_position in &active_symbols {
                if current_number.position.0 - symbol_position.0 > 1 {
                    continue;
                }

                if (current_number.position.1 .0 as isize - symbol_position.1 .0 as isize).abs() < 2
                    || ((current_number.position.1 .1 - 1) as isize - symbol_position.1 .0 as isize)
                        .abs()
                        < 2
                {
                    all_sum += current_number.number;
                    matched = true;
                }
            }

            for potential_gear in &mut potential_gears {
                if (potential_gear.position.0 as isize - current_number.position.0 as isize).abs()
                    > 1
                {
                    continue;
                }

                if (current_number.position.1 .0 as isize - potential_gear.position.1 .0 as isize)
                    .abs()
                    < 2
                    || ((current_number.position.1 .1 - 1) as isize
                        - potential_gear.position.1 .0 as isize)
                        .abs()
                        < 2
                {
                    potential_gear.part_numbers.push(current_number.clone());
                }
            }

            if !matched {
                unmatched_numbers.push(current_number);
            }

            let _ = line_chars.advance_by(number_str_len - 1);
            char_idx += number_str_len;
        }

        #[cfg(debug_assertions)]
        println!("unmatched_numbers {:?}", unmatched_numbers);
        #[cfg(debug_assertions)]
        println!("active_symbols {:?}", active_symbols);
    }

    if only_gears {
        println!(
            "{}",
            potential_gears
                .iter()
                .filter(|mg| mg.part_numbers.len() == 2)
                .map(|mg| mg.part_numbers.iter().fold(1, |acc, pn| acc * pn.number))
                .fold(0, |acc, val| acc + val)
        );
    } else {
        println!("{all_sum}");
    }
}

fn main() {
    let args = Args::parse();
    let Some(p) = args.input_file_path else {
        panic!("missing input file");
    };

    let f = fs::File::open(p).unwrap();
    let file_reader = io::BufReader::new(f);

    sum_symbol_adjacent_numbers(file_reader, args.only_gears);
}
