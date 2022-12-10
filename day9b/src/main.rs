use std::fs;
use std::cmp::Ordering::*;
use std::collections::HashSet;

#[derive(Clone)]
struct Knot {
    pos_x: i32,
    pos_y: i32,
}

impl Knot {
    fn too_far(&self, other: &Knot) -> bool {
        self.pos_x.abs_diff(other.pos_x) > 1 || self.pos_y.abs_diff(other.pos_y) > 1
    }
}

fn follow(leader: &Knot, follower: &Knot) -> (i32, i32) {
    match (leader.pos_x.cmp(&follower.pos_x), leader.pos_y.cmp(&follower.pos_y)) {
        (Less, Less) => (follower.pos_x - 1, follower.pos_y - 1),
        (Less, Equal) => (follower.pos_x - 1, follower.pos_y),
        (Less, Greater) => (follower.pos_x - 1, follower.pos_y + 1),
        (Equal, Less) => (follower.pos_x, follower.pos_y - 1),
        (Equal, Equal) => panic!("HELP"),
        (Equal, Greater) => (follower.pos_x, follower.pos_y + 1),
        (Greater, Less) => (follower.pos_x + 1, follower.pos_y - 1),
        (Greater, Equal) => (follower.pos_x + 1, follower.pos_y),
        (Greater, Greater) => (follower.pos_x + 1, follower.pos_y + 1),
    }
}

fn apply_instruction(knots: &mut Vec::<Knot>, instruction: &char, visited: &mut HashSet::<(i32, i32)>) {
    let (offset_x, offset_y): (i32, i32) = match instruction {
        'U' => (0, -1),
        'R' => (1, 0),
        'D' => (0, 1),
        'L' => (-1, 0),
        _ => panic!("Invalid input"),
    };

    knots[0].pos_x += offset_x;
    knots[0].pos_y += offset_y;

    for n in 1..knots.len() {
        if knots[n-1].too_far(&knots[n]) {
            let (new_x, new_y) = follow(&knots[n-1], &knots[n]);
            knots[n].pos_x = new_x;
            knots[n].pos_y = new_y;

            if n == 9 {
                visited.insert((knots[n].pos_x, knots[n].pos_y));
            }
        }
    }
}

fn apply_instructions(mut knots: &mut Vec::<Knot>, instructions: &Vec::<(char, usize)>) -> usize {
    let mut visited = HashSet::from([(0, 0)]);

    instructions.iter()
        .flat_map(|(direction, steps)| vec![direction; *steps])
        .for_each(
            |direction| { 
            apply_instruction(&mut knots, direction, &mut visited);
        });

    return visited.len();
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

    let mut knots = vec![Knot {pos_x: 0, pos_y: 0}; 10];
    
    let part_two = apply_instructions(&mut knots, &instructions);
    println!("Part two: {}", part_two);
}