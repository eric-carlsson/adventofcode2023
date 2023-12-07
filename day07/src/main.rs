use std::{
    cmp::Ordering,
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::{
    FoldWhile::{Continue, Done},
    Itertools,
};

fn main() {
    println!("--- Part 1 ---");
    part_1();
}

fn part_1() {
    let file = File::open("input.txt").unwrap();
    let lines = BufReader::new(file).lines().map(Result::unwrap);

    let mut rounds = lines
        .map(|ref l| {
            let mut tokens = l.split(" ");
            let hand = tokens.next().unwrap().to_string();
            let bet = tokens.next().unwrap().parse::<i32>().unwrap();
            (hand, bet)
        })
        .collect::<Vec<_>>();

    rounds.sort_by(|(a, _), (b, _)| match type_value(a).cmp(&type_value(b)) {
        Ordering::Less => Ordering::Less,
        Ordering::Greater => Ordering::Greater,
        Ordering::Equal => compare_hands(a, b),
    });

    let total = rounds.iter().enumerate().fold(0, |acc, (i, (_, bet))| {
        acc + bet * (i32::try_from(i).unwrap() + 1)
    });

    println!("The total winning are: {:?}", total);
}

// Evaluate hand gets the relative value of the hand's type.
fn type_value(hand: &str) -> i32 {
    let counts = hand.chars().counts();
    let val = counts.values().collect::<Vec<_>>();

    // Use ranking rules to compute a relaltive value of each hand
    match val {
        c if c.contains(&&5) => 6,
        c if c.contains(&&4) => 5,
        c if c.contains(&&3) && c.contains(&&2) => 4,
        c if c.contains(&&3) => 3,
        c if c.iter().filter(|v| v == &&&2).count() == 2 => 2,
        c if c.iter().max().unwrap() == &&2 => 1,
        _ => 0,
    }
}

fn cmp_char(a: char, b: char) -> Ordering {
    let tree = vec![
        '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
    ];

    tree.iter()
        .position(|x| *x == a)
        .cmp(&tree.iter().position(|x| *x == b))
}

fn compare_hands(a: &str, b: &str) -> Ordering {
    a.chars()
        .zip(b.chars())
        // THis would ideally be done with recursion, but we use fold_while to simplify it.
        .fold_while(Ordering::Equal, |acc, (x, y)| match cmp_char(x, y) {
            Ordering::Greater => Done(Ordering::Greater),
            Ordering::Less => Done(Ordering::Less),
            Ordering::Equal => Continue(acc),
        })
        .into_inner()
}
