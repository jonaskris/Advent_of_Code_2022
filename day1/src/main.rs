use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    let mut elf_calories = input
        .split("\r\n\r\n")
        .map(
            |elf_items| elf_items
            .split("\n")
            .map(
                |item| item
                .trim()
                .parse::<i32>().unwrap()
            ).sum::<i32>()
        ).collect::<Vec::<i32>>();

    elf_calories.sort_by(|a, b| b.cmp(a));
    

    let max = elf_calories.iter().max().unwrap();
    let max_three = &elf_calories[..3].iter().sum::<i32>();

    println!("Elf with max calories: {}", max);
    println!("Sum of 3 fattest elves: {}", max_three);
}