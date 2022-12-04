use std::fs;
use std::ops::RangeInclusive;

struct RangePair {
    left: RangeInclusive<u32>,
    right: RangeInclusive<u32>,
}

impl RangePair {
    fn from_line(line: &str) -> Self {
        let (left, right) = line.split_once(',').expect("Must be pair");
        let (left_start, left_end) = left.split_once('-').expect("Range must have start and end");
        let (right_start, right_end) = right.split_once('-').expect("Range must have start and end");
        
        Self{
            left: (left_start.parse().unwrap()..=left_end.parse().unwrap()),
            right: (right_start.parse().unwrap()..=right_end.parse().unwrap()),
        }
    }

    fn either_full_overlaps(&self) -> bool {
        self.left.contains(self.right.start()) && self.left.contains(self.right.end())
            || self.right.contains(self.left.start()) && self.right.contains(self.left.end())
    }

    fn either_partially_overlaps(&self) -> bool {
        self.left.contains(self.right.start()) || self.left.contains(self.right.end())
            || self.right.contains(self.left.start()) || self.right.contains(self.left.end())
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read file");

    let pairs: Vec::<RangePair> = input.split("\n")
                        .map(|line| line.trim())
                        .map(RangePair::from_line)
                        .collect();

    let pair_full_overlaps = pairs.iter().filter(|pair| pair.either_full_overlaps()).count();
    println!("Part one count: {}", pair_full_overlaps);       
    
    let pair_partial_overlaps = pairs.iter().filter(|pair| pair.either_partially_overlaps()).count();
    println!("Part two count: {}", pair_partial_overlaps);    
}