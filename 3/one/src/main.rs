use std::{path::Path, fs::File, io::{self, BufRead}};

fn main() {
    match process_file("src/input.txt"){
        Ok(res) => println!("{}", res.to_string()),
        Err(e) => println!("Error: {}", e),
    }
}

#[derive(Debug)]
enum CheckResult{
    None,
    Symbol,
    Number,
    ToolNumber
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

fn process_grid(grid: Vec<Vec<char>>) -> i32 {
    let mut numbers : Vec<String> = Vec::new();
    let grid_len = grid.len();
    println!("grid len: {}", grid_len);
    for (y_index, row) in grid.iter().enumerate() {
        let mut last_result : CheckResult = CheckResult::None;
        let is_tool = |grid: &Vec<Vec<char>>, x: usize, y: usize| {
            let is_symbol = |grid: &Vec<Vec<char>>, x: usize, y: usize| {
                match grid[y][x]{
                    '0'..='9' => false,
                    '.' => false,
                    _ => true
                }
            };

            let y_indices_to_check = match y{
                0 => {
                    vec![y+1]
                },
                _ if y == grid_len - 1 => vec![y-1, y],
                _ => {
                   vec![y-1, y+1]
                }
            };

            let x_length = grid[0].len();
            let x_indices_to_check = match x {
                0 => vec![x, x + 1],
                _ if x == x_length - 1 => vec![x],
                _ => vec![x]
            };

            let mut is_tool = false;
            for y_index in &y_indices_to_check{
                for x_index in &x_indices_to_check{
                    if is_symbol(grid, *x_index, *y_index){
                        is_tool = true;
                        break;
                    }
                }
            }
            is_tool
        };

        let is_start_of_tool = |grid: &Vec<Vec<char>>, x: usize, y: usize| {
            let is_symbol = |grid: &Vec<Vec<char>>, x: usize, y: usize| {
                match grid[y][x]{
                    '0'..='9' => false,
                    '.' => false,
                    _ => true
                }
            };

            match y {
                0 => is_symbol(grid, x, y+1),
                _ if y == grid_len - 1 => is_symbol(grid, x, y-1),
                _ => match x {
                    0 => false,
                    _ => is_symbol(grid, x - 1, y-1) || is_symbol(grid, x - 1, y+1)
                }
            }
        };

        let marks_end_of_tool = |grid: &Vec<Vec<char>>, x: usize, y: usize| {
            let is_symbol = |grid: &Vec<Vec<char>>, x: usize, y: usize| {
                match grid[y][x]{
                    '0'..='9' => false,
                    '.' => false,
                    _ => true
                }
            };

            if is_symbol(grid, x, y){
                return true;
            }

            return match y {
                0 => is_symbol(grid, x, y+1),
                _ if y == grid_len - 1 => is_symbol(grid, x, y-1),
                _ => is_symbol(grid, x, y-1) || is_symbol(grid, x, y+1)
            }
        };

        let mut current_number : Option<String> = None;

        for (x_index, ch) in row.iter().enumerate() {
            match last_result {
                CheckResult::Symbol => {
                    println!("encountered Symbol");
                    // print char
                    println!("{}", ch);
                    match ch {
                        '0'..='9' => {
                            last_result = CheckResult::ToolNumber;
                            current_number = Some(ch.to_string());
                        },
                        '.' => last_result = CheckResult::None,
                        _ => last_result = CheckResult::Symbol
                    }
                }
                CheckResult::Number => {
                    println!("encountered Number");
                    match ch {
                        '0'..='9' => {
                            if is_tool(&grid, x_index, y_index){
                                last_result = CheckResult::ToolNumber;
                            } else {
                                last_result = CheckResult::Number;
                            }
                            current_number = Some(format!("{}{}", current_number.unwrap(), ch));
                        },
                        '.' => { 
                            let end_of_tool = marks_end_of_tool(&grid, x_index, y_index);
                            if end_of_tool {
                                println!("encountered end of tool");
                                numbers.push(current_number.take().unwrap());
                                last_result = CheckResult::Symbol;
                            } else {
                                last_result = CheckResult::None;
                            }
                        }, // symbol, promote any in progress numbers
                        _ => {
                            numbers.push(current_number.take().unwrap());
                            last_result = CheckResult::Symbol;
                        }
                    }
                },
                CheckResult::ToolNumber => {
                    println!("encountered ToolNumber");
                    match ch {
                        // if last number was invalid, so it this
                        '0'..='9' => {
                            current_number = Some(format!("{}{}", current_number.unwrap(), ch));
                            last_result = CheckResult::ToolNumber;
                        },
                        '.' => {
                            last_result = CheckResult::None;
                            numbers.push(current_number.take().unwrap());
                            current_number = None;
                        },
                        _ => {
                            last_result = CheckResult::Symbol;
                            numbers.push(current_number.take().unwrap());
                            current_number = None;
                        }
                    };
                },
                CheckResult::None => {
                    println!("encountered None");
                    match ch {
                         // start a new number
                        '0'..='9' => {
                            current_number = Some(ch.to_string());
                            let is_tool = is_tool(&grid, x_index, y_index);
                            let is_start_of_tool = is_start_of_tool(&grid, x_index, y_index);
                            println!("is_tool: {}", is_tool);
                            println!("is_start_of_tool: {}", is_start_of_tool);

                            last_result = match is_tool || is_start_of_tool{
                                true =>  CheckResult::ToolNumber,
                                false =>  CheckResult::Number
                            };
                        },
                        '.' => (), // noop
                        _ =>  last_result = CheckResult::Symbol
                        
                    }
                }
            }
            match &current_number {
                Some(number) => println!("current number: {}", number),
                None => println!("current number: none")
            }
            println!("y: {}, x: {}, ch: {}", y_index, x_index, ch);
        }
        
        match last_result {
            CheckResult::ToolNumber => numbers.push(current_number.take().unwrap()),
            _ => ()
        }
        last_result = CheckResult::None;
    }
    println!("{:?}", numbers);
    let mut sum = 0;
    numbers
        .iter()
        .map(|number| number.parse::<i32>().unwrap())
        .for_each(|number| sum += number);
    sum
}