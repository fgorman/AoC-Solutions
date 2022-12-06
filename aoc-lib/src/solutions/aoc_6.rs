pub fn solution(input: String) {
    // Part 1, num characters in starting message 4
    let num_characters_read_size_4 = get_num_characters_read(&input, 4);
    println!("Number of characters read before a size 4 start is found: {}", num_characters_read_size_4);

    // Part 2, num characters in starting message 14
    let num_characters_read_size_14 = get_num_characters_read(&input, 14);
    println!("Number of characters read before a size 14 start is found: {}", num_characters_read_size_14);
}

fn get_num_characters_read(input: &String, start_packet_msg: usize) -> usize {
    let mut curr_window: Vec<char> = Vec::new();

    let mut start_of_window = 0_usize;
    let mut end_of_window = start_packet_msg;

    let chars: Vec<char> = input.chars().collect::<Vec<char>>();

    curr_window.extend_from_slice(&chars[start_of_window..end_of_window]);

    while !is_starting_sequence(&curr_window) {
        start_of_window += 1;
        end_of_window += 1;
        curr_window.clear();
        curr_window.extend_from_slice(&chars[start_of_window..end_of_window]);
    }

    end_of_window
}

// This is ok since we know the sequence is going to be of constant length, so this will be O(1)
// For part 1 that sequence is of length 4
// For part 2 that sequence is of length 14
fn is_starting_sequence(chars: &Vec<char>) -> bool {
    for i in 0..chars.len() {
        for j in 0..chars.len() {
            if i == j {
                continue;
            }
            if chars[i] == chars[j] {
                return false;
            }
        }
    }
    return true;
}