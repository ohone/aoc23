use std::{path::Path, fs::File, io::{self, BufRead}};

fn main() {
    match process_file("src/input.txt"){
        Ok(res) => println!("{}", res.to_string()),
        Err(e) => println!("Error: {}", e),
    }
}

fn process_file<P>(filename: P) -> io::Result<i32>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);


    let mut results : Vec<i32> = Vec::new();    
    for line_result in reader.lines() {
        let line = line_result?;
        let mut first_char = '_';
        let mut last_char = '_';
        for ch in line.chars() {
            if ch.is_numeric() {
                println!("{} is a number", ch);
                if first_char == '_'{
                    first_char = ch;
                }
                last_char = ch;
            }
        }
        let mut string = String::new();
        string.push(first_char);
        string.push(last_char);
        println!("{} is the number", string);
        results.push(string.parse::<i32>().unwrap());
    }

    let sum = results.iter().sum::<i32>();

    Ok(sum)
}