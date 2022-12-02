pub fn solution(input: String) {
    // Part 1
    let score_p1 = get_score_part_1(&input);
    println!("Part 1 score = {}", score_p1);

    let score_p2 = get_score_part_2(&input);
    println!("Part 2 score = {}", score_p2);
}

fn get_score_part_1(input: &String) -> u32 {
    // A = Rock ; B = Paper ; C = Scissors
    // X = Rock ; Y = Paper ; Z = Scissors

    let mut score = 0_u32;

    for line in input.lines() {
        let split_line = line.split_whitespace().into_iter().collect::<Vec<&str>>();
        let their_pick = split_line[0];
        let our_pick = split_line[1];

        if their_pick == "A" {
            if our_pick == "X" {
                score += 1 + 3;
            } else if our_pick == "Y" {
                score += 2 + 6;
            } else if our_pick == "Z" {
                score += 3;
            }
        } else if their_pick == "B" {
            if our_pick == "X" {
                score += 1;
            } else if our_pick == "Y" {
                score += 2 + 3;
            } else if our_pick == "Z" {
                score += 3 + 6;
            }
        } else if their_pick == "C" {
            if our_pick == "X" {
                score += 1 + 6;
            } else if our_pick == "Y" {
                score += 2;
            } else if our_pick == "Z" {
                score += 3 + 3;
            }
        }
    }

    score
}

fn get_score_part_2(input: &String) -> u32 {
    // A = Rock ; B = Paper ; C = Scissors
    // X = Lose ; Y = Draw ; Z = Win

    let mut score = 0_u32;

    for line in input.lines() {
        let split_line = line.split_whitespace().into_iter().collect::<Vec<&str>>();
        let their_pick = split_line[0];
        let our_pick = split_line[1];

        if their_pick == "A" {
            if our_pick == "X" {
                score += 3;
            } else if our_pick == "Y" {
                score += 1 + 3;
            } else if our_pick == "Z" {
                score += 2 + 6;
            }
        } else if their_pick == "B" {
            if our_pick == "X" {
                score += 1;
            } else if our_pick == "Y" {
                score += 2 + 3;
            } else if our_pick == "Z" {
                score += 3 + 6;
            }
        } else if their_pick == "C" {
            if our_pick == "X" {
                score += 2;
            } else if our_pick == "Y" {
                score += 3 + 3;
            } else if our_pick == "Z" {
                score += 1 + 6;
            }
        }
    }

    score
}