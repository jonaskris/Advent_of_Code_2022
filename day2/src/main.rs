use std::fs;

fn score_for_round(their_choice: char, my_choice: char) -> u32 {
    let hand_score = match my_choice {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => panic!("Not a valid choice"),
    };

    let vs_score = match(their_choice, my_choice) {
        ('B', 'X') | ('C', 'Y') | ('A', 'Z') => 0,
        ('A', 'X') | ('B', 'Y') | ('C', 'Z') => 3,
        ('C', 'X') | ('A', 'Y') | ('B', 'Z') => 6,
        _ => panic!("Not a valid choice"),
    };

    return hand_score + vs_score;
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read file");

    let games = input.split("\n").collect::<Vec::<&str>>();
    let choices = games
                    .iter()
                    .map(
                        |game| 
                        {(
                            game.chars().nth(0).expect("Must have first choice!"), 
                            game.chars().nth(2).expect("Must have second choice!")
                        )}
                    ).collect::<Vec::<(char, char)>>();

    let score_part_one = choices.iter().map(
        |(their_choice, my_choice)|
         score_for_round(*their_choice, *my_choice)
        ).sum::<u32>();
    println!("Part one score: {}", score_part_one);
    

    let score_part_two = choices.iter().map(
        |(their_choice, outcome)| 
        {
            let my_choice = match (their_choice, outcome) {
                ('A', 'Y') | ('B', 'X') | ('C', 'Z') => 'X',
                ('A', 'Z') | ('B', 'Y') | ('C', 'X') => 'Y',
                ('A', 'X') | ('B', 'Z') | ('C', 'Y') => 'Z',
                _ => panic!("Not a valid choice"),
            };

            score_for_round(*their_choice, my_choice)
        }).sum::<u32>();
    println!("Part two score: {}", score_part_two);
}