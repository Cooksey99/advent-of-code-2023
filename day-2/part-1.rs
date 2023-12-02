use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // const inputs: [&str; 5] = [
    //     "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
    //     "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
    //     "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
    //     "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
    //     "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
    // ];
    let path = Path::new("inputs.txt");
    let file = File::open(&path);
    let reader = io::BufReader::new(file.unwrap());
    let inputs: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    const RED: u32 = 12;
    const GREEN: u32 = 13;
    const BLUE: u32 = 14;

    let mut total = 0;

    for val in inputs {
        // determine game
        let temp = val.split(":").collect::<Vec<&str>>();
        let game = temp[0].split(" ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
        let mut invalid = false;

        // println!("Game: {}", game);

        let sub_games = temp[1].split(";").collect::<Vec<&str>>();
        // println!("Sub games: {:?}", sub_games);

        for sub_game in sub_games {

            // println!("Sub game: {}", sub_game);
            let pair = sub_game.split(",").collect::<Vec<&str>>()
            .iter()
                .map(|x| x.trim().split(" ").collect::<Vec<&str>>())
                .collect::<Vec<Vec<&str>>>();

            // println!("Pair: {:?}", pair);
            for p in pair {
                let count = p[0].parse::<u32>().unwrap();
                let color = p[1];

                if (color == "red" && count > RED) 
                    || (color == "green" && count > GREEN) 
                    || (color == "blue" && count > BLUE) {
                    invalid = true;
                }
            }
        }

        if invalid == false {
            total += game;
        }
    }

    // 2015
    println!("Total: {}", total);
}