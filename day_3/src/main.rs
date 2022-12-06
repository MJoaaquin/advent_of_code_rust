use std::collections::HashMap;

fn calculate(character: &str) -> u8 {
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
    let rucksacks_content = "vJrwpWtwJgWrhcsFMMfFFhFp".to_string();

    let mid = rucksacks_content.chars().count() / 2;

    let mut appearances_of_characters: HashMap<String, u8> = HashMap::new();

    let (first_part, second_part) = rucksacks_content.split_at(mid);

    for char in first_part.to_string().chars() {
        appearances_of_characters.insert(char.to_string(), 0);
    }

    println!("a: {:?}", appearances_of_characters);

    for char in second_part.to_string().chars() {
        if let Some(value) = appearances_of_characters.get_mut(&char.to_string()) {
            *value += 1;
        }
    }

    println!("b: {:?}", appearances_of_characters);

    let mut result: u8 = 0;

    for (key, value) in appearances_of_characters {
        if value > 0 {
            result = result + calculate(key.as_str())
        }
    }

    println!("{}", result);
}
