pub fn solution(input: String) {
    // Part 1
    let most_active_product = get_mul_of_two_most_active(&input, 20, 3);
    println!("Product of two most active monkeys after 20 iters with worry reduction of 3: {most_active_product}");

    // Part 2
    let most_active_product = get_mul_of_two_most_active(&input, 10000, 1);
    println!("Product of two most active monkeys after 10000 iters with worry reduction of 1: {most_active_product}");
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<usize>,
    operation: fn(usize, usize) -> usize,
    operation_arg: String,
    test: usize,
    true_throw: usize,
    false_throw: usize,
    num_inspected: usize
}

impl Monkey {
    fn new() -> Self {
        Self {
            items: Vec::new(),
            operation: |old, arg| old + arg,
            operation_arg: String::new(),
            test: 0,
            true_throw: 0,
            false_throw: 0,
            num_inspected: 0
        }
    }

    fn set_items(&mut self, items: Vec<usize>) {
        self.items = items;
    }

    fn set_operation(&mut self, operation: fn(usize, usize) -> usize, operation_arg: &str) {
        self.operation = operation;
        self.operation_arg = operation_arg.to_string();
    }

    fn set_test(&mut self, test: usize) {
        self.test = test;
    }

    fn set_true_throw(&mut self, true_throw: usize) {
        self.true_throw = true_throw;
    }

    fn set_false_throw(&mut self, false_throw: usize) {
        self.false_throw = false_throw;
    }

    fn add_item(&mut self, item: usize) {
        self.items.push(item);
    }

    // Returns vec of all items that need to passed to another monkey
    // each in the format of (monkey_to, item)
    fn inspect_items(&mut self, worry_reduction: usize, modulo: usize) -> Vec<(usize, usize)> {
        let mut items_to_move = Vec::new();

        for item in self.items.clone() {
            self.num_inspected += 1;
            let mut new = self.items.remove(0).clone();

            if self.operation_arg == "old" {
                new = (self.operation)(item.clone(), item.clone());
            } else {
                new = (self.operation)(item, self.operation_arg.parse::<usize>().unwrap());
            }

            new %= modulo;

            new /= worry_reduction;
            
            if new % self.test == 0 {
                items_to_move.push((self.true_throw, new));
            } else {
                items_to_move.push((self.false_throw, new))
            }
        }

        items_to_move
    }
}

fn get_mul_of_two_most_active(input: &String, num_iters: usize, worry_reduction: usize) -> usize {
    let mut monkeys = get_monkeys(input);
    let modulo = get_modulo(&monkeys);

    // Run for 20 rounds
    for _ in 0..num_iters {
        for i in 0..monkeys.len() {
            let items_to_move = monkeys[i].inspect_items(worry_reduction, modulo);

            for (to, item) in items_to_move {
                monkeys[to].add_item(item);
            }
        }
    }

    monkeys.sort_by(
        |monkey_1: &Monkey, monkey_2: &Monkey| monkey_1.num_inspected.cmp(&monkey_2.num_inspected));
    
    monkeys[monkeys.len()-1].num_inspected * monkeys[monkeys.len()-2].num_inspected
}

fn get_modulo(monkeys: &Vec<Monkey>) -> usize {
    let mut modulo: usize = 1;
    
    monkeys.iter().for_each(|monkey| modulo *= monkey.test);

    modulo
}

fn get_monkeys(input: &String) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();

    let monkey_lines = input.split("\n\n");

    for monkey_line in monkey_lines {
        let mut monkey = Monkey::new();
        for line in monkey_line.lines() {
            if line.contains("items") {
                let str_items = line.split_at(line.find(":").unwrap() + 1).1;
                let items = str_items.trim().split(", ")
                    .map(|item| item.parse::<usize>().unwrap()).collect::<Vec<usize>>();
                monkey.set_items(items);
            } else if line.contains("Operation") {
                let op = line.split_at(line.find(":").unwrap() + 1).1.trim();
                let op_and_arg = determine_operation_fn(op);
                monkey.set_operation(op_and_arg.0, op_and_arg.1);
            } else if line.contains("Test") {
                let split = line.split_whitespace().collect::<Vec<&str>>();
                monkey.set_test(split[3].parse::<usize>().unwrap());
            } else if line.contains("true") {
                let split = line.split_whitespace().collect::<Vec<&str>>();
                monkey.set_true_throw(split[5].parse::<usize>().unwrap());
            } else if line.contains("false") {
                let split = line.split_whitespace().collect::<Vec<&str>>();
                monkey.set_false_throw(split[5].parse::<usize>().unwrap());
            }
        }
        monkeys.push(monkey);
    }

    monkeys
}

fn determine_operation_fn(operation_str: &str) -> (fn(usize, usize) -> usize, &str) {
    let split_op = operation_str.split_whitespace().collect::<Vec<&str>>();

    match split_op[3] {
        "*" => {
            (|old, arg| old * arg, split_op[4])
        },
        "+" => {
            (|old, arg| old + arg, split_op[4])
        },
        _ => panic!("Unsupported operation in {operation_str}")
    }
}

mod tests {
    const INPUT: &str = 
        "Monkey 0:\n\
            Starting items: 79, 98\n\
            Operation: new = old * 19\n\
            Test: divisible by 23\n\
                If true: throw to monkey 2\n\
                If false: throw to monkey 3\n\
        \n\
        Monkey 1:\n\
            Starting items: 54, 65, 75, 74\n\
            Operation: new = old + 6\n\
            Test: divisible by 19\n\
                If true: throw to monkey 2\n\
                If false: throw to monkey 0\n\
        \n\
        Monkey 2:\n\
            Starting items: 79, 60, 97\n\
            Operation: new = old * old\n\
            Test: divisible by 13\n\
                If true: throw to monkey 1\n\
                If false: throw to monkey 3\n\
        \n\
        Monkey 3:\n\
            Starting items: 74\n\
            Operation: new = old + 3\n\
            Test: divisible by 17\n\
                If true: throw to monkey 0\n\
                If false: throw to monkey 1";

    #[test]
    fn get_monkeys_test() {
        let input = String::from(INPUT);
        
        let monkeys = super::get_monkeys(&input);

        assert_eq!(4, monkeys.len());
    }

    #[test]
    fn get_mul_of_two_most_active_20_iters_3_worry_reduction_test() {
        let input = String::from(INPUT);

        assert_eq!(10605, super::get_mul_of_two_most_active(&input, 20, 3));
    }

    #[test]
    fn get_mul_of_two_most_active_10000_iters_no_worry_reduction_test() {
        let input = String::from(INPUT);

        assert_eq!(2713310158, super::get_mul_of_two_most_active(&input, 10000, 1));
    }


}