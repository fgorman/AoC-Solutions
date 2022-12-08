pub fn solution(input: String) {
    let tree_grid = parse_tree_grid(&input);

    // Part 1
    let num_visible = get_num_visible_trees(&tree_grid);
    println!("Number of visible trees: {num_visible}");

    // Part 2
    let highest_scenic_score = get_highest_scenic_score(&tree_grid);
    println!("Highest scenic score: {highest_scenic_score}");
}

fn parse_tree_grid(input: &String) -> Vec<Vec<u8>> {
    let mut tree_grid = Vec::new();

    for line in input.lines() {
        let mut tree_line = Vec::new();
        for tree in line.chars() {
            tree_line.push(tree.to_digit(10).unwrap() as u8);
        }
        tree_grid.push(tree_line);
    }

    tree_grid
}
////////////////////////////////////////////////////////////////////
/// Part 1 functions
////////////////////////////////////////////////////////////////////

fn get_num_visible_trees(tree_grid: &Vec<Vec<u8>>) -> usize {
    
    let mut visible = vec![vec![false; tree_grid[0].len()]; tree_grid.len()];

    // Check up and left and outer trees
    for i in 0..tree_grid.len() {
        for j in 0..tree_grid[i].len() {
            // Outer trees
            if i == 0 || j == 0 || i == tree_grid.len() - 1 || j == tree_grid[i].len() - 1 {
                visible[i][j] = true;
            } else if check_trees_up(&tree_grid, i, j) {
                visible[i][j] = true;
            } else if check_trees_down(&tree_grid, i, j) {
                visible[i][j] = true;
            } else if check_trees_left(&tree_grid, i, j) {
                visible[i][j] = true;
            } else if check_trees_right(&tree_grid, i, j) {
                visible[i][j] = true;
            }
        }
    }

    let mut num_visible = 0_usize;

    for i in 0..visible.len() {
        for j in 0..visible[i].len() {
            if visible[i][j] {
                num_visible += 1;
            }
        }
    }

    num_visible
}

fn check_trees_up(tree_grid: &Vec<Vec<u8>>, tree_i: usize, tree_j: usize) -> bool {
    for i in 0..tree_i {
        if tree_grid[i][tree_j] >= tree_grid[tree_i][tree_j] {
            return false;
        }
    }

    true
}

fn check_trees_down(tree_grid: &Vec<Vec<u8>>, tree_i: usize, tree_j: usize) -> bool {
    for i in tree_i+1..tree_grid.len() {
        if tree_grid[i][tree_j] >= tree_grid[tree_i][tree_j] {
            return false;
        }
    }

    true
}

fn check_trees_left(tree_grid: &Vec<Vec<u8>>, tree_i: usize, tree_j: usize) -> bool {
    for j in 0..tree_j {
        if tree_grid[tree_i][j] >= tree_grid[tree_i][tree_j] {
            return false;
        }
    }

    true
}

fn check_trees_right(tree_grid: &Vec<Vec<u8>>, tree_i: usize, tree_j: usize) -> bool {
    for j in tree_j+1..tree_grid[tree_i].len() {
        if tree_grid[tree_i][j] >= tree_grid[tree_i][tree_j] {
            return false;
        }
    }

    true
}


////////////////////////////////////////////////////////////////////
/// Part 2 functions 
////////////////////////////////////////////////////////////////////

fn get_highest_scenic_score(tree_grid: &Vec<Vec<u8>>) -> usize {
    let mut highest_score = 0_usize;

    for i in 0..tree_grid.len() {
        for j in 0..tree_grid[i].len() {
            let up_score = get_score_up(&tree_grid, i, j);
            let down_score = get_score_down(&tree_grid, i, j);
            let left_score = get_score_left(&tree_grid, i, j);
            let right_score = get_score_right(&tree_grid, i, j);
            let total_score = up_score * down_score * left_score * right_score;

            if total_score > highest_score {
                highest_score = total_score
            }
        }
    }

    highest_score
}

fn get_score_up(tree_grid: &Vec<Vec<u8>>, tree_i: usize, tree_j: usize) -> usize {
    let mut trees_viewable = 0_usize;

    for i in (0..tree_i).rev() {
        if tree_grid[i][tree_j] >= tree_grid[tree_i][tree_j] {
            trees_viewable += 1;
            break;
        } else {
            trees_viewable += 1;
        }
    }

    trees_viewable
}

fn get_score_down(tree_grid: &Vec<Vec<u8>>, tree_i: usize, tree_j: usize) -> usize {
    let mut trees_viewable = 0_usize;

    for i in tree_i+1..tree_grid.len() {
        if tree_grid[i][tree_j] >= tree_grid[tree_i][tree_j] {
            trees_viewable += 1;
            break;
        } else {
            trees_viewable += 1;
        }
    }

    trees_viewable
}

fn get_score_left(tree_grid: &Vec<Vec<u8>>, tree_i: usize, tree_j: usize) -> usize {
    let mut trees_viewable = 0_usize;

    for j in (0..tree_j).rev() {
        if tree_grid[tree_i][j] >= tree_grid[tree_i][tree_j] {
            trees_viewable += 1;
            break;
        } else {
            trees_viewable += 1;
        }
    }

    trees_viewable
}

fn get_score_right(tree_grid: &Vec<Vec<u8>>, tree_i: usize, tree_j: usize) -> usize {
    let mut trees_viewable = 0_usize;

    for j in tree_j+1..tree_grid[tree_i].len() {
        if tree_grid[tree_i][j] >= tree_grid[tree_i][tree_j] {
            trees_viewable += 1;
            break;
        } else {
            trees_viewable += 1;
        }
    }

    trees_viewable
}



mod tests {
    #[test]
    fn test_num_visible_trees() {
        let input = String::from("30373\n25512\n65332\n33549\n35390");

        let tree_grid = super::parse_tree_grid(&input);

        assert_eq!(21, super::get_num_visible_trees(&tree_grid));
    }

    #[test]
    fn test_highest_scenic_score() {
        let input = String::from("30373\n25512\n65332\n33549\n35390");

        let tree_grid = super::parse_tree_grid(&input);

        assert_eq!(8, super::get_highest_scenic_score(&tree_grid));
    }
}