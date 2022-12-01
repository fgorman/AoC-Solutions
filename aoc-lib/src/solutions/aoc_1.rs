use std::collections::HashMap;

pub fn solution(input: String) {
    // Part 1
    let calories_per_elf = get_calories_per_elf(input);
    let highest_calories_held = get_highest_calories_held(&calories_per_elf);
    println!("Highest calories held by one elf: {}", highest_calories_held);

    // Part 2
    let top_3_elves = get_top_three_elves(&calories_per_elf);
    let sum_top_3_elves = sum_top_three_elves(top_3_elves);
    println!("Sum of top 3 elves: {}", sum_top_3_elves);
}

fn get_calories_per_elf(calories_list: String) -> HashMap<u32, u32> {
    let mut calories_per_elf: HashMap<u32, u32> = HashMap::new();

    let mut elf_number = 0_u32;

    for line in calories_list.lines() {
        if line.is_empty() {
            elf_number += 1;
            continue;
        }

        let line_to_u32_res = line.parse::<u32>();

        let calories = match line_to_u32_res {
            Ok(res) => res,
            Err(err) => panic!("Unable to convert line to u32: {}", err),
        };

        *calories_per_elf.entry(elf_number).or_insert(0) += calories;
    }

    calories_per_elf
}

fn get_highest_calories_held(calories_per_elf: &HashMap<u32, u32>) -> u32 {
    let mut highest_calories_held = u32::MIN;

    for (_, calories) in calories_per_elf {
        if *calories > highest_calories_held {
            highest_calories_held = *calories;
        }
    }

    highest_calories_held
}

fn get_top_three_elves(calories_per_elf: &HashMap<u32, u32>) -> (u32, u32, u32) {
    let mut elf_1 = u32::MIN;
    let mut elf_2 = u32::MIN;
    let mut elf_3 = u32::MIN;

    for (_, calories) in calories_per_elf {
        if *calories > elf_1 {
            elf_3 = elf_2;
            elf_2 = elf_1;
            elf_1 = *calories;
        } else if *calories > elf_2 {
            elf_3 = elf_2;
            elf_2 = *calories;
        } else if *calories > elf_3 {
            elf_3 = *calories;
        }
    }

    (elf_1, elf_2, elf_3)
}

fn sum_top_three_elves(top_3: (u32, u32, u32)) -> u32 {
    top_3.0 + top_3.1 + top_3.2
}