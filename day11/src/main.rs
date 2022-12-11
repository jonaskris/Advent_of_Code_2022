use std::fs;

#[derive(Clone)]
struct Monkey {
    inspect_count: usize,
    items: Vec::<usize>,
    operation: String,
    test_divisor: usize,
    conditional_throw_to: (usize, usize),
}

impl Monkey {
    fn nums_from_str(string: &str) -> Vec::<usize> {
        let filtered_string = string.trim()
            .chars()
            .filter(|c| c.is_digit(10) || *c == ',')
            .collect::<String>();
        
        return filtered_string.split(',')
            .map(|string| string.parse::<usize>().expect("Invalid input"))
            .collect::<Vec::<usize>>();
    }

    fn num_from_string(string: &str) -> usize {
        let index = string.chars().position(|c| c.is_digit(10)).unwrap();
        return string[index..].trim().parse::<usize>().expect("Invalid input"); 
    }

    fn operation_from_str(string: &str) -> &str {
        let index = string.chars().position(|c| c == '+' || c == '*').unwrap();
        return &string[index..].trim(); 
    }

    fn from_str(string: &str) -> Self {
        let mut lines = string.split("\n").skip(1);
        
        return Monkey {
            inspect_count: 0,
            items: Monkey::nums_from_str(lines.next().expect("Invalid input")),
            operation: Monkey::operation_from_str(lines.next().expect("Invalid input")).to_owned(),
            test_divisor: Monkey::num_from_string(lines.next().expect("Invalid input")),
            conditional_throw_to: (
                Monkey::num_from_string(lines.next().expect("Invalid input")), 
                Monkey::num_from_string(lines.next().expect("Invalid input")), 
            ),
        }
    }

    fn inspect_item(&mut self) {
        self.inspect_count += 1;
        let item = &mut self.items[0];

        let (operation, num_str) = self.operation.trim().split_once(' ').expect("Invalid input");
        if num_str == "old" {
            match operation.chars().nth(0).expect("Invalid input") {
                '*' => *item = *item * *item,
                '+' => *item = *item + *item,
                _ => panic!("Operation '{}' not implemented!", operation),
            }
        } else {
            let num = num_str.trim().parse::<usize>().expect("Invalid input");
            match operation.chars().nth(0).expect("Invalid input") {
                '*' => *item = *item * num,
                '+' => *item = *item + num,
                _ => panic!("Operation '{}' not implemented!", operation),
            }
        }
    }
}

fn play_keep_away(monkeys: &mut Vec::<Monkey>, rounds: usize, reduce_worriness: bool) {
    let max_significant_worriness: usize = monkeys.iter().map(|monkey| monkey.test_divisor).product::<usize>();

    for _ in 0..rounds {
        for monkey_index in 0..monkeys.len() {
            while monkeys[monkey_index].items.len() > 0 {
                // Inspect
                monkeys[monkey_index].inspect_item();

                if reduce_worriness {
                    monkeys[monkey_index].items[0] = monkeys[monkey_index].items[0] / 3;
                } else {
                    monkeys[monkey_index].items[0] = monkeys[monkey_index].items[0] % max_significant_worriness;
                }

                // Throw
                let (if_true, if_false) = monkeys[monkey_index].conditional_throw_to;
                if monkeys[monkey_index].items[0] % monkeys[monkey_index].test_divisor == 0 {
                    let temp = monkeys[monkey_index].items.remove(0);
                    monkeys[if_true].items.push(temp);
                } else {
                    let temp = monkeys[monkey_index].items.remove(0);
                    monkeys[if_false].items.push(temp);
                }
            }
        }
    }
}

fn main() {
    let input_string = fs::read_to_string("input.txt").expect("Unable to read file");

    let mut monkeys = input_string.split("\n\r")
        .map(|line| Monkey::from_str(line.trim()))
        .collect::<Vec::<Monkey>>();
    let mut monkeys_part_two = monkeys.clone();

    // Part one
    play_keep_away(&mut monkeys, 20, true);

    let mut inspect_count = monkeys.iter().map(|monkey| monkey.inspect_count).collect::<Vec::<usize>>();
    inspect_count.sort_by(|a, b| b.cmp(a));

    println!("Part one: {}", inspect_count[0] * inspect_count[1]);

    // Part two
    play_keep_away(&mut monkeys_part_two, 10000, false);

    let mut inspect_count = monkeys_part_two.iter().map(|monkey| monkey.inspect_count).collect::<Vec::<usize>>();
    inspect_count.sort_by(|a, b| b.cmp(a));

    println!("Part two: {}", inspect_count[0] * inspect_count[1]);
}