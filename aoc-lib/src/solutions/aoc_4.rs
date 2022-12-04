pub fn solution(input: String) {
    // Part 1
    let count_of_complete_overlap = get_count_of_complete_overlap(&input);
    println!("Number of pairs that completely overlap: {}", count_of_complete_overlap);

    // Part 2
    let count_of_any_overlap = get_count_of_any_overlap(&input);
    println!("Number of pairs that have any overlap: {}", count_of_any_overlap);
}

fn get_count_of_complete_overlap(input: &String) -> u32 {
    let mut count = 0_u32;

    for line in input.lines() {
        let ranges = line.split(",").collect::<Vec<&str>>();
        let elf_1 = ranges[0].split("-").collect::<Vec<&str>>();
        let elf_2 = ranges[1].split("-").collect::<Vec<&str>>();
        let range_1 = (
            elf_1[0].parse::<u32>().expect("can't parse to u32"),
            elf_1[1].parse::<u32>().expect("can't parse to u32")
        );
        let range_2 = (
            elf_2[0].parse::<u32>().expect("can't parse to u32"),
            elf_2[1].parse::<u32>().expect("can't parse to u32")
        );

        if range_1.0 >= range_2.0 && range_1.1 <= range_2.1 {
            count +=1;
        } else if range_2.0 >= range_1.0 && range_2.1 <= range_1.1 {
            count += 1;
        }
    }

    count
}

fn get_count_of_any_overlap(input: &String) -> u32 {
    let mut count = 0_u32;

    for line in input.lines() {
        let ranges = line.split(",").collect::<Vec<&str>>();
        let elf_1 = ranges[0].split("-").collect::<Vec<&str>>();
        let elf_2 = ranges[1].split("-").collect::<Vec<&str>>();
        let range_1 = (
            elf_1[0].parse::<u32>().expect("can't parse to u32"),
            elf_1[1].parse::<u32>().expect("can't parse to u32")
        );
        let range_2 = (
            elf_2[0].parse::<u32>().expect("can't parse to u32"),
            elf_2[1].parse::<u32>().expect("can't parse to u32")
        );

        if range_1.0 >= range_2.0 && range_1.1 <= range_2.1 {
            count +=1;
        } else if range_2.0 >= range_1.0 && range_2.1 <= range_1.1 {
            count += 1;
        } else if range_1.1 >= range_2.0 && range_1.0 <= range_2.0 {
            count += 1;
        } else if range_2.1 >= range_1.0 && range_2.0 <= range_1.0 {
            count += 1;
        }
    }

    count
}