use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    println!("--- Part 1 ---");
    part_1();
    println!("--- Part 2 ---");
    part_2();
}

fn part_1() {
    let input = parse_input();

    let target: HashMap<&str, i32> = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    let output = input
        .enumerate()
        .filter(|(_, x)| {
            // Inverse filter since we want to get games that ARE possible
            !x.iter().any(|y| {
                target.iter().any(|(&k, &v)| {
                    let &val = y.get(k).unwrap_or(&0);
                    val > v
                })
            })
        })
        .map(|(i, _)| i + 1)
        .sum::<usize>();

    println!("Number of possible games: {:?}", output);
}

fn part_2() {
    let input = parse_input();

    let output = input
        // Get max of each cube for all rounds in each game
        .map(|x| {
            x.iter().fold(HashMap::new(), |mut acc, y| {
                y.iter().for_each(|(k, v)| {
                    acc.insert(k.clone(), *v.max(acc.get(k).unwrap_or(&0)));
                });
                acc
            })
        })
        // Multiple the # cubes
        .map(|x| x.values().fold(1, |acc, y| acc * y))
        // Sum over games
        .sum::<i32>();

    println!("The total power is: {}", output)
}

fn parse_input() -> impl Iterator<Item = Vec<HashMap<String, i32>>> {
    let file = File::open("input.txt").unwrap();
    let lines = BufReader::new(file).lines();

    lines
        .map(Result::unwrap)
        // Remove line prefix
        .map(|x| {
            let i = x.find(":").unwrap();
            // + 2 will remove whitespace as well
            x[i + 2..].to_owned()
        })
        // Parse parts within each line
        .map(|x| {
            x.split("; ")
                .map(|y| {
                    y.split(", ").fold(HashMap::new(), |mut acc, y| {
                        let mut substr = y.split(" ");
                        let count = substr.next().unwrap().parse::<i32>().unwrap();
                        let color = substr.next().unwrap();
                        acc.insert(color.to_string(), count);
                        acc
                    })
                })
                .collect::<Vec<_>>()
        })
}
