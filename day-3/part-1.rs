use std::collections::HashMap;

fn main() {
    let input = [
        "467..114..",
        "...*......",
        "..35..633.",
        "......#...",
        "617*......",
        ".....+.58.",
        "..592.....",
        "......755.",
        "...$.*....",
        ".664.598..",
    ];


    let mut prev_row = HashMap::new();
    let mut prev_symbol = Vec::new();

    input.iter().enumerate().for_each(
        |(index, row)| {
            // println!("{}: {}", index, row);
            let mut curr_row: HashMap<usize, u32> = HashMap::new();
            let mut symbol_index = Vec::new();

            let mut curr_num = String::new();
            let mut is_num = false;
            row.chars().enumerate().for_each(
                |(i, c)| {
                    if c.is_digit(10) {
                        is_num = true;
                        curr_num.push(c);
                    } else if c == '.' {
                        if curr_num.len() > 0 {
                            let num = curr_num.parse::<u32>().unwrap();
                            curr_row.insert(i, num);
                            curr_num.clear();
                        } 
                    } else { // means it's a symbol
                        symbol_index.push(i);
                    }

                    is_num = false;
                }
            );

            println!("Curr_row: {:?}", curr_row);
            println!("Symbols: {:?}\n", symbol_index);
            
        }
        prev_row = curr_row;
        prev_symbol = symbol_index;
    );

}