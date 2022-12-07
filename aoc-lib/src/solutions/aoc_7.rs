use std::collections::HashMap;

pub fn solution(input: String) {
    let fs = get_file_system(&input);
    
    // Part 1
    let sum_le_100000 = fs.get_sum_le_100000();
    println!("Sum of directories with size <= 100000: {}", sum_le_100000);

    // Part 2
    let size_of_dir_to_delete = fs.get_min_dir_to_delete();
    println!("Size of dir to delete: {size_of_dir_to_delete}");
}

#[derive(Debug)]
struct FileSystem {
    dirs: HashMap<String, usize>,
    children: HashMap<String, Vec<String>>,
    parent: HashMap<String, String>
}

impl FileSystem {
    fn new() -> Self {
        Self {
            dirs: HashMap::new(),
            children: HashMap::new(),
            parent: HashMap::new()
        }
    }

    fn insert_child(&mut self, dir_name: String, child_name: String) {
        self.children.entry(dir_name).or_insert(Vec::new()).push(child_name);
    }

    fn insert_dir(&mut self, dir_name: String, parent: String) {
        self.dirs.insert(dir_name.clone(), 0);
        self.children.insert(dir_name.clone(), Vec::new());
        self.parent.insert(dir_name.clone(), parent);
    }

    fn inc_size(&mut self, dir_name: String, inc_by: usize) {
        *self.dirs.get_mut(&dir_name).unwrap() += inc_by;

        let mut curr_path = dir_name.clone();

        get_path_up(&mut curr_path);

        while curr_path != "/" {
            *self.dirs.get_mut(&curr_path).unwrap() += inc_by;
            get_path_up(&mut curr_path);
        }

        *self.dirs.get_mut(&curr_path).unwrap() += inc_by;
    }

    fn get_sum_le_100000(&self) -> usize {
        let mut sum = 0_usize;

        for (dir, size) in &self.dirs {
            println!("Size for {dir}: {size}");
            if *size <= 100_000 {
                sum += *size;
            } 
        }

        sum
    }

    fn get_min_dir_to_delete(&self) -> usize {
        let root_size = self.dirs.get("/").unwrap();

        let min_size_needed = 30000000_usize;

        let size_left = 70000000 - root_size;

        let mut closest_to_needed = *root_size;

        let mut size_of_dir_to_delete = *root_size;

        for (_, size) in &self.dirs {
            let size_if_deleted =  size_left + *size;
            if !(size_if_deleted >= min_size_needed) {
                continue; 
            }

            let size_above_needed = size_if_deleted - min_size_needed;
            if size_above_needed < closest_to_needed {
                closest_to_needed = size_above_needed;
                size_of_dir_to_delete = *size;
            }
        }

        size_of_dir_to_delete
    }
}

fn get_path_down(curr_path: &mut String, dir_down: String) {
    *curr_path = format!("{curr_path}{dir_down}/")
}

fn get_path_up(curr_path: &mut String) {
    if curr_path == "/" {
        return;
    }

    let first_idx = curr_path.rfind("/").unwrap();
    curr_path.remove(first_idx);
    let idx = curr_path.rfind("/").unwrap();
    let split_path = curr_path.split_at(idx+1);
    *curr_path = split_path.0.to_string();
}

fn get_file_system(input: &String) -> FileSystem {
    let mut fs = FileSystem::new();

    fs.insert_dir("/".to_string(), "".to_string());

    let mut curr_path = "/".to_string();

    for line in input.lines().skip(1) {
        let split_line = line.split_whitespace().collect::<Vec<&str>>();

        if split_line[1] == "cd" {
            if split_line[2] == ".." {
                get_path_up(&mut curr_path);
            } else {
                get_path_down(&mut curr_path, split_line[2].to_string());
            }
            continue;
        } else if split_line[1] == "ls" {
            continue;
        }


        if split_line[0] == "dir" {
            let mut path_to_dir = curr_path.clone();
            get_path_down(&mut path_to_dir, split_line[1].to_string());
            fs.insert_dir(path_to_dir.clone(), curr_path.clone());
            fs.insert_child(curr_path.clone(), path_to_dir.clone());
        } else {
            println!("Increasing size for {curr_path}");
            let inc_by = split_line[0].parse::<usize>().unwrap();
            fs.inc_size(curr_path.clone(), inc_by);
        }
    }

    fs
}