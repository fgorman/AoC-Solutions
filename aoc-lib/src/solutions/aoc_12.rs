use std::{collections::{HashMap, BinaryHeap}, cmp::Ordering};

pub fn solution(input: String) {
    let (mut heightmap, a_locations) = build_heightmap(&input);

    // Part 1
    let shortest_path = get_shortest_path(&heightmap);
    println!("Shortest path: {shortest_path}");
    
    // Part 2
    let shortest_path_for_any_a = get_shortest_path_for_any_a(&mut heightmap, &a_locations);
    println!("Shortest path for any a: {shortest_path_for_any_a}");
}

#[derive(Debug)]
struct HeightMap {
    adj_list: HashMap<usize, Vec<usize>>,
    starting_pos: usize,
    ending_pos: usize
}

impl HeightMap {
    fn new() -> Self {
        Self {
            adj_list: HashMap::new(),
            starting_pos: 0,
            ending_pos: 0
        }
    }

    fn insert_edge(&mut self, from: usize, to: usize) {
        self.adj_list.entry(from).or_insert(Vec::new()).push(to);
    }
}

#[derive(Eq, PartialEq)]
struct Visit{
    pos: usize, 
    pos_dist: usize
}

impl Ord for Visit {
    fn cmp(&self, other: &Self) -> Ordering {
        other.pos_dist.cmp(&self.pos_dist)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for Visit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Just run Djikstra's
fn get_shortest_path(heightmap: &HeightMap) -> usize {
    let mut dist = vec![usize::MAX; heightmap.adj_list.len()];
    let mut queue = BinaryHeap::new();

    dist[heightmap.starting_pos] = 0;
    queue.push(Visit { pos: heightmap.starting_pos, pos_dist: 0 });
        
    while let Some(Visit{ pos, pos_dist}) = queue.pop() {
        if pos == heightmap.ending_pos {
            return pos_dist;
        }

        if pos_dist > dist[pos] {
            continue;
        }

        for next_position in &heightmap.adj_list[&pos] {
            let next = Visit { pos: *next_position, pos_dist: dist[pos] + 1};
            
            if next.pos_dist < dist[next.pos] {
                dist[*next_position] = next.pos_dist;
                queue.push(next);
            }
        }
    }

    dist[heightmap.ending_pos]
}

// Would be much faster if I used Bellman-Ford, using the destination as the source
// but I didn't feel like implementing it
fn get_shortest_path_for_any_a(heightmap: &mut HeightMap, a_locations: &Vec<usize>) -> usize {
    let mut shortest_path = usize::MAX;

    for loc in a_locations {
        heightmap.starting_pos = *loc;

        let path_length = get_shortest_path(heightmap);

        if path_length < shortest_path {
            shortest_path = path_length;
        }
    }

    shortest_path
}

fn build_heightmap(input: &String) -> (HeightMap, Vec<usize>) {
    // First parse the input to a Vec<Vec<char>>
    let mut parsed_map = input.lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    // Build the heightmap
    let mut heightmap = HeightMap::new();
    let mut a_locations = Vec::new();

    let num_rows = parsed_map.len();
    let num_cols = parsed_map[0].len();

    for i in 0..num_rows {
        for j in 0..num_cols {
            if parsed_map[i][j] == 'S' {
                heightmap.starting_pos = i * num_cols + j;
                parsed_map[i][j] = 'a';
            } else if parsed_map[i][j] == 'E' {
                heightmap.ending_pos = i * num_cols + j;
                parsed_map[i][j] = 'z';
            }
        }
    }

    for i in 0..num_rows {
        for j in 0..num_cols {
            let curr_val = parsed_map[i][j];

            let curr_pos = i * num_cols + j;
            let next_val = char::from_u32(curr_val as u32 + 1).unwrap();

            if curr_val == 'a' {
                a_locations.push(curr_pos);
            }

            if i == 0 && j == 0 {
                if parsed_map[i+1][j] <= next_val {
                    heightmap.insert_edge(curr_pos, (i+1)*num_cols + j);
                }
                if parsed_map[i][j+1] <= next_val {
                    heightmap.insert_edge(curr_pos, i*num_cols + j+1);
                }
            } else if i == num_rows - 1 && j == num_cols - 1 {
                if parsed_map[i-1][j] <= next_val {
                    heightmap.insert_edge(curr_pos, (i-1)*num_cols + j);
                }
                if parsed_map[i][j-1] <= next_val {
                    heightmap.insert_edge(curr_pos, i*num_cols + j-1);
                }
            } else if i == 0 && j == num_cols - 1 {
                if parsed_map[i+1][j] <= next_val {
                    heightmap.insert_edge(curr_pos, (i+1)*num_cols + j);
                }
                if parsed_map[i][j-1] <= next_val {
                    heightmap.insert_edge(curr_pos, i*num_cols + j-1);
                }
            } else if i == num_rows - 1 && j == 0 {
                if parsed_map[i-1][j] <= next_val {
                    heightmap.insert_edge(curr_pos, (i-1)*num_cols + j);
                }
                if parsed_map[i][j+1] <= next_val {
                    heightmap.insert_edge(curr_pos, i*num_cols + j+1);
                }
            } else if i == 0 {
                if parsed_map[i+1][j] <= next_val {
                    heightmap.insert_edge(curr_pos, (i+1)*num_cols + j);
                }
                if parsed_map[i][j+1] <= next_val {
                    heightmap.insert_edge(curr_pos, i*num_cols + j+1);
                }
                if parsed_map[i][j-1] <= next_val {
                    heightmap.insert_edge(curr_pos, i*num_cols + j-1);
                }
            } else if j == 0 {
                if parsed_map[i+1][j] <= next_val {
                    heightmap.insert_edge(curr_pos, (i+1)*num_cols + j);
                }
                if parsed_map[i-1][j] <= next_val {
                    heightmap.insert_edge(curr_pos, (i-1)*num_cols + j);
                }
                if parsed_map[i][j+1] <= next_val {
                    heightmap.insert_edge(curr_pos, i*num_cols + j+1);
                }
            } else if i == num_rows - 1 {
                if parsed_map[i-1][j] <= next_val {
                    heightmap.insert_edge(curr_pos, (i-1)*num_cols + j);
                }
                if parsed_map[i][j-1] <= next_val {
                    heightmap.insert_edge(curr_pos, i*num_cols + j-1);
                }
                if parsed_map[i][j+1] <= next_val {
                    heightmap.insert_edge(curr_pos, i*num_cols + j+1);
                }
            } else if j == num_cols - 1 {
                if parsed_map[i+1][j] <= next_val {
                    heightmap.insert_edge(curr_pos, (i+1)*num_cols + j);
                }
                if parsed_map[i-1][j] <= next_val {
                    heightmap.insert_edge(curr_pos, (i-1)*num_cols + j);
                }
                if parsed_map[i][j-1] <= next_val {
                    heightmap.insert_edge(curr_pos, i*num_cols + j-1);
                }
            } else {
                if parsed_map[i+1][j] <= next_val {
                    heightmap.insert_edge(curr_pos, (i+1)*num_cols + j);
                }
                if parsed_map[i-1][j] <= next_val {
                    heightmap.insert_edge(curr_pos, (i-1)*num_cols + j);
                }
                if parsed_map[i][j+1] <= next_val {
                    heightmap.insert_edge(curr_pos, i*num_cols + j+1);
                }
                if parsed_map[i][j-1] <= next_val {
                    heightmap.insert_edge(curr_pos, i*num_cols + j-1);
                }
            }
        }
    }

    (heightmap, a_locations)
}

mod tests {
    #[test]
    fn build_heightmap_test() {
        let input = String::from("Sabqponm\n\
            abcryxxl\n\
            accszExk\n\
            acctuvwj\n\
            abdefghi");

        let (heightmap, _) = super::build_heightmap(&input);

        assert_eq!(0, heightmap.starting_pos);
        assert_eq!(21, heightmap.ending_pos);
        assert_eq!(40, heightmap.adj_list.len());
    }

    #[test]
    fn get_shortest_path_test() {
        let input = String::from("Sabqponm\n\
            abcryxxl\n\
            accszExk\n\
            acctuvwj\n\
            abdefghi");

        let (heightmap, _) = super::build_heightmap(&input);

        assert_eq!(31, super::get_shortest_path(&heightmap));
    }

    #[test]
    fn get_shortest_path_for_any_a_test() {
        let input = String::from("Sabqponm\n\
            abcryxxl\n\
            accszExk\n\
            acctuvwj\n\
            abdefghi");

        let (mut heightmap, a_locations) = super::build_heightmap(&input);

        assert_eq!(29, super::get_shortest_path_for_any_a(&mut heightmap, &a_locations));
    }
}