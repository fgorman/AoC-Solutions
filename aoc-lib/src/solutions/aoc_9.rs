use std::collections::HashSet;

pub fn solution(input: String) {
    // Part 1
    let num_visited_len_2 = get_num_visited_by_tail(&input, 2);
    println!("Num positions visited by tail of rope with len 2: {num_visited_len_2}");

    // Part 2
    let num_visited_len_10 = get_num_visited_by_tail(&input, 10); 
    println!("Num positions visited by tail of rope with len 10 {num_visited_len_10}");
}

struct Rope {
    knots: Vec<(isize, isize)>,
    visited: HashSet<(isize, isize)>
}

impl Rope {
    fn new(len: usize) -> Self {
        Self {
            knots: vec![(0,0); len],
            visited: HashSet::new()
        }
    }

    fn move_rope(&mut self, direction: &str, num_moves: usize) {
        for _ in 0..num_moves {
            // Move the head
            match direction {
                "U" => self.knots[0].1 += 1,
                "D" => self.knots[0].1 -= 1,
                "L" => self.knots[0].0 -= 1,
                "R" => self.knots[0].0 += 1,
                _ => panic!("No move found for input: {direction}")
            };
    
            // Process the move for the rest of the rope
            for i in 1..self.knots.len() {
                let curr_knot = self.knots[i];
                let prev_knot = self.knots[i-1];
    
                self.knots[i] = Rope::process_move(prev_knot, curr_knot);
            }

            self.visited.insert(self.knots[self.knots.len()-1]);
        }
    }

    fn is_within_1_pos(prev_knot: (isize, isize), curr_knot: (isize, isize)) -> bool {
        isize::abs(prev_knot.0 - curr_knot.0) <= 1 
            && isize::abs(prev_knot.1 - curr_knot.1) <= 1
    }

    fn process_move(prev_knot: (isize, isize), curr_knot: (isize, isize)) -> (isize, isize) {
        let diff = (
            prev_knot.0 - curr_knot.0,
            prev_knot.1 - curr_knot.1
        );

        if Rope::is_within_1_pos(prev_knot, curr_knot) { return curr_knot; }

        match diff {
            // Knot before moved up
            (0, 2) => (curr_knot.0, curr_knot.1 + 1),
            (1, 2) => (curr_knot.0 + 1, curr_knot.1 + 1),
            (-1, 2) => (curr_knot.0 - 1, curr_knot.1 + 1),
            // Knot before moved down
            (0, -2) => (curr_knot.0, curr_knot.1 - 1),
            (1, -2) => (curr_knot.0 + 1, curr_knot.1 - 1),
            (-1, -2) => (curr_knot.0 - 1, curr_knot.1 - 1),
            // Knot before moved left
            (-2, 0) => (curr_knot.0 - 1, curr_knot.1),
            (-2, 1) => (curr_knot.0 - 1, curr_knot.1 + 1),
            (-2, -1) => (curr_knot.0 - 1, curr_knot.1 - 1),
            // Knot before moved right
            (2, 0) => (curr_knot.0 + 1, curr_knot.1),
            (2, 1) => (curr_knot.0 + 1, curr_knot.1 + 1),
            (2, -1) => (curr_knot.0 + 1, curr_knot.1 - 1),
            // Knot before moved up and right
            (2, 2) => (curr_knot.0 + 1, curr_knot.1 + 1),
            // Knot before move up and left
            (-2, 2) => (curr_knot.0 - 1, curr_knot.1 + 1),
            // Knot before move down and right
            (2, -2) => (curr_knot.0 + 1, curr_knot.1 - 1),
            // Knot before moved down and left
            (-2, -2) => (curr_knot.0 - 1, curr_knot.1 -1),
            // Incompatible move
            _ => panic!("Received incompatiable move with diff: {:?}", diff)
        }
    }
}

fn get_num_visited_by_tail(input: &String, rope_len: usize) -> usize {
    let mut rope = Rope::new(rope_len);

    for line in input.lines() {
        let split_line = line.split_whitespace().collect::<Vec<&str>>();
        let num_times_moved = split_line[1].parse::<usize>().unwrap();

        rope.move_rope(split_line[0], num_times_moved);
    }

    rope.visited.len()
}

mod tests {
    #[test]
    fn test_num_visited_by_tail_with_len_2() {
        let input = String::from("R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2");

        assert_eq!(13, super::get_num_visited_by_tail(&input, 2));
    }

    #[test]
    fn test_num_visited_by_tail_with_len_10_1() {
        let input = String::from("R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2");

        assert_eq!(1, super::get_num_visited_by_tail(&input, 10));
    }

    #[test]
    fn test_num_visited_by_tail_with_len_10_2() {
        let input = String::from("R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20");

        assert_eq!(36, super::get_num_visited_by_tail(&input, 10));
    }
}