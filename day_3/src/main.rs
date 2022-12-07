use std::{collections::HashMap, fs};

fn calculate(character: &str) -> u32 {
    match character {
        "a" => 1,
        "b" => 2,
        "c" => 3,
        "d" => 4,
        "e" => 5,
        "f" => 6,
        "g" => 7,
        "h" => 8,
        "i" => 9,
        "j" => 10,
        "k" => 11,
        "l" => 12,
        "m" => 13,
        "n" => 14,
        "o" => 15,
        "p" => 16,
        "q" => 17,
        "r" => 18,
        "s" => 19,
        "t" => 20,
        "u" => 21,
        "v" => 22,
        "w" => 23,
        "x" => 24,
        "y" => 25,
        "z" => 26,
        "A" => 27,
        "B" => 28,
        "C" => 29,
        "D" => 30,
        "E" => 31,
        "F" => 32,
        "G" => 33,
        "H" => 34,
        "I" => 35,
        "J" => 36,
        "K" => 37,
        "L" => 38,
        "M" => 39,
        "N" => 40,
        "O" => 41,
        "P" => 42,
        "Q" => 43,
        "R" => 44,
        "S" => 45,
        "T" => 46,
        "U" => 47,
        "V" => 48,
        "W" => 49,
        "X" => 50,
        "Y" => 51,
        "Z" => 52,
        _ => panic!("character not match"),
    }
}

fn main() {
    let source_path = "./src/source.txt";
    let file_content = fs::read_to_string(source_path).unwrap();

    let rucksacks_list: Vec<&str> = file_content.split("\n").collect();

    let mut general_result: u32 = 0;

    for rucksack_content in rucksacks_list {
        let mut appearances_of_characters: HashMap<String, u32> = HashMap::new();

        let mid = rucksack_content.chars().count() / 2;
        let (first_part, second_part) = rucksack_content.split_at(mid);

        for char in first_part.to_string().chars() {
            appearances_of_characters.insert(char.to_string(), 0);
        }

        for char in second_part.to_string().chars() {
            if let Some(value) = appearances_of_characters.get_mut(&char.to_string()) {
                *value += 1;
            }
        }

        let mut result: u32 = 0;

        for (key, value) in appearances_of_characters {
            if value > 0 {
                result = result + calculate(key.as_str())
            }
        }

        general_result = general_result + result
    }

    println!("general result: {}", general_result)
}
