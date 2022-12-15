use std::fs;
use std::iter::Peekable;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    Integer(u32),
    List(Vec<Packet>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::Integer(left), Packet::Integer(right)) => left.cmp(right),
            (Packet::List(left), Packet::List(right)) => left.cmp(right),
            (Packet::Integer(_left), Packet::List(_right)) => {
                Packet::List(vec![self.clone()]).cmp(&other.clone())
            }
            (Packet::List(_left), Packet::Integer(_right)) => {
                self.clone().cmp(&Packet::List(vec![other.clone()]))
            }
        }
    }
}

fn parse_packet(packet_iter: &mut Peekable<impl Iterator<Item = char>>) -> Packet {
    let mut vector: Vec<Packet> = Vec::<Packet>::new();

    while let Some(character) = packet_iter.next() {
        match character {
            '[' => vector.push(parse_packet(packet_iter)),
            '0'..='9' => {
                let mut num_str = "".to_owned();
                num_str.push(character);

                while let Some(potential_num) = packet_iter.peek() {
                    if potential_num.is_digit(10) {
                        num_str.push(packet_iter.next().unwrap())
                    } else {
                        break;
                    }
                }
                vector.push(Packet::Integer(num_str.parse::<u32>().unwrap()));
            }
            ']' => return Packet::List(vector),
            ',' => {}
            _ => panic!("HELP {}", character),
        }
    }

    match &vector[0] {
        Packet::Integer(_num) => panic!("Invalid input"),
        Packet::List(_list) => return vector[0].clone(),
    }
}

fn main() {
    let input_string = fs::read_to_string("input.txt").expect("Unable to read file");

    let mut packets: Vec<Packet> = input_string
        .split("\r\n")
        .filter_map(|line| {
            if line.is_empty() {
                return None;
            } else {
                return Some(parse_packet(&mut line.chars().peekable()));
            }
        })
        .collect();

    let part_one = packets
        .chunks_exact(2)
        .enumerate()
        .filter(|(_i, packets)| packets[0] <= packets[1])
        .map(|(i, _)| i + 1)
        .sum::<usize>();
    println!("Part one: {}", part_one);

    let mut divider_packets = vec![
        parse_packet(&mut "[[2]]".chars().peekable()),
        parse_packet(&mut "[[6]]".chars().peekable()),
    ];
    packets.append(&mut divider_packets.clone());
    packets.sort_unstable();

    let part_two = divider_packets
        .iter()
        .filter_map(|divider_packet| packets.binary_search(divider_packet).ok())
        .map(|i| i + 1)
        .product::<usize>();
    println!("Part two: {}", part_two);
}
