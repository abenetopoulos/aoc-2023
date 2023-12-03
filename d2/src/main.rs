use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead};
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short('f'), long)]
    pub input_file_path: Option<PathBuf>,

    #[arg(long, default_value("false"))]
    pub solve_max_constraint: bool,
}

fn solve_with_max_constraint(file_reader: io::BufReader<fs::File>) {
    let max_balls: HashMap<String, u32> = [
        ("red".to_string(), 12),
        ("green".to_string(), 13),
        ("blue".to_string(), 14),
    ]
    .iter()
    .cloned()
    .collect();

    let mut sum = 0;
    for line in file_reader.lines() {
        let Ok(l) = line else {
            panic!("invalid line");
        };

        let mut split_line = l.split(':');
        let game_id = Vec::from_iter(split_line.next().unwrap().split(' '))[1];

        let rounds = split_line.next().unwrap().split(';');
        let mut maxes: HashMap<String, u32> = [
            ("red".to_string(), 0),
            ("green".to_string(), 0),
            ("blue".to_string(), 0),
        ]
        .iter()
        .cloned()
        .collect();

        for round in rounds {
            for substring in round.split(", ") {
                let mut iter = substring.trim().split(' ');

                let count = iter.next().unwrap().parse::<u32>().unwrap();
                let color = iter.next().unwrap().to_string();

                #[cfg(debug_assertions)]
                println!("Game {game_id} count {} for {}", count, color);

                maxes
                    .entry(color)
                    .and_modify(|e| *e = std::cmp::max(count, *e));
            }
        }

        let mut is_game_possible = true;
        for (color, max_count) in maxes {
            if max_count > max_balls[&color] {
                is_game_possible = false;
                break;
            }
        }

        if !is_game_possible {
            #[cfg(debug_assertions)]
            println!("Game {game_id} is not possible");
            continue;
        }

        #[cfg(debug_assertions)]
        println!("Game {game_id} is possible");

        sum += game_id.parse::<usize>().unwrap();
    }

    println!("{sum}");
}

fn solve_without_max_constraint(file_reader: io::BufReader<fs::File>) {
    let mut sum = 0;
    for line in file_reader.lines() {
        let Ok(l) = line else {
            panic!("invalid line");
        };

        let mut split_line = l.split(':');
        let game_id = Vec::from_iter(split_line.next().unwrap().split(' '))[1];

        let rounds = split_line.next().unwrap().split(';');
        let mut maxes: HashMap<String, u32> = [
            ("red".to_string(), 0),
            ("green".to_string(), 0),
            ("blue".to_string(), 0),
        ]
        .iter()
        .cloned()
        .collect();

        for round in rounds {
            for substring in round.split(", ") {
                let mut iter = substring.trim().split(' ');

                let count = iter.next().unwrap().parse::<u32>().unwrap();
                let color = iter.next().unwrap().to_string();

                #[cfg(debug_assertions)]
                println!("Game {game_id} count {} for {}", count, color);

                maxes
                    .entry(color)
                    .and_modify(|e| *e = std::cmp::max(count, *e));
            }
        }

        #[cfg(debug_assertions)]
        println!("Game {game_id}'s maxes are {:?}", maxes);

        let power = maxes.values().map(|e| *e).reduce(|acc, e| acc * e).unwrap();
        #[cfg(debug_assertions)]
        println!("Game {game_id}'s power is {}", power);

        sum += power;
    }

    println!("{sum}");
}

fn main() {
    let args = Args::parse();
    let Some(p) = args.input_file_path else {
        panic!("missing input file");
    };

    let f = fs::File::open(p).unwrap();
    let file_reader = io::BufReader::new(f);

    if args.solve_max_constraint {
        solve_with_max_constraint(file_reader);
    } else {
        solve_without_max_constraint(file_reader);
    }
}
