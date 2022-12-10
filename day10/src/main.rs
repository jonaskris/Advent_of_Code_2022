use std::fs;

fn calc_signal_strength_sum(instructions: &Vec::<isize>, cycles: &[isize]) -> isize {
    let mut x = 1;
    let mut sum = 0;
    instructions.iter()
        .enumerate()
        .for_each(
            |(i, instruction)| {
                if cycles.contains(&(i as isize + 1)) {
                    sum += (i as isize + 1) * x;
                }
                x += instruction;
            }
        );

    return sum;    
}

fn print_screen(screen: &Vec::<char>, width: usize, height: usize) {
    for y in 0..height {
        for x in 0..width {
            print!("{}", screen[y * width + x]);
        }
        println!();
    }
}

fn draw_screen(instructions: &Vec::<isize>, width: usize, height: usize) -> Vec::<char> {
    let mut screen = vec!['.'; width * height];
    let mut x = 1;

    instructions.iter()
        .enumerate()
        .for_each(
            |(i, instruction)| {
                if ((i % 40) as isize).abs_diff(x) <= 1 {
                    screen[i] = '#';
                }
                x += instruction;
            }
        );

    return screen;
}

fn main() {
    let input_string = fs::read_to_string("input.txt").expect("Unable to read file");

    let instructions = input_string.split("\n").flat_map(|line| {
        let mut split_instruction = line.split(' ');
        let (instruction, num) = (split_instruction.next().unwrap(), split_instruction.next());
        let mut cycle_instructions = vec![];
        
        match instruction.trim() {
            "noop" => cycle_instructions.push(0),
            "addx" => {
                cycle_instructions.push(0);
                cycle_instructions.push(num.unwrap().trim().parse::<isize>().expect("Invalid input"));
            },
            _ => panic!("Invalid input"),
        }

        return cycle_instructions;
    })
    .collect::<Vec::<isize>>();
    
    let cycles = [20, 60, 100, 140, 180, 220];
    println!("Part one: {}", calc_signal_strength_sum(&instructions, &cycles));
    


    let screen = draw_screen(&instructions, 40, 6);
    println!("Part two:");
    print_screen(&screen, 40, 6);
}



