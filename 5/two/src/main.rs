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
        let mut start = value.start - self.source_range_start;
        start += self.dest_range_start;
        let mut end_result = value.end - self.source_range_start;
        end_result += self.dest_range_start;
        start..end_result
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

    let mut final_pointers = Vec::new();
    for pointer in seeds_ranges {
        let mut new_number = pointer;
        println!("Range: {:?}", new_number);
        for mapping_set in &final_mappings {
            println!("  Mapping set: {:?}", mapping_set);
            let new_range : Vec<Range<i64>> = mapping_set
                .iter()
                .filter(|o| o.source_range_contains_range(new_number.start, new_number.end))
                .map(|o| o.map_range(&new_number))
                .take(1)
                .collect();

            if new_range.len() > 0 {
                println!("  ==Matched: {:?}", new_range[0]);
                new_number = new_range[0].clone();
            }
        }

        final_pointers.push(new_number);
    }

    return final_pointers.iter().map(|x| x.start).min().unwrap();
}