use clap::Parser;

use aoc_lib::run_aoc_day;
use chrono::{ Utc, Datelike };

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    pub day: usize,

    #[arg(short, long)]
    pub year: usize,
}

fn main() {
    let args: Args = Args::parse();

    if args.day < 1 || args.day > 25 {
        panic!("Days are between 1 and 25 for AoC, dumbass");
    }

    let current_year = Utc::now().year() as usize;

    if args.year < 2015 || args.year > current_year {
        panic!("AoC started in 2015 and only can be up to the current year, dumbass");
    }

    run_aoc_day(args.year, args.day);
}
