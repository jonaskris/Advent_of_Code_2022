use std::fs;

fn has_duplicates<'a>(elements: &[char]) -> Option<usize> {
    return elements.iter().enumerate().find_map(|(i, e)| {
        if elements.iter().skip(i + 1).find(|f| f == &e).is_some() {
            return Some(i);
        } else {
            return None
        }
    })
}

fn main() {
    let input_string = fs::read_to_string("input.txt").expect("Unable to read file");
    let input = input_string.chars().collect::<Vec::<char>>();

    let part_one = input
                        .windows(4)
                        .enumerate()
                        .find_map(|(i, window)| has_duplicates(window).xor(Some(i + 4)));
    println!("Part one: {}", part_one.unwrap());


    let part_two = input
                        .windows(14)
                        .enumerate()
                        .find_map(|(i, window)| has_duplicates(window).xor(Some(i + 14)));
    println!("Part two: {}", part_two.unwrap());
}