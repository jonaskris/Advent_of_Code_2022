use std::fs;
use std::fmt;
use std::collections::VecDeque;

struct Matrix<T> {
    elements: Vec::<T>,
    width: usize,
    height: usize,
}

impl fmt::Display for Matrix<char> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string: String = "".to_owned(); 
        for line_index in 0..self.height {
            string.push_str(&self.elements[line_index*self.width .. (line_index + 1)*self.width].iter().collect::<String>());
            if line_index != self.height {
                string.push('\n')
            }
        }
        write!(f, "{}", string)
    }
}

impl fmt::Display for Matrix<usize> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string: String = "".to_owned(); 
        for y in 0..self.height {
            for x in 0..self.width {
                string.push_str(&self.elements[y * self.width + x].to_string());
                string.push_str(" ");
            }
            string.push('\n');
        }
        write!(f, "{}", string)
    }
}

impl<T> Matrix<T> {
    fn element_at(&self, position: (isize, isize)) -> Option<&T> {
        if position.0 < 0 || position.0 >= self.width as isize || position.1 < 0 || position.1 >= self.height as isize {
            return None;
        }

        return Some(&self.elements[&self.width * position.1 as usize + position.0 as usize]);
    }

    fn set_element_at(&mut self, position: (isize, isize), element: T) {
        self.elements[self.width * position.1 as usize + position.0 as usize] = element;
    }
}

impl Matrix<char> {
    fn from_str(string: &str) -> Self {
        let width = string.find("\r\n").expect("Invalid input");
        let height = string.matches("\r\n").count() + 1;

        Matrix {
            elements: string.chars().filter(|c| c.is_alphabetic()).collect::<Vec::<char>>(),
            width: width,
            height: height,
        }
    }

    fn bfs(&self, start: (isize, isize), visited: &mut Matrix<usize>, reverse: bool) {
        let mut queue: VecDeque<(isize, isize, usize)> = VecDeque::new();
        queue.push_back((start.0, start.1, 1));
        visited.set_element_at(start, 1);

        let directions: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

        while let Some(current) = queue.pop_front() {
            let current_pos = (current.0, current.1);
            let current_val = self.element_at(current_pos).unwrap();

            let neighbors = directions.iter()
                .map(|direction| (current_pos.0 as isize + direction.0, current_pos.1 as isize + direction.1))
                .collect::<Vec::<(isize, isize)>>();

            for neighbor_pos in neighbors {
                let neighbor_visited = visited.element_at(neighbor_pos);
                let neighbor = self.element_at(neighbor_pos);

                if neighbor.is_none() || neighbor_visited.is_none() || *visited.element_at(neighbor_pos).unwrap() != 0 as usize{
                    continue;
                }
                
                let mut diff = (*neighbor.unwrap() as u32) as i32 - (*current_val as u32) as i32; 
                if reverse {
                    diff = (*current_val as u32) as i32 - (*neighbor.unwrap() as u32) as i32;
                }

                if diff <= 1 {
                    queue.push_back((neighbor_pos.0, neighbor_pos.1, current.2 + 1));
                    visited.set_element_at(neighbor_pos, current.2 + 1);
                }
            }
        }

        visited.elements.iter_mut().for_each(|steps| {
            if *steps != 0 {
                *steps -= 1;
            }
        });
    }
}

impl Matrix<usize> {
    fn from(width: usize, height: usize) -> Self {
        Matrix {
            elements: vec![0; width * height],
            width: width,
            height: height,
        }
    }
}

fn main() {
    let input_string = fs::read_to_string("input.txt").expect("Unable to read file");

    let mut matrix = Matrix::<char>::from_str(&input_string);

    let start_pos_1d = matrix.elements.iter().position(|c| *c == 'S').unwrap();
    let start_pos_2d = ((start_pos_1d % matrix.width) as isize, (start_pos_1d / matrix.width) as isize);
    matrix.elements[start_pos_1d] = 'a';
    let end_pos_1d = matrix.elements.iter().position(|c| *c == 'E').unwrap();
    let end_pos_2d = ((end_pos_1d % matrix.width) as isize, (end_pos_1d / matrix.width) as isize);
    matrix.elements[end_pos_1d] = 'z';

    let mut matrix_visited = Matrix::<usize>::from(matrix.width, matrix.height);
    matrix.bfs(start_pos_2d, &mut matrix_visited, false);
    let part_one = matrix_visited.element_at(end_pos_2d).unwrap();
    println!("Part one: {:?}", part_one);

    let mut matrix_visited_part_two = Matrix::<usize>::from(matrix.width, matrix.height);
    matrix.bfs(end_pos_2d, &mut matrix_visited_part_two, true);
    
    let part_two = matrix.elements.clone().iter()
        .enumerate()
        .filter(|(_i, element)| **element == 'a')
        .map(|(i, _element)| ((i % matrix.width) as isize, (i / matrix.width) as isize))
        .map(|position| matrix_visited_part_two.element_at(position).unwrap())
        .filter(|steps| **steps != 0)
        .min().unwrap();

    println!("Part two: {}", part_two);
}