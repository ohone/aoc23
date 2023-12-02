use std::{path::Path, fs::File, io::{self, BufRead}, collections::VecDeque};

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

    const NUMBER_STRINGS: [&'static str; 9] = ["one", "two", "three","four","five","six","seven","eight","nine"];

    let mut results : Vec<i32> = Vec::new();    

    for line_result in reader.lines() {
        let line = line_result?;

        let mut first_number = '_';
        let mut last_number = '_';

        // index in NUMBER_STRINGS, number of times it has been seen
        let mut queue : VecDeque<(usize,i32)> = VecDeque::new();
        
        for ch in line.chars() {
            if ch.is_numeric() {
                println!("{} is a number", ch);
                if first_number == '_'{
                    first_number = ch;
                }
                last_number = ch;
            }
            else{
                // work through the queue items, if the next char matches, increment the count
                // if the entire string has been found, add to results
                for _ in 0..queue.len(){
                    let (i, count) = queue.pop_front().unwrap();
                    
                    let str = NUMBER_STRINGS[i];
                    let char_to_compare = str.chars().nth(count as usize).unwrap();
                    if char_to_compare == ch{
                        if count == str.len() as i32 - 1{
                            let char_representation = (i + 1).to_string().chars().nth(0).unwrap();
                            if first_number == '_'{
                                first_number = char_representation;
                            }
                            last_number = char_representation;
                        }
                        else{
                            queue.push_back((i, count + 1));
                        }
                    }
                }

                // if the next char matches the first char of any of the strings, add it to the queue
                let potential_new_numbers : Vec<usize> =  NUMBER_STRINGS
                    .iter()
                    .enumerate()
                    .filter_map(|(i, &s)|  s
                            .chars()
                            .nth(0)
                            .filter(|&c| c == ch)
                            .map(|_| i))
                    .collect();

                for p in &potential_new_numbers{
                    queue.push_back((*p, 1));
                }
            }
        }
        
        let mut string = String::new();
        string.push(first_number);
        string.push(last_number);
        println!("{} is the number", string);
        results.push(string.parse::<i32>().unwrap());
    }
    let sum = results.iter().sum::<i32>();

    Ok(sum)
}