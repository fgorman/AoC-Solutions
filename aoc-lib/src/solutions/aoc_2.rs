use std::collections::HashMap;

pub fn solution(input: String) {
    // Part 1
    let mut part_1_strat = HashMap::new();
    part_1_strat.insert("A X", 4);
    part_1_strat.insert("A Y", 8);
    part_1_strat.insert("A Z", 3);
    part_1_strat.insert("B X", 1);
    part_1_strat.insert("B Y", 5);
    part_1_strat.insert("B Z", 9);
    part_1_strat.insert("C X", 7);
    part_1_strat.insert("C Y", 2);
    part_1_strat.insert("C Z", 6);

    let score_p1 = get_score(&input, &part_1_strat);
    println!("Part 1 score = {}", score_p1);

    // Part 2
    let mut part_2_strat = HashMap::new();
    part_2_strat.insert("A X", 3);
    part_2_strat.insert("A Y", 4);
    part_2_strat.insert("A Z", 8);
    part_2_strat.insert("B X", 1);
    part_2_strat.insert("B Y", 5);
    part_2_strat.insert("B Z", 9);
    part_2_strat.insert("C X", 2);
    part_2_strat.insert("C Y", 6);
    part_2_strat.insert("C Z", 7);

    let score_p2 = get_score(&input, &part_2_strat);
    println!("Part 2 score = {}", score_p2);
}

fn get_score(input: &String, strategy: &HashMap<&str, u32>) -> u32 {
    // A = Rock ; B = Paper ; C = Scissors
    // X = Rock ; Y = Paper ; Z = Scissors

    let mut score = 0_u32;

    for line in input.lines() {
        score += strategy[line];
    }

    score
}