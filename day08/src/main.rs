use core::panic;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    println!("--- Part 1 ---");
    part_1();
}

fn part_1() {
    let (map, network) = parse_input();
    let map = map.chars();

    let mut pos = "AAA";
    let mut count = 0;
    for action in map.cycle() {
        if pos == "ZZZ" {
            break;
        }

        let (left, right) = network.get(pos).unwrap();
        // println!("{}, {}", left, right);
        pos = match action {
            'L' => left,
            'R' => right,
            _ => panic!("invalid action"),
        };
        count += 1;
    }

    println!("The number of steps: {}", count);
}

fn parse_input() -> (String, HashMap<String, (String, String)>) {
    let file = File::open("input.txt").unwrap();
    let mut lines = BufReader::new(file).lines().map(Result::unwrap);

    let map = lines.next().unwrap();

    let mut network = HashMap::new();
    lines.skip(1).for_each(|l| {
        let mut parts = l.split(" = ");
        let root = parts.next().unwrap();
        let children = parts.next().unwrap();
        let left = &children[1..=3];
        let right = &children[6..=8];
        network.insert(root.to_string(), (left.to_string(), right.to_string()));
    });

    (map, network)
}
