use std::{path::Path, fs::File, io::{self, BufRead}, fmt::Display};

fn main() {
    println!("{}", read_lines_into_games("src/input.txt").to_string());
}

#[derive(Debug)]
struct RangeMapping {
    dest_range_start: i64,
    source_range_start: i64,
    range_length: i64
}

impl RangeMapping {
    fn map(&self, value: i64) -> i64 {
        let mut result = value - self.source_range_start;
        result += self.dest_range_start;
        result
    }

    fn source_range_contains(&self, value: i64) -> bool {
        println!("Checking if {} is in range {} to {}", value, self.source_range_start, self.source_range_start + self.range_length);
        let result = value >= self.source_range_start && value < self.source_range_start + self.range_length;
        println!("Result: {}", result);
        
        result
    }
}

fn read_lines_into_games<P>(filename: P) -> i64
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    let reader = io::BufReader::new(file);

    let mut line_reader = reader.lines();

    let seeds = line_reader
        .next()
        .unwrap()
        .unwrap()
        .split(":")
        .skip(1)
        .next()
        .unwrap()
        .split(" ")
        .filter(|x| !x.chars().all(char::is_whitespace))
        .map(|x| {
            println!("Seed: {}", x);
            x.trim().parse::<i64>().unwrap()
        })
        .collect::<Vec<i64>>();

    let mut final_mappings : Vec<Vec<RangeMapping>> = Vec::new();

    let mut current_mapping_set = Vec::new();

    let mut iterated_reader = line_reader.skip(2);

    while let Some(line) = iterated_reader.next() {
        let line = line.unwrap();
        println!("Line: {}", line);
        match line.trim().is_empty() {
            true => {
                final_mappings.push(current_mapping_set);
                current_mapping_set = Vec::new();
                iterated_reader.nth(0);
            },
            false => 
            {
                let parts : Vec<i64> = line.split(" ").filter(|x| !x.is_empty()).map(|p| p.parse::<i64>().unwrap()).collect();
                let mapping = RangeMapping {
                    dest_range_start: parts[0],
                    source_range_start: parts[1],
                    range_length: parts[2]
                };
                current_mapping_set.push(mapping);
            }
        }
    }
    if current_mapping_set.len() > 0 {
        final_mappings.push(current_mapping_set);
    }

    let mut pointers = seeds.clone();
    let mut final_pointers : Vec<i64> = Vec::new();
    for pointer in pointers {
        let mut new_number = pointer;

        println!("Pointer: {}", new_number);

        for mapping_set in &final_mappings {
            let new_numbers : Vec<i64> = mapping_set.iter().filter(|o| o.source_range_contains(new_number)).map(|o| o.map(new_number)).take(1).collect();
            if new_numbers.len() > 0 {
                new_number = new_numbers[0];
                println!("Mapped to: {}", new_number);
            }
            else{
                println!("No mapping found for {}", pointer)
            }
        }

        final_pointers.push(new_number);
    }

    return final_pointers.iter().min().unwrap().clone();
}