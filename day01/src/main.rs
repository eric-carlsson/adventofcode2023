use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

fn main() {
    println!("--- Part 1 ---");
    part_1();
    println!("--- Part 2 ---");
    part_2();
}

fn part_1() {
    let file = File::open("input.txt").unwrap();
    let lines = BufReader::new(file).lines();

    let translation_map = HashMap::from([
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);

    let keys = translation_map
        .keys()
        .copied()
        .collect::<Vec<_>>()
        .join("|");
    let pat = Regex::new(format!("({})", keys).as_str()).unwrap();
    let pat_rev =
        Regex::new(format!("({})", keys.chars().rev().collect::<String>()).as_str()).unwrap();

    let sum: u32 = lines
        .map(|x| {
            let s = x.unwrap();

            // Find first digit with regular regexp match
            let first_match = pat.find(s.as_str()).unwrap();
            let first = translation_map.get(first_match.as_str()).unwrap();

            //Find last digit with reverse regexp match
            let s_rev = s.chars().rev().collect::<String>();
            let last_match = pat_rev.find(s_rev.as_str()).unwrap();
            let last = translation_map.get(last_match.as_str()).unwrap();

            // Fast digit concatenation :)
            first * 10 + last
        })
        .sum();

    println!("The sum is: {}", sum);
}

// Same solution as p2 but we extend map with lettered digits too.
fn part_2() {
    let file = File::open("input.txt").unwrap();
    let lines = BufReader::new(file).lines();

    let translation_map = HashMap::from([
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let keys = translation_map
        .keys()
        .copied()
        .collect::<Vec<_>>()
        .join("|");
    let pat = Regex::new(format!("({})", keys).as_str()).unwrap();
    let pat_rev =
        Regex::new(format!("({})", keys.chars().rev().collect::<String>()).as_str()).unwrap();

    let sum: u32 = lines
        .map(|x| {
            let s = x.unwrap();

            // Find first digit with regular regexp match
            let first_match = pat.find(s.as_str()).unwrap();
            let first = translation_map.get(first_match.as_str()).unwrap();

            //Find last digit with reverse regexp match
            let s_rev = s.chars().rev().collect::<String>();
            let last_match = pat_rev.find(s_rev.as_str()).unwrap();
            let last = translation_map
                .get(
                    last_match
                        .as_str()
                        .chars()
                        .rev()
                        .collect::<String>()
                        .as_str(),
                )
                .unwrap();

            // Fast digit concatenation :)
            first * 10 + last
        })
        .sum();

    println!("The sum is: {}", sum);
}
