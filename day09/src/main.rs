use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

fn main() {
    println!("--- Part 1 ---");
    part_1();
    println!("--- Part 2 ---");
    part_2();
}

fn part_1() {
    let file = File::open("input.txt").unwrap();
    let lines = BufReader::new(file).lines().map(Result::unwrap);

    // Extrapolate all the values.
    let extra = lines.map(|line| {
        let start = line.split(" ").map(|v| v.parse::<i64>().unwrap());
        let mut history = vec![start.collect::<Vec<_>>()];

        while !history.iter().last().unwrap().iter().all(|v| *v == 0) {
            let curr = history.last().unwrap();
            let next = curr.iter().tuple_windows().map(|(a, b)| b - a);
            history.push(next.collect_vec());
        }

        // Get the last element for each row.
        let last = history.iter().map(|v| v.iter().last().unwrap());

        // Starting from the back, we extrapolate each value using the previous and
        // return the final one.
        last.rev().copied().reduce(|acc, x| acc + x).unwrap()
    });

    // The value we are looking for is the sum of all such values.
    let res = extra.sum::<i64>();

    println!("The sum of all extrapolated values is: {}", res)
}

// Same thing as part 1, but this time we work from the start of each history. We also need to flip the math operations.
// Thankfully very easy with rust iterators. :)
fn part_2() {
    let file = File::open("input.txt").unwrap();
    let lines = BufReader::new(file).lines().map(Result::unwrap);

    let extra = lines.map(|line| {
        let start = line.split(" ").map(|v| v.parse::<i64>().unwrap());
        let mut history = vec![start.collect::<Vec<_>>()];

        while !history.iter().last().unwrap().iter().all(|v| *v == 0) {
            let curr = history.last().unwrap();
            let next = curr.iter().tuple_windows().map(|(a, b)| b - a);
            history.push(next.collect_vec());
        }

        // Get the FIRST element for each row.
        let first = history.iter().map(|v| v.iter().next().unwrap());

        // Flip the operator on acc.
        first.rev().copied().reduce(|acc, x| x - acc).unwrap()
    });

    let res = extra.sum::<i64>();

    println!("The sum of all extrapolated values is: {}", res)
}
