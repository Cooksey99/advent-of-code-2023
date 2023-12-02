use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // const inputs: [&str; 4] = [
    //     "1abc2",
    //     "pqr3stu8vwx",
    //     "a1b2c3d4e5f",
    //     "treb7uchet"
    // ];

    let path = Path::new("inputs.txt");
    let file = File::open(&path);
    let reader = io::BufReader::new(file.unwrap());
    let inputs: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    println!("Inputs: {:?}", inputs);

    let mut total = 0;

    for word in inputs {
        // find first number in word
        let (mut first, mut second) = (0, 0);
        let mut found = false;
        let mut sum = 0;
        let mut array: Vec<u32> = Vec::new();

        for (i, c) in word.chars().enumerate() {
            if c.is_digit(10) {
                if found == true {
                    second = c.to_digit(10).unwrap();
                } else {
                    first = c.to_digit(10).unwrap();
                    second = c.to_digit(10).unwrap();
                }
                found = true;

            }
        }
        let concat = first.to_string() + &second.to_string();

        println!("{} + {} = {}", first, second, concat);
        total += concat.parse::<u32>().unwrap();
    }

    println!("Total: {}", total);

}