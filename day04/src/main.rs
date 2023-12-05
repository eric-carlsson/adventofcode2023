use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    println!("--- Part 1 ---");
    part_1();
}

fn part_1() {
    let f = File::open("input.txt").unwrap();
    let l = BufReader::new(f).lines().map(Result::unwrap);

    let sum = l
        .map(|line| {
            // Remove line prefix
            let m = line.split(":").nth(1).unwrap();

            // Parse the groups of numbers
            let mut n = m.split("|").map(|group| {
                group
                    .split(" ")
                    // Remove empty strings (caused by single digit numbers)
                    .filter(|s| !s.is_empty())
                    .map(|num| num.parse::<u32>().unwrap())
                    .collect::<HashSet<_>>()
            });

            let want = n.next().unwrap();
            let got = n.next().unwrap();
            let exp = want.intersection(&got).count();

            match exp {
                0 => 0_u32,
                _ => 2_u32.pow(u32::try_from(exp).unwrap() - 1),
            }
        })
        .sum::<u32>();

    println!("The total number of points is: {}", sum);
}
