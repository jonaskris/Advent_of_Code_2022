use std::fs;
use std::iter::Peekable;

struct Directory {
    name: String,
    files: Vec::<(u32, String)>,
    directories: Vec::<Directory>,
}

impl Directory {
    fn from_lines<'a>(lines: &mut Peekable<impl Iterator<Item = &'a str>>) -> Self {
        let mut new_directory = Directory { name: lines.next().unwrap().split(' ').last().unwrap().to_string(), files: vec![], directories: vec![] };
        
        while let Some(i) = lines.peek() {
            let mut split_iter = i.split(' ');
            let (a, b, c) = (split_iter.next().expect(""), split_iter.next().expect(""), split_iter.next());
                        
            match (a, b) {
                ("$", "cd") => {
                    if c == Some("..") {
                        lines.next();
                        return new_directory;
                    } else {
                        new_directory.directories.push(Directory::from_lines(lines));
                    }
                },
                ("$", "ls") => {lines.next(); continue;},
                ("dir", _) => {lines.next(); continue;},
                _ => {lines.next(); new_directory.files.push((a.parse::<u32>().unwrap(), b.to_string()))}, 
            }
        }

        return new_directory;
    }

    fn size(&self) -> u32 {
        return &self.directories.iter().map(|dir| dir.size()).sum::<u32>() + &self.files.iter().map(|(size, _)| size).sum::<u32>();
    }

    fn sum_dir_sizes_under_limit(&self, limit: u32) -> u32 {
        let self_size = self.size();
        let children_size = self.directories.iter().map(|dir| dir.sum_dir_sizes_under_limit(limit)).sum::<u32>();
        
        if self_size <= limit {
            return self_size + children_size;
        }

        return children_size;
    }

    fn collect_dir_sizes(&self, directories: &mut Vec::<u32>) {
        directories.push(self.size());

        self.directories.iter().for_each(|dir| dir.collect_dir_sizes(directories));
    }

    fn get_used_space(&self) -> u64 {
        return self.files.iter().map(|(size, _)| *size as u64).sum::<u64>() + self.directories.iter().map(|dir| dir.get_used_space()).sum::<u64>();
    }

    fn print_self(&self, spacing: usize) {        
        println!("{}{}", " ".repeat(spacing), self.name);

        for (size, name) in &self.files {
            println!("{}{} (file, size={})", " ".repeat(spacing + 1), name, size);
        }

        for dir in &self.directories {
            dir.print_self(spacing + 1);
        }
    }
}

fn main() {
    let input_string = fs::read_to_string("input.txt").expect("Unable to read file");
    let input = input_string.split("\n").map(|line| line.trim()).collect::<Vec::<&str>>();

    let dir = Directory::from_lines(&mut input.into_iter().peekable());


    println!("Part one: {}", dir.sum_dir_sizes_under_limit(100000));

    let mut dir_sizes: Vec::<u32> = Vec::new();
    dir.collect_dir_sizes(&mut dir_sizes);
    let used_space: u64 = dir.get_used_space();
    let disk_size = 70000000;
    let required_space = 30000000 - (disk_size - used_space);
    println!("Part two: {}", dir_sizes.iter().filter(|&&size| size as u64 >= required_space).min().unwrap());
}