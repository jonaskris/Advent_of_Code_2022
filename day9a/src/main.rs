use std::fs;
use std::cmp;

fn print_area(area: &Vec::<char>) {
    for y in 0..3 {
        for x in 0..3 {
            print!("{} ", area[y * 3 + x]);
        }
        println!();
    }
    println!();
}

fn print_visited(visited: &Vec::<Vec::<bool>>) {
    for y in 0..visited.len() {
        for x in 0..visited[0].len() {
            if visited[y][x] {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn apply_instruction<'a>(area: &'a mut Vec::<char>, instruction: &char) -> (i32, i32) {
    let tail_position = area.iter().position(|direction| *direction == 'X').or(area.iter().position(|direction| *direction == 'T')).unwrap();
    let (mut tail_x, mut tail_y) = ((tail_position % 3) as i32, (tail_position / 3) as i32);

    match instruction {
        'U' => {
            if tail_y == 2 && (tail_x == 0 || tail_x == 2) {
                tail_x = 1;
            }
            tail_y = cmp::min(2, tail_y + 1);
        },
        'R' => {
            if tail_x == 0 && (tail_y == 0 || tail_y == 2) {
                tail_y = 1;
            }
            tail_x = cmp::max(0, tail_x - 1);
        },
        'D' => {
            if tail_y == 0 && (tail_x == 0 || tail_x == 2) {
                tail_x = 1;
            }
            tail_y = cmp::max(0, tail_y - 1);
        },
        'L' => {
            if tail_x == 2 && (tail_y == 0 || tail_y == 2) {
                tail_y = 1;
            }
            tail_x = cmp::min(2, tail_x + 1);
        },
        _ => panic!("Invalid input"),
    }

    *area = vec!['.'; 3 * 3];
    if tail_x == 1 && tail_y == 1 {
        area[4] = 'X';
    } else {
        area[4] = 'H';
        area[(tail_y * 3) as usize + tail_x as usize] = 'T';
    }

    match instruction {
        'U' => return (0, -1),
        'R' => return (1, 0),
        'D' => return (0, 1),
        'L' => return (-1, 0),
        _ => panic!("Invalid input"),
    }
}

fn set_visited(area: &Vec::<char>, mut visited: &mut Vec::<Vec::<bool>>, offset_x: &mut i32, offset_y: &mut i32) {
    let tail_position = area.iter().position(|direction| *direction == 'X').or(area.iter().position(|direction| *direction == 'T')).unwrap();
    let (mut tail_x, mut tail_y) = ((tail_position % 3) as i32 - 1 + *offset_x, (tail_position / 3) as i32 - 1 + *offset_y);


    if tail_y < 0 {
        visited.insert(0, vec![false; visited[0].len()]);
        *offset_y += 1;
        tail_y += 1;
    } else if tail_y >= visited.len() as i32 {
        visited.push(vec![false; visited[0].len()]);
    }

    if tail_x < 0 {
        visited.iter_mut().for_each(|vector| vector.insert(0, false));
        *offset_x += 1;
        tail_x += 1;
    } else if tail_x >= visited[0].len() as i32 {
        visited.iter_mut().for_each(|vector| vector.push(false));
    }

    visited[tail_y as usize][tail_x as usize] = true;
}

fn apply_instructions(mut area: &mut Vec::<char>, visited: &mut Vec::<Vec::<bool>>, instructions: &Vec::<(char, usize)>) {
    let (mut offset_x, mut offset_y) = (0, 0);
    instructions.iter()
        .flat_map(|(direction, steps)| vec![direction; *steps])
        .for_each(
            |direction| { 
            let (instruction_offset_x, instruction_offset_y) = apply_instruction(&mut area, direction);
            offset_x += instruction_offset_x;
            offset_y += instruction_offset_y;
            set_visited(&area, visited, &mut offset_x, &mut offset_y);
        });
}

fn main() {
    let input_string = fs::read_to_string("input.txt").expect("Unable to read file");

    let instructions = input_string.split("\n")
        .map(
            |line| {
                let (direction, steps) = line.trim().split_once(' ').expect("Invalid input");
                (direction.chars().nth(0).expect("Invalid input"), steps.parse::<usize>().expect("Invalid input"))
            })
        .collect::<Vec::<(char, usize)>>();

    let mut area = vec!['.'; 3 * 3];
    area[4] = 'X';

    let mut visited: Vec::<Vec::<bool>> = vec![vec![true; 1]];

    apply_instructions(&mut area, &mut visited, &instructions);                  
    //print_visited(&visited);

    let part_one = visited.iter().flatten().filter(|visited| **visited).count();
    println!("Part one: {}", part_one);
}