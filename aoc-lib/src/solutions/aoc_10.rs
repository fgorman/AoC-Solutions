use std::collections::{HashSet, HashMap};

pub fn solution(input: String) {
    let (num_cycles, cycle_addends) = get_cycle_addends_and_num_cycles(&input);

    // Part 1
    let cycles_p1: HashSet<usize> = HashSet::from([20, 60, 100, 140, 180, 220]);
    let signal_strengths_sum = get_signal_strength_sum(num_cycles, &cycle_addends, &cycles_p1);
    println!("Sum of signal strengths at cycles {:?}: {signal_strengths_sum}", cycles_p1);

    // Part 2
    let crt_pixels = process_pixel_prints(num_cycles, &cycle_addends);
    println!("CRT:");
    print_crt(&crt_pixels);
}

fn get_cycle_addends_and_num_cycles(input: &String) -> (usize, HashMap<usize, isize>) {
    let mut cycle_addends = HashMap::new();
    let mut num_cycles = 0_usize;

    for line in input.lines() {
        let split_line = line.split_whitespace().collect::<Vec<&str>>();

        if split_line[0] == "noop" {
            num_cycles += 1;
        } else {
            num_cycles += 2;
            let addend = split_line[1].parse::<isize>().unwrap();
            cycle_addends.insert(num_cycles, addend);
        }
    }

    (num_cycles, cycle_addends)
}

fn get_signal_strength_sum(num_cycles: usize, cycle_addends: &HashMap<usize, isize>, cycles: &HashSet<usize>) -> isize {
    let mut sum = 0_isize;
    let mut x_register = 1_isize;

    for cycle in 1..=num_cycles {
        if cycles.contains(&cycle) {
            sum += x_register * cycle as isize;
        }

        if cycle_addends.contains_key(&cycle) {
            x_register += cycle_addends.get(&cycle).unwrap();
        }
    }

    sum
}

fn process_pixel_prints(num_cycles: usize, cycle_addends: &HashMap<usize, isize>) -> Vec<Vec<char>> {
    let mut pixels = vec![vec!['.'; 40]; 6];
    let mut x_register = 1_isize;
    let mut line_num = 0_usize;

    for cycle in 0..num_cycles {
        if cycle % 40 == 0 && cycle > 0 {
            line_num += 1;
        }

        let line_pixel = cycle % 40;
        if (x_register-1..=x_register+1).contains(&(line_pixel as isize)) {
            pixels[line_num][line_pixel] = '#';
        }

        if cycle_addends.contains_key(&(cycle + 1)) {
            x_register += cycle_addends.get(&(cycle + 1)).unwrap();
        }
    }

    pixels
}

fn print_crt(pixels: &Vec<Vec<char>>) {
    for line in pixels {
        for pixel in line {
            print!("{}", pixel);
        }
        println!();
    }

    println!();
}

mod tests {
    use std::collections::HashSet;

    const INPUT: &str = "addx 15
    addx -11
    addx 6
    addx -3
    addx 5
    addx -1
    addx -8
    addx 13
    addx 4
    noop
    addx -1
    addx 5
    addx -1
    addx 5
    addx -1
    addx 5
    addx -1
    addx 5
    addx -1
    addx -35
    addx 1
    addx 24
    addx -19
    addx 1
    addx 16
    addx -11
    noop
    noop
    addx 21
    addx -15
    noop
    noop
    addx -3
    addx 9
    addx 1
    addx -3
    addx 8
    addx 1
    addx 5
    noop
    noop
    noop
    noop
    noop
    addx -36
    noop
    addx 1
    addx 7
    noop
    noop
    noop
    addx 2
    addx 6
    noop
    noop
    noop
    noop
    noop
    addx 1
    noop
    noop
    addx 7
    addx 1
    noop
    addx -13
    addx 13
    addx 7
    noop
    addx 1
    addx -33
    noop
    noop
    noop
    addx 2
    noop
    noop
    noop
    addx 8
    noop
    addx -1
    addx 2
    addx 1
    noop
    addx 17
    addx -9
    addx 1
    addx 1
    addx -3
    addx 11
    noop
    noop
    addx 1
    noop
    addx 1
    noop
    noop
    addx -13
    addx -19
    addx 1
    addx 3
    addx 26
    addx -30
    addx 12
    addx -1
    addx 3
    addx 1
    noop
    noop
    noop
    addx -9
    addx 18
    addx 1
    addx 2
    noop
    noop
    addx 9
    noop
    noop
    noop
    addx -1
    addx 2
    addx -37
    addx 1
    addx 3
    noop
    addx 15
    addx -21
    addx 22
    addx -6
    addx 1
    noop
    addx 2
    addx 1
    noop
    addx -10
    noop
    noop
    addx 20
    addx 1
    addx 2
    addx 2
    addx -6
    addx -11
    noop
    noop
    noop";

    #[test]
    fn get_signal_strengths_test() {
        let input = INPUT.to_string();
        let (num_cycles, cycle_addends) = super::get_cycle_addends_and_num_cycles(&input);
        let cycles: HashSet<usize> = HashSet::from([20, 60, 100, 140, 180, 220]);
        assert_eq!(13140, super::get_signal_strength_sum(num_cycles, &cycle_addends, &cycles));
    }

    #[test]
    fn process_pixel_prints_test() {
        let input = INPUT.to_string();
        let (num_cycles, cycle_addends) = super::get_cycle_addends_and_num_cycles(&input);
        let expected_output: Vec<Vec<char>> = Vec::from([
            "##..##..##..##..##..##..##..##..##..##..".chars().collect(),
            "###...###...###...###...###...###...###.".chars().collect(),
            "####....####....####....####....####....".chars().collect(),
            "#####.....#####.....#####.....#####.....".chars().collect(),
            "######......######......######......####".chars().collect(),
            "#######.......#######.......#######.....".chars().collect(),
        ]);
        assert_eq!(expected_output, super::process_pixel_prints(num_cycles, &cycle_addends));
    }
}