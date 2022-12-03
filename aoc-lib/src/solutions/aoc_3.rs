pub fn solution(input: String) {
    // Part 1
    let shared_items = get_shared_items_in_rucksacks(&input);
    let total_priority_of_shared_items = get_total_priority_for_items(&shared_items);
    println!("Total priority of all shared items: {}", total_priority_of_shared_items);

    // Part 2
    let shared_items_between_elves = get_shared_items_for_three_elves(&input);
    let total_priority_of_each_triplet = get_total_priority_for_items(&shared_items_between_elves);
    println!("Total priority of grouped elves: {}", total_priority_of_each_triplet);
}

fn get_shared_items_in_rucksacks(rucksacks: &String) -> Vec<char> {
    let mut shared_items: Vec<char> = Vec::new();

    for rucksack in rucksacks.lines() {
        let mid_point = rucksack.len() / 2;
        let first_compartment = &rucksack[0..mid_point];
        let second_compartment = &rucksack[mid_point..];

        shared_items.push(get_shared_item_in_compartments(first_compartment, second_compartment));
    }

    shared_items
}

fn get_shared_item_in_compartments(first_compartment: &str, second_compartment: &str) -> char {
    for item in first_compartment.chars() {
        if second_compartment.contains(item) {
            return item;
        }
    }

    panic!("Did not find item that is in both compartments");
}

fn get_shared_items_for_three_elves(rucksacks: &String) -> Vec<char> {
    let mut shared_items: Vec<char> = Vec::new();

    let split_rucksacks = rucksacks.lines().collect::<Vec<&str>>();

    for rucksack_number in (0..split_rucksacks.len()).step_by(3) {
        let first_rucksack = split_rucksacks[rucksack_number];
        let second_rucksack = split_rucksacks[rucksack_number+1];
        let third_rucksack = split_rucksacks[rucksack_number+2];


        shared_items.push(get_shared_item_between_elves(first_rucksack, second_rucksack, third_rucksack));
    }

    shared_items
}

fn get_shared_item_between_elves(first_elf: &str, second_elf: &str, third_elf: &str) -> char {
    for item in first_elf.chars() {
        if second_elf.contains(item) && third_elf.contains(item) {
            return item;
        }
    }

    panic!("Didn't find a shared item between three elves");
}

fn get_total_priority_for_items(items: &Vec<char>) -> u32 {
    let mut score = 0_u32;

    for item in items {
        if item.is_lowercase() {
            // Convert from decimal values to 1 - 26
            score += *item as u32 - 96;
        } else if item.is_uppercase() {
            // Convert from decimal value to 27 - 52
            score += *item as u32 - 38;
        }
    }

    score
}