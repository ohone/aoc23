use std::{path::Path, fs::File, io::{self, BufRead}};

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

fn is_tool(start_x: i32, end_x: i32, y: i32, grid: &Vec<Vec<char>>) -> bool {
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

    for x in x_to_check {
        for y in y_to_check.clone() {
            if tool_char(grid[y as usize][x as usize]) {
                return true;
            }
        }
    }
    return false;
}

fn tool_char(ch: char) -> bool {
    match ch{
        '0'..='9' => false,
        '.' => false,
        _ => true
    }
}

fn process_grid(grid: Vec<Vec<char>>) -> i32 {
    let mut numbers : Vec<String> = Vec::new();
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
                    println!("xindex: {}, number: {}", x_index, number);
                    if is_tool((x_index - number.len()) as i32, (x_index - 1) as i32, y_index as i32, &grid) {
                        numbers.push(number.to_string());
                    }
                    current_number = None;
                },
                (false, None) => {
                    // do nothing
                }
            }
            println!("y: {}, x: {}, ch: {}", y_index, x_index, ch);
        }

        match current_number {
            Some(number) => {
                println!("xindex: {}, number: {}", grid_len, number);
                if is_tool((grid_len - number.len()) as i32, (grid_len - 1) as i32, y_index as i32, &grid) {
                    numbers.push(number.to_string());
                }
                current_number = None;
            },
            None => {
                // do nothing
            }
        }
    }
    return numbers.iter().map(|x| x.parse::<i32>().unwrap()).sum();  
}
