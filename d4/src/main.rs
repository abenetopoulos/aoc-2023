use core::str::Split;
use std::fs;
use std::io::{self, BufRead};
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short('f'), long)]
    pub input_file_path: Option<PathBuf>,

    #[arg(short('s'), long, default_value("false"))]
    pub scores: bool,
}

fn parse_numbers_list(numbers_list_iter: &mut Split<char>) -> Vec<usize> {
    numbers_list_iter
        .next()
        .unwrap()
        .trim()
        .split(char::is_whitespace)
        .filter(|e| e.len() > 0)
        .map(|c| c.trim().parse::<usize>().unwrap())
        .collect()
}

struct Card {
    winning_numbers: Vec<usize>,
    our_numbers: Vec<usize>,
}

impl Card {
    fn get_score(&self) -> u32 {
        let mut card_matches = 0;
        let mut min_matched_winning_number = 0;
        for number in &self.our_numbers {
            for (idx, winning_number) in self.winning_numbers[min_matched_winning_number..]
                .iter()
                .enumerate()
            {
                if *number == *winning_number {
                    min_matched_winning_number = idx;
                    card_matches += 1;
                    break;
                }
            }
        }

        card_matches
    }
}

fn parse_card(game_numbers: &str) -> Card {
    let mut numbers_list_iter = game_numbers.split('|');
    let mut winning_numbers: Vec<usize> = parse_numbers_list(&mut numbers_list_iter);
    let mut our_numbers: Vec<usize> = parse_numbers_list(&mut numbers_list_iter);

    winning_numbers.sort();
    our_numbers.sort();

    Card {
        winning_numbers,
        our_numbers,
    }
}

fn part_one(file_reader: io::BufReader<fs::File>) {
    let mut score: usize = 0;
    for line in file_reader.lines() {
        let Ok(l) = line else {
            panic!("invalid line");
        };
        let mut split_line = l.split(':');
        let game = split_line.next().unwrap();
        let card = parse_card(split_line.next().unwrap());

        let card_matches = card.get_score();

        #[cfg(debug_assertions)]
        println!("{game}: {card_matches} matches");
        score += 2_usize.pow(card_matches - 1);
    }

    println!("{score}");
}

fn part_two(file_reader: io::BufReader<fs::File>) {
    let mut num_cards: usize = 0;
    let mut num_copies: Vec<usize> = vec![0];

    for line in file_reader.lines() {
        let Ok(l) = line else {
            panic!("invalid line");
        };
        num_cards += 1;
        let current_card_copies = match num_copies.is_empty() {
            true => 1,
            false => {
                let copies = num_copies.remove(0);
                num_cards += copies;
                copies + 1
            }
        };

        let mut split_line = l.split(':');
        let game = split_line.next().unwrap();
        let card = parse_card(split_line.next().unwrap());

        #[cfg(debug_assertions)]
        println!("{game}: current_card_copies: {current_card_copies}, card score {}", card.get_score());

        let mut card_matches = card.get_score() as usize;
        num_copies = num_copies.iter().map(|e| {
            match card_matches > 0 {
                true => {
                    card_matches -= 1;
                    *e + current_card_copies
                },
                false => *e
            }
        }).collect();
        for _ in 0..card_matches {
            num_copies.push(current_card_copies);
        }

        #[cfg(debug_assertions)]
        println!("{game}: {card_matches} matches, num_copies: {:?}", num_copies);
    }

    println!("{num_cards}");
}

fn main() {
    let args = Args::parse();
    let Some(p) = args.input_file_path else {
        panic!("missing input file");
    };

    let f = fs::File::open(p).unwrap();
    let file_reader = io::BufReader::new(f);

    if args.scores {
        part_one(file_reader);
    } else {
        part_two(file_reader);
    }
}
