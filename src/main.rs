#![allow(dead_code)]

pub mod shared;
use shared::Day;
pub mod day1;
use day1::Day1;

use clap::Parser;
use std::process::exit;
use std::fs;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    day: u8,
    input_file: String,
}

fn main() {
    let args = Args::parse();
    let input_content = fs::read_to_string(args.input_file).unwrap();
    match args.day {
        1 => {
            let mut day1 = Day1::parse_input(input_content);
            println!("Part1: {}", day1.solve_part1());
            println!("Part2: {}", day1.solve_part2());
        }
        _ => {
            exit(1);
        },
    }
}
