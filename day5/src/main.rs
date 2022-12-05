use std::fs;

#[derive(Default, Clone)]
struct Column {
    crates: Vec::<char>,
}

#[derive(Clone)]
struct Columns {
    columns: Vec::<Column>,
}

impl Columns {
    fn from_str(text: &str) -> Self {
        let mut newColumns = vec![Column { crates: vec![]}; text.find("\n").unwrap() / 4 ];

        for line in text.split("\n") {
            for (i, character) in line.chars().skip(1).step_by(4).enumerate().filter(|(i, character)| *character != ' ') {
                if character.is_numeric() {
                    continue;
                }
                newColumns[i].crates.push(character);
            }
        }

        Columns {
            columns: newColumns,
        }
    }

    fn apply_instruction(&mut self, instruction: &Instruction) {
        let mut tmp = self.columns[instruction.from].crates.drain( ..instruction.count ).rev().collect::<Vec::<char>>();
        tmp.append(&mut self.columns[instruction.to].crates);
        self.columns[instruction.to].crates = tmp;
    }

    fn apply_instruction_can_move_multiple(&mut self, instruction: &Instruction) {
        let mut tmp = self.columns[instruction.from].crates.drain( ..instruction.count ).collect::<Vec::<char>>();
        tmp.append(&mut self.columns[instruction.to].crates);
        self.columns[instruction.to].crates = tmp;
    }
}

struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

impl Instruction {
    fn from_line(line: &str) -> Self {
        let tokens: Vec::<&str> = line.split(" ").collect();

        Instruction {
            count: tokens[1].trim().parse::<usize>().unwrap(),
            from: tokens[3].trim().parse::<usize>().unwrap() - 1,
            to: tokens[5].trim().parse::<usize>().unwrap() - 1,
        }
    }
}


fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read file");

    let (crates_string, instructions_string) = input.split_once("\r\n\r\n").unwrap();

    let mut columns_part_one = Columns::from_str(crates_string);
    let mut columns_part_two = columns_part_one.clone();

    let mut instructions: Vec::<Instruction> = instructions_string.split("\n").map(Instruction::from_line).collect();

    for instruction in &instructions {
        columns_part_one.apply_instruction(&instruction);
    }
    let part_one = columns_part_one.columns.iter().map(|column| column.crates[0]).collect::<String>();
    println!("Part one: {}", part_one);

    for instruction in &instructions {
        columns_part_two.apply_instruction_can_move_multiple(&instruction);
    }
    let part_two = columns_part_two.columns.iter().map(|column| column.crates[0]).collect::<String>();
    println!("Part two: {}", part_two);
}