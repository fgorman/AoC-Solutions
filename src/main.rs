use clap::Parser;

use aoc_2022_lib::run_aoc_day;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    pub day: usize,
}

fn main() {
    let args: Args = Args::parse();

    if args.day < 1 || args.day > 25 {
        panic!("Days are between 1 and 25 for AoC, dumbass");
    }

    run_aoc_day(args.day);
}
