use std::{path::Path, fs::File, io::{self, BufRead}, fmt::Display, ops::Range};

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
    fn map_range(&self, value: &Range<i64>) -> Range<i64> {
        let new_start = match value.start < self.source_range_start {
            true => {
                self.source_range_start
            },
            false => {
                value.start
            }
        };
            
        let new_end = match value.end > self.source_range_start + self.range_length {
            true => {
                self.source_range_start + self.range_length
            },
            false => {
                value.end
            }
        };

        let matching_range = new_start..new_end;
        println!("        Matching Range: {:?}", matching_range);
        return matching_range.start - self.source_range_start + self.dest_range_start..matching_range.end - self.source_range_start + self.dest_range_start;
    }

    fn source_range_contains_range(&self, start: i64, end: i64) -> bool{
        let end_of_source_range = self.source_range_start + self.range_length - 1;
        let end_of_range = end;

        let result = (start >= self.source_range_start && start <= end_of_source_range) || (end_of_range >= self.source_range_start && end_of_range <= end_of_source_range);
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

    let binding = line_reader
        .next()
        .unwrap()
        .unwrap();

    let mut seeds = binding
        .split(":")
        .skip(1)
        .next()
        .unwrap()
        .split(" ")
        .filter(|x| !x.chars().all(char::is_whitespace))
        .map(|x| {
            println!("Seed: {}", x);
            x.trim().parse::<i64>().unwrap()
        });


    let mut seeds_ranges = Vec::new(); 
    while let Some(first) = seeds.next(){
        if let Some(second) = seeds.next(){
            seeds_ranges.push(first..first + second);
        }
    }

    let mut final_mappings : Vec<Vec<RangeMapping>> = Vec::new();

    let mut current_mapping_set = Vec::new();

    let mut iterated_reader = line_reader.skip(2);

    // load mappings into memory
    while let Some(line) = iterated_reader.next() {
        let line = line.unwrap();
        println!("Line: {}", line);
        // empty line means we're done with this mapping set
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


    let mut current_ranges = seeds_ranges.clone();
    for mapping_set in &final_mappings {
        let mut new_ranges = Vec::new();
        
        while let Some(range) = current_ranges.pop() {
            for result in process_range_through_mapping_set(&range, mapping_set) {
                new_ranges.push(result);
            }
        }

        current_ranges = new_ranges;
    }
    

    current_ranges.iter().map(|z| z.start).min().unwrap()
}

fn process_range_through_mapping_set(range: &Range<i64>, mappings: &Vec<RangeMapping>) -> Vec<Range<i64>> {
    let mut new_ranges = Vec::new();
    for mapping in mappings {
        match try_mapping_transform_range(mapping, &range) {
            Some(new_range) => {
                new_ranges.push(new_range);
            },
            None => {
                new_ranges.push(range.clone());
            }
        }
    }

    new_ranges
}

fn try_mapping_transform_range(mapping: &RangeMapping, range: &Range<i64>) -> Option<Range<i64>> {
    if !mapping.source_range_contains_range(range.start, range.end) {
        return None;
    }

    println!("    Contains: {:?} -> {:?}", mapping, range);

    Some(mapping.map_range(range))
}