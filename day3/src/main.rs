use std::fs;
use std::collections::HashSet;

fn char_to_uint(character: char) -> u32 {
    match character {
        'a'..='z' => character.clone() as u32 - 96,
        'A'..='Z' => character.clone() as u32 - 64 + 26,
        _ => panic!("Invalid item"),    
    }
}

fn find_common_item(groups: &[&str]) -> char {
    let mut hash_set: HashSet::<char> = HashSet::from_iter(('a'..='z').into_iter().chain(('A'..='Z').into_iter()).collect::<Vec::<char>>());

    for group in groups.iter() {
        let other_hash_set: HashSet::<char> = HashSet::from_iter(group.chars());
        hash_set = hash_set.intersection(&other_hash_set).cloned().collect::<HashSet::<char>>();
    }

    if hash_set.len() != 1 {
        panic!("Invalid input: no common items found, or more than one found");
    }

    return hash_set.iter().next().cloned().unwrap();
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read file");

    let rucksacks = input
                    .split("\n")
                    .map(|line| line.trim())
                    .collect::<Vec::<&str>>();

    let mut part_one_sum: u32 = 0;
    for rucksack in &rucksacks {
        let (left, right) = rucksack.split_at(rucksack.len()/2);

        let common_item = find_common_item(&[&left, &right]);
        part_one_sum += char_to_uint(common_item);
    }                
    println!("Part one sum: {:?}", part_one_sum);

    let mut part_two_sum: u32 = 0;
    for three_rucksacks in rucksacks.chunks(3) {
        let common_item = find_common_item(&three_rucksacks);
        part_two_sum += char_to_uint(common_item);
    }   
    println!("Part two sum: {:?}", part_two_sum);
}