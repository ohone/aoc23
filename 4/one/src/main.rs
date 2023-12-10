use std::{path::Path, fs::File, io::{self, BufRead}};

fn main() {
    match process_file("src/input.txt"){
        Ok(res) => println!("{}", res.to_string()),
        Err(e) => println!("Error: {}", e),
    }
}

struct Game {
    winning_numbers: Vec<i32>,
    my_numbers: Vec<i32>
}

fn read_lines_into_games<P>(filename: P) -> io::Result<Vec<Game>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let games = reader.lines().into_iter().map(|line| {
        let line = line.unwrap();
        let parts : Vec<&str> = line.split("|").collect();
        let winning = parts[0].split(":").skip(1).next().unwrap().trim().split(" ").filter(|str| !str.is_empty()).map(|x| x.parse::<i32>().unwrap()).collect();
        let my_numbers = parts[1].trim().split(" ").filter(|str| !str.is_empty()).map(|x| x.parse::<i32>().unwrap()).collect();
        Game {
            winning_numbers: winning,
            my_numbers: my_numbers
        }
    })
    .collect();

    Ok(games)
}


fn process_file<P>(filename: P) -> io::Result<i32>
where
    P: AsRef<Path>,
{
    println!("Processing file: {:?}", filename.as_ref());
    return match read_lines_into_games(filename){
        Ok(games) => {
            let results = games.into_iter().map(|game| game_points(game)).sum();
            Ok(results)
        },
        Err(e) => Err(e)
    }
}


fn game_points(game: Game) -> i32 {  
    let mut points = 0;  
    for number in game.my_numbers.iter() {
        if game.winning_numbers.contains(number) {
            points = match points {
                0 => 1,
                _ => points * 2
            }
        }
    }    
    points
}
