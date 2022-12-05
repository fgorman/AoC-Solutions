use regex::Regex;

type Stack = Vec<char>;

pub fn solution(input: String) {
    let stacks = get_initial_stacks(&input);

    // Part 1
    let mut p1_stacks = stacks.clone();
    process_cratemover_9000(&input, &mut p1_stacks);
    print_top_of_each_stack(p1_stacks);

    // Part 2
    let mut p2_stacks = stacks.clone();
    process_cratemover_9001(&input, &mut p2_stacks);
    print_top_of_each_stack(p2_stacks);
}

fn get_initial_stacks(input: &String) -> Vec<Stack> {
    let mut stacks: Vec<Stack> = vec![Vec::new(); 9];

    let pattern: Regex = Regex::new(r"\[(.*?)\]").unwrap();

    for line in input.lines() {
        if line.eq("") {
            break;
        }

        pattern.find_iter(line).for_each(|cr| {
            let stack_num = cr.start() / 4;
            stacks[stack_num].push(cr.as_str().chars().nth(1).unwrap());
        });
    }

    for stack in stacks.iter_mut() {
        stack.reverse();
    }

    stacks
}

fn process_cratemover_9000(input: &String, stacks: &mut Vec<Stack>) {
    for line in input.lines() {
        if !line.contains("move") {
            continue;
        }

        let words_in_line = line.split(" ").collect::<Vec<&str>>();

        // each move line in form of: move X from Y to Z
        let num_to_move: u32 = words_in_line[1].parse().unwrap();
        let from: usize = words_in_line[3].parse().unwrap();
        let to: usize = words_in_line[5].parse().unwrap();

        for _ in 0..num_to_move {
            let cr_to_move = stacks[from-1].pop().unwrap();
            stacks[to-1].push(cr_to_move);
        }
    }
}

fn process_cratemover_9001(input: &String, stacks: &mut Vec<Stack>) {
    for line in input.lines() {
        if !line.contains("move") {
            continue;
        }

        let words_in_line = line.split(" ").collect::<Vec<&str>>();

        // each move line in form of: move X from Y to Z
        let num_to_move: u32 = words_in_line[1].parse().unwrap();
        let from: usize = words_in_line[3].parse().unwrap();
        let to: usize = words_in_line[5].parse().unwrap();

        let mut cr_to_move: Vec<char> = Vec::new();

        for _ in 0..num_to_move {
            cr_to_move.push(stacks[from-1].pop().unwrap());
        }

        cr_to_move.reverse();

        stacks[to-1].extend(cr_to_move);
    }
}

fn print_top_of_each_stack(stacks: Vec<Stack>) {
    print!("Top of each stack: ");

    for stack in stacks {
        print!("{}", stack[stack.len()-1]);
    }

    println!();
}