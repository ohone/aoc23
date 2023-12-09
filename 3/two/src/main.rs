use std::{path::Path, fs::File, io::{self, BufRead}, collections::HashMap};

fn main() {
    match process_file("src/input.txt"){
        Ok(res) => println!("{}", res.to_string()),
        Err(e) => println!("Error: {}", e),
    }
}

fn read_lines_into_grid<P>(filename: P) -> io::Result<Vec<Vec<char>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let mut grid = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let row: Vec<char> = line.chars().collect();
        grid.push(row);
    }

    Ok(grid)
}


fn process_file<P>(filename: P) -> io::Result<i32>
where
    P: AsRef<Path>,
{
    println!("Processing file: {:?}", filename.as_ref());
    return match read_lines_into_grid(filename){
        Ok(grid) => {
            let results = process_grid(grid);
            Ok(results)
        },
        Err(e) => Err(e)
    }
}

fn is_number(ch: char) -> bool {
    return ch.is_digit(10);
}

fn is_part_of_gear(start_x: i32, end_x: i32, y: i32, grid: &Vec<Vec<char>>) -> Option<Vec<(i32, i32)>> {
    let start_check_x = match start_x {
        0 => 0,
        _ => start_x-1
    };

    let end_check_x = match end_x {
        x if x == (grid[0].len() as i32) - 1 => end_x,
        _ => end_x + 1,
    };
    let x_to_check =  start_check_x..=end_check_x;

    let y_to_check = match y {
        0 => 0..=1,
        y if y == (grid.len() as i32) - 1 => (y-1)..=y,
        _ => (y-1)..=(y+1)
    };
    
    let gear_chars = x_to_check.clone().flat_map(|x| {
        y_to_check
            .clone()
            .filter(move |y|  gear_char(grid[*y as usize][x as usize]))
            .map(move |y| (x, y))
    }).collect::<Vec<(i32,i32)>>();
    
    return match gear_chars.len() {
        0 => None,
        _ => Some(gear_chars)
    };
}

fn gear_char(ch: char) -> bool {
    match ch{
        '*' => true,
        _ => false
    }
}

fn process_grid(grid: Vec<Vec<char>>) -> i32 {
    let mut numbers : Vec<(String, Vec<(i32,i32)>)> = Vec::new();
    let grid_len = grid.len();
    println!("grid len: {}", grid_len);
    
    let mut current_number : Option<String> = None;

    for (y_index, row) in grid.iter().enumerate() {
        for (x_index, ch) in row.iter().enumerate() {
            match (is_number(*ch), &current_number) {
                (true, Some(number)) => {
                    current_number = Some(format!("{}{}", number, ch));
                },
                (true, None) => {
                    current_number = Some(ch.to_string());
                },
                (false, Some(number)) => {
                    let gear_chars = is_part_of_gear((x_index - number.len()) as i32, (x_index - 1) as i32, y_index as i32, &grid);
                    match gear_chars {
                        Some(gear_chars) => {
                            numbers.push((number.to_string(), gear_chars));
                        },
                        None => {
                            // do nothing
                        }
                    }
                    current_number = None;
                },
                (false, None) => {
                    // do nothing
                }
            }
        }

        match current_number {
            Some(number) => {
                let gear_chars = is_part_of_gear((grid_len - number.len()) as i32, (grid_len - 1) as i32, y_index as i32, &grid);
                match gear_chars {
                    Some(gear_chars) => {
                        numbers.push((number.to_string(), gear_chars));
                    },
                    None => {
                        // do nothing
                    }
                }
                println!("xindex: {}, number: {}", grid_len, number);
                current_number = None;
            },
            None => {
                // do nothing
            }
        }
    }

    let potential_gears = numbers
        .iter()
        .map(|x| x.1.clone())
        .flatten()
        .collect::<Vec<(i32,i32)>>();

    let gears = find_pairs_with_exactly_two_occurrences(potential_gears);

    let mut tally = 0;
    for gear in gears {
        let tally_start = tally;
        let mut maybe_number: Option<&(String, Vec<(i32, i32)>)> = None;
        for number in &numbers {
            if number.1.contains(&gear) {
                match maybe_number {
                    Some(some_number) => {
                        println!("{} {}", some_number.0, number.0);
                        tally += some_number.0.parse::<i32>().unwrap() * number.0.parse::<i32>().unwrap();
                    },
                    None => {
                        maybe_number = Some(number);
                    }
                }
            }
            
            if tally_start != tally{
                break;
            }
        }
    }
    
    tally
}

fn find_pairs_with_exactly_two_occurrences(pairs: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut occurrences = HashMap::new();
    let mut result = Vec::new();

    // Count occurrences of each pair
    for pair in pairs {
        *occurrences.entry(pair).or_insert(0) += 1;
    }

    // Find pairs that occurred exactly twice
    for (pair, count) in occurrences {
        if count == 2 {
            result.push(pair);
        }
    }

    result
}