use std::{path::Path, fs::File, io::{self, BufRead}};

fn main() {
    match process_file("src/input.txt"){
        Ok(res) => println!("{}", res.to_string()),
        Err(e) => println!("Error: {}", e),
    }
}

struct Game {
    rounds: Vec<Round>
}

struct Round{
    blue: i32,
    green: i32,
    red: i32
}

fn process_file<P>(filename: P) -> io::Result<i32>
where
    P: AsRef<Path>,
{
    const MAX_CUBES: &'static Round = {
        &Round {
            blue: 14,
            green: 13,
            red: 12
        }
    };

    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let mut results : Vec<i32> = Vec::new();    
    let mut index = 1;
    for line_result in reader.lines() {
        let line = line_result?;

        let rounds : Vec<Round> = line
            .split(":")
            .into_iter()
            .skip(1)
            .next()
            .unwrap()
            .split(";")
            .map(string_to_round)
            .collect();

        let mut illegal = false;
        for round in rounds {
            if round.blue > MAX_CUBES.blue || round.green > MAX_CUBES.green || round.red > MAX_CUBES.red {
                println!("Round {} - Red: {}, Green: {}, Blue: {}", index, round.red, round.green, round.blue);
                println!("Max cubes exceeded");
                illegal = true;
                break;
            }
        }   

        if !illegal{
            results.push(index);
        }

        index += 1;
    }

    let sum = results.iter().sum::<i32>();

    Ok(sum)
}

fn string_to_round(line: &str) -> Round {
    let round_items = line
        .trim()
        .split(", ");

    let mut red : i32 = 0;
    let mut green : i32 = 0;
    let mut blue : i32 = 0;

    for item in round_items {
        let parts : Vec<&str> = item.split(" ").collect();
        let color = parts[1];
        let value = parts[0].parse::<i32>().unwrap();
        match color {
            "red" => red = value,
            "green" => green = value,
            "blue" => blue = value,
            _ => println!("Unknown color {}", color)
        }
    }
    Round {
        blue,
        green,
        red
    }
}