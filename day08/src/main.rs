use core::panic;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use num::integer;

fn main() {
    println!("--- Part 1 ---");
    part_1();
    println!("--- Part 2 ----");
    part_2();
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
        pos = match action {
            'L' => left,
            'R' => right,
            _ => panic!("invalid action"),
        };
        count += 1;
    }

    println!("The number of steps: {}", count);
}

// For part 2 the idea is to find cycles that start and end at a end-position (ending with Z).
// The result will be the LCM of the lenghts of the (shortest) of these cycles. This makes some
// fairly heavy assumptions about our input, but the assumptions turned out to be true and it
// makes us not have to brute force. :)
fn part_2() {
    let (map, network) = parse_input();
    let map = map.chars();

    let start_pos = network.keys().filter(|k| k.chars().last() == Some('A'));

    let cycle_lens = start_pos
        .map(|start| {
            let mut pos = start;
            let mut count = 0;
            let mut cycle_pos = HashMap::new();
            for action in map.clone().cycle() {
                let (left, right) = network.get(pos).unwrap();
                pos = match action {
                    'L' => left,
                    'R' => right,
                    _ => panic!("invalid action"),
                };

                count += 1;

                // If we have already visited this end-position we have found a loop.
                // This loop will be the shortest one, so no need to continue searching.
                if cycle_pos.contains_key(pos) {
                    break;
                }

                // If we have reached a end-position we take a note of the position
                // and how long it took us to reach it. This will let us compute the
                // length of the cycle later.
                if pos.chars().last() == Some('Z') {
                    cycle_pos.insert(pos, count);
                }
            }

            // The length of cycle starting from the end-position.
            count - cycle_pos.get(pos).unwrap()
        })
        .collect::<Vec<_>>();

    // The result is the LCM of all the cycle lengths.
    // Quick mafs tells us: lcm(a,b,c) = lcm(a,lcm(b,c))
    let res = cycle_lens
        .iter()
        .fold(1, |acc, x| integer::lcm(acc, i64::from(*x)));

    println!("The number of steps: {:?}", res);
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
