use reqwest::blocking::get as reqwest_get;

mod solutions;
use solutions::*;

pub fn run_aoc_day(day: usize) {
    let input = get_input(day);

    match day {
        1 => aoc_1::solution(input),
        2 => aoc_2::solution(input),
        3 => aoc_3::solution(input),
        4 => aoc_4::solution(input),
        5 => aoc_5::solution(input),
        6 => aoc_6::solution(input),
        7 => aoc_7::solution(input),
        8 => aoc_8::solution(input),
        9 => aoc_9::solution(input),
        10 => aoc_10::solution(input),
        11 => aoc_11::solution(input),
        12 => aoc_12::solution(input),
        13 => aoc_13::solution(input),
        14 => aoc_14::solution(input),
        15 => aoc_15::solution(input),
        16 => aoc_16::solution(input),
        17 => aoc_17::solution(input),
        18 => aoc_18::solution(input),
        19 => aoc_19::solution(input),
        20 => aoc_20::solution(input),
        21 => aoc_21::solution(input),
        22 => aoc_22::solution(input),
        23 => aoc_23::solution(input),
        24 => aoc_24::solution(input),
        25 => aoc_25::solution(input),
        _ => ()
    }
}

fn get_input(day: usize) -> String {
    let endpoint = format!("https://adventofcode.com/2022/day/{}/input", day);

    let reqwest_response = reqwest_get(endpoint);

    let response = match reqwest_response {
        Ok(response) => response,
        Err(err) => panic!("Error with getting day {} input: {}", day, err)
    };

    if !response.status().is_success() {
        panic!("Error with getting day {} input: {}", day, response.text().unwrap());
    }

    let response_text = match response.text() {
        Ok(text) =>  text,
        Err(err) => panic!("Error with getting text from response: {}", err)
    };

    response_text
}