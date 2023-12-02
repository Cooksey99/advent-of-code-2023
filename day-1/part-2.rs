use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    const inputs: [&str; 7] = [
        "two1nine",
        "eightwothree",
        "abcone2threexyz",
        "xtwone3four",
        "4nineeightseven2",
        "zoneight234",
        "7pqrstsixteen",
    ];

    // let path = Path::new("inputs.txt");
    // let file = File::open(&path);
    // let reader = io::BufReader::new(file.unwrap());
    // let inputs: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    println!("Inputs: {:?}", inputs);

    let mut total = 0;

    let mut map = HashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);
    map.insert("three", 3);
    map.insert("four", 4);
    map.insert("five", 5);
    map.insert("six", 6);
    map.insert("seven", 7);
    map.insert("eight", 8);
    map.insert("nine", 9);

    for word in inputs {
        // find first number in word
        let (mut first, mut second) = (0, 0);
        let mut number = String::new();
        let mut found = false;

        for (i, c) in word.chars().enumerate() {
            if !c.is_digit(10) {
                number += &c.to_string();
                let mut temp = 0;

                for key in map.keys() {
                    if number.contains(key) {
                        
                        temp = *map.get(key).unwrap();


                        if found == true {
                            second = temp;
                        } else {
                            first = temp;
                            second = temp;
                        }
                        found = true;
                        number = String::new();
                    }
                }
            } else {
                number = String::new();

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
