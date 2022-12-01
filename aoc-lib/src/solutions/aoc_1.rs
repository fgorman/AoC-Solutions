pub fn solution(input: String) {
    // Part 1
    let mut calories_per_elf = get_calories_per_elf(input);
    let highest_calories_held = get_highest_calories_held(&mut calories_per_elf);
    println!("Highest calories held by one elf: {}", highest_calories_held);

    // Part 2
    let sum_top_3_elves = sum_top_three_elves(&mut calories_per_elf);
    println!("Sum of top 3 elves: {}", sum_top_3_elves);
}

fn get_calories_per_elf(calories_list: String) -> Vec<u32> {
    let mut calories_per_elf: Vec<u32> = Vec::new();

    let mut calories = 0_u32;

    for line in calories_list.lines() {
        if line.is_empty() {
            calories_per_elf.push(calories);
            calories = 0;
            continue;
        }

        let line_to_u32_res = line.parse::<u32>();

        calories += match line_to_u32_res {
            Ok(res) => res,
            Err(err) => panic!("Unable to convert line to u32: {}", err),
        };
    }

    calories_per_elf
}

fn get_highest_calories_held(calories_per_elf: &mut Vec<u32>) -> u32 {
    calories_per_elf.sort();

    calories_per_elf[calories_per_elf.len()-1]
}

fn sum_top_three_elves(calories_per_elf: &mut Vec<u32>) -> u32 {
    calories_per_elf.sort();

    let n = calories_per_elf.len();

    calories_per_elf[n-3] + calories_per_elf[n-2] + calories_per_elf[n-1]
}