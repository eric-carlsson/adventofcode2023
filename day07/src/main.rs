use std::{
    cmp::Ordering,
    fs::File,
    io::{BufRead, BufReader},
    vec,
};

use itertools::{
    FoldWhile::{Continue, Done},
    Itertools,
};

fn main() {
    println!("--- Part 1 ---");
    part_1();
    println!("--- Part 2 ---");
    part_2();
}

fn part_1() {
    let mut rounds = parse_input().collect::<Vec<_>>();

    let faces = vec![
        '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
    ];

    // First we compare the type ranks of the hands.
    rounds.sort_by(|(a, _), (b, _)| match type_value(a).cmp(&type_value(b)) {
        Ordering::Less => Ordering::Less,
        Ordering::Greater => Ordering::Greater,
        // If the type ranks are equal, we move on to checking the value of the faces for each of the cards.
        Ordering::Equal => compare_faces(a, b, &faces),
    });

    // Now we sum the bets up.
    let total = rounds.iter().enumerate().fold(0, |acc, (i, (_, bet))| {
        acc + bet * (i32::try_from(i).unwrap() + 1)
    });

    println!("The total winning are: {:?}", total);
}

// Similar to part 1. There's probably a smart way of modifying the ranking criteria to do this,
// but since the space of all possible hands when considering jokers is still relatively small, we
// can simply brute force the solution by testing all possible hands and picking those that give
// the best rank.
fn part_2() {
    let mut rounds = parse_input().collect::<Vec<_>>();

    // Joker is now worth the least in direct face comparison.
    let faces = vec![
        'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
    ];

    rounds.sort_by(|(a, _), (b, _)| {
        // Skip the joker, get all joker combinations
        let ab = vec![a, b];
        // Evaluate all possible hands that can be formed by replacing the joker with each of the other card faces.
        let mut comb = ab.iter().map(|hand| {
            faces
                .iter()
                // Skip the joker itself
                .skip(1)
                .map(|c| type_value(&hand.replace('J', c.to_string().as_str())))
                .max()
                .unwrap()
        });

        match comb.next().unwrap().cmp(&comb.next().unwrap()) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => compare_faces(a, b, &faces),
        }
    });

    let total = rounds.iter().enumerate().fold(0, |acc, (i, (_, bet))| {
        acc + bet * (i32::try_from(i).unwrap() + 1)
    });

    println!("The total winning are: {:?}", total);
}

fn parse_input() -> impl Iterator<Item = (String, i32)> {
    let file = File::open("input.txt").unwrap();
    let lines = BufReader::new(file).lines().map(Result::unwrap);

    lines.map(|ref l| {
        let mut tokens = l.split(" ");
        let hand = tokens.next().unwrap().to_string();
        let bet = tokens.next().unwrap().parse::<i32>().unwrap();
        (hand, bet)
    })
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

fn cmp_char(a: char, b: char, char_order: &Vec<char>) -> Ordering {
    char_order
        .iter()
        .position(|x| *x == a)
        .cmp(&char_order.iter().position(|x| *x == b))
}

fn compare_faces(a: &str, b: &str, char_order: &Vec<char>) -> Ordering {
    a.chars()
        .zip(b.chars())
        // THis would ideally be done with recursion, but we use fold_while to simplify it.
        .fold_while(Ordering::Equal, |acc, (x, y)| {
            match cmp_char(x, y, char_order) {
                Ordering::Greater => Done(Ordering::Greater),
                Ordering::Less => Done(Ordering::Less),
                Ordering::Equal => Continue(acc),
            }
        })
        .into_inner()
}
