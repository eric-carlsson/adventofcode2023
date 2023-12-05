use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    println!("--- Part 1 ---");
    part_1();
    println!("--- Part 2 ----");
    part_2();
}

fn part_1() {
    let f = File::open("input.txt").unwrap();
    let l = BufReader::new(f).lines().map(Result::unwrap);

    let sum = l
        .map(|line| {
            let n = n_matches(&line);

            match n {
                0 => 0_u32,
                _ => 2_u32.pow(u32::try_from(n).unwrap() - 1),
            }
        })
        .sum::<u32>();

    println!("The total number of points is: {}", sum);
}

fn part_2() {
    let f = File::open("input.txt").unwrap();
    let l = BufReader::new(f).lines().map(Result::unwrap);

    let mut count: HashMap<usize, u32> = HashMap::new();

    l.enumerate().for_each(|(i, line)| {
        // Add one card. This is our starting card.
        count.entry(i).and_modify(|v| *v += 1).or_insert(1);

        let n = n_matches(&line);
        (i + 1..=i + n).for_each(|j| {
            // Current .card count. We clone to release immutable borrow of count.
            let curr_count = count.get(&i).unwrap().clone();

            count
                // Get entry j+1 since we want to add tickets starting after current one
                .entry(j)
                // Add new card count. Instead of adding 1 N times, we add N directly.
                .and_modify(|v| *v += curr_count)
                .or_insert(curr_count);
        });
    });

    let sum = count.values().sum::<u32>();

    println!("The total number of cards is: {:?}", sum);
}

fn n_matches(line: &str) -> usize {
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
    want.intersection(&got).count()
}
