use std::{path::Path, fs::File, io::{self, BufRead}, os::unix::process};

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
            Ok(process_games(games))
        },
        Err(e) => Err(e)
    }
}

fn process_games(games: Vec<Game>) -> i32{
    let mut counters : Vec<i32> = vec![0; games.len()];
    for (index,game) in games.iter().enumerate() {
        println!("Counters: {:?}", counters);
        println!("Game: {}", index);

        let points = matching_numbers(game);
        let times_to_run_card = counters[index] + 1;

        for _ in 0..times_to_run_card {
            for card_to_copy_idx in (index as i32)..((index as i32)+ points){
                counters[card_to_copy_idx as usize + 1] = counters[card_to_copy_idx as usize + 1] + 1;
            }
        }
    }

    counters.into_iter().map(|x| x + 1).sum()
}

fn matching_numbers(game: &Game) -> i32 {  
    game.my_numbers.iter().filter(|&&number| game.winning_numbers.contains(&number)).count() as i32
}
