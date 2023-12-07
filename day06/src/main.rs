use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    println!("--- Part 1 ----");
    part_1();
    println!("--- Part 2 ----");
    part_2();
}

fn part_1() {
    let file = parse_file().collect::<Vec<_>>();
    let mut lines = file.iter().map(|l| {
        l.split(" ")
            .filter(|v| !v.is_empty())
            .map(|v| v.parse::<i32>().unwrap())
    });

    let times = lines.next().unwrap();
    let distances = lines.next().unwrap();

    let comb = times.zip(distances).fold(1, |comb, (t_tot, d_tot)| {
        comb * (1..=t_tot).fold(0, |sum, t| match (t_tot - t) * t > d_tot {
            true => sum + 1,
            false => sum,
        })
    });

    println!("The total number of ways to win is: {}", comb)
}

fn part_2() {
    let mut lines = parse_file().map(|l| {
        l.chars()
            .filter(|c| *c != ' ')
            .collect::<String>()
            .parse::<i64>()
            .unwrap()
    });

    let t_tot = lines.next().unwrap();
    let d_tot = lines.next().unwrap();

    let sum = (1..=t_tot).fold(0, |sum, t| match (t_tot - t) * t > d_tot {
        true => sum + 1,
        false => sum,
    });

    println!("The total number of ways to win is: {}", sum)
}

fn parse_file() -> impl Iterator<Item = String> {
    let file = File::open("input.txt").unwrap();
    BufReader::new(file)
        .lines()
        .into_iter()
        .map(|x| x.unwrap().split(":").last().unwrap().to_string())
}
