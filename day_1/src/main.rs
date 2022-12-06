use std::{fs, str::FromStr};

fn main() {
    let path = "./src/inventory.txt";
    let file_content = fs::read_to_string(path).unwrap();

    let content_splitted: Vec<&str> = file_content.split("\n\n").collect();

    let vector_of_vectors: Vec<Vec<i32>> = content_splitted
        .iter()
        .map(|string| string.split("\n").collect::<Vec<&str>>())
        .map(|vector| {
            vector
                .iter()
                .map(|str| FromStr::from_str(str).unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();

    let summary_of_calories: Vec<i32> = vector_of_vectors
        .iter()
        .map(|vector_of_calories| vector_of_calories.iter().sum())
        .collect();

    println!(
        "higher amount of calories: {}",
        summary_of_calories.iter().max().unwrap()
    );
}
