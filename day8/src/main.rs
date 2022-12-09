use std::fs;

fn get_visible(vector: & Vec::<char>, width: usize, height: usize, x: usize, y: usize, include_last_obscuring_tree: bool) -> (usize, usize, usize, usize) {
    let self_height = vector[y * width + x];
    
    let mut visible_top: usize = 0;
    if y != 0 {
        for top_y in (0..y).rev() {
            if vector[top_y * width + x] >= self_height {
                if include_last_obscuring_tree {
                    visible_top += 1;
                }
                break;
            } else {
                visible_top += 1;
            }
        }
    }

    let mut visible_left: usize = 0;
    if x != 0 {
        for left_x in (0..x).rev() {
            if vector[y * width + left_x] >= self_height {
                if include_last_obscuring_tree {
                    visible_left += 1;
                }
                break;
            } else {
                visible_left += 1;
            }
        }
    }

    let mut visible_right: usize = 0;
    for right_x in (x+1)..width {
        if vector[y * width + right_x] >= self_height {
            if include_last_obscuring_tree {
                visible_right += 1;
            }
            break;
        } else {
            visible_right += 1;
        }
    }

    let mut visible_bottom: usize = 0;
    for bottom_y in (y+1)..height {
        if vector[bottom_y * width + x] >= self_height {
            if include_last_obscuring_tree {
                visible_bottom += 1;
            }
            break;
        } else {
            visible_bottom += 1;
        }
    }
    
    return (visible_top, visible_right, visible_bottom, visible_left);
}

fn is_visible(visible_directions: (usize, usize, usize, usize), width: usize, height: usize, x: usize, y: usize) -> bool {
    let (visible_top, visible_right, visible_bottom, visible_left) = visible_directions;
    return (visible_right == width - x - 1) || (visible_left == x) || (visible_bottom == height - y - 1) || (visible_top == y);
}

fn main() {
    let input_string = fs::read_to_string("input.txt").expect("Unable to read file");

    let input = input_string.split("\n").map(|line| line.trim()).collect::<Vec::<&str>>();
    let (width, height) = (input[0].len(), input.len());

    let input_char_vec = input.join("").chars().collect::<Vec::<char>>();

    let count = (0..input_char_vec.len())
            .filter(
                |i| {
                    let x = i % width;
                    let y = i / width;

                    is_visible(get_visible(&input_char_vec, width, height, x, y, false), width, height, x, y)
                }
            ).count();
    println!("Part one: {}", count);


    let max_scenic_score = (0..input_char_vec.len())
        .map(
            |i| {
                let x = i % width;
                let y = i / width;

                let (top, right, bottom, left) = get_visible(&input_char_vec, width, height, x, y, true);
                return top * right * bottom * left;
            }
        ).max().unwrap();
    println!("Part two: {}", max_scenic_score);
}
