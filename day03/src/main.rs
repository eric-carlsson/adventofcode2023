use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    iter,
};

use itertools::Itertools;
use regex::Regex;

fn main() {
    println!("--- Part 1 ---");
    part_1();
    println!("--- Part 2 ---");
    part_2();
}

fn part_1() {
    let f = File::open("input.txt").unwrap();
    let l = BufReader::new(f).lines().map(Result::unwrap);

    let line_len = 140; // Length of each line
    let empty = ".".repeat(line_len);

    let pattern = Regex::new(r"(\d+)").unwrap();

    let sum =
        // Pad the input with empty lines
        iter::once(empty.clone())
        .chain(l)
        .chain(iter::once(empty))
        // Create a moving window of triplets
        .tuple_windows::<(_, _, _)>()
        // Compute the value sum of middle element for each iteration.
        // The idea is that for each number, we look at the surrounding spaces (+-1)
        // including the above and below line. If any space is not a dot or digit,
        // we should consider this number.
        .map(|tup| {
            // Match all numbers in the middle line.
            let matches = pattern.find_iter(tup.1.as_str());

            // Filter out which numbers are attaches to a special character.
            matches.filter(|m| {
                // Go back one to get surrounding character. Careful about underflow.
                let start = m.start().saturating_sub(1);
                // End gets last idx + 1, so no need to add.
                // For edge case we will have to be careful not to overflow.
                let end = m.end().min(line_len-1);
                [&tup.0, &tup.1, &tup.2].iter().any(|t| {
                    let slice = &t[start..=end];
                    // Check if any character in the slice is a special character.
                    slice.contains(|x: char| {
                        !(x.is_numeric() || x == '.')
                    })
                })
            })
            .map(|m| {
                m.as_str().parse::<i32>().unwrap()
            })
            .sum::<i32>()
        })
        .sum::<i32>();

    println!("The total sum is: {:?}", sum);
}

// Similar idea as pt 1, but now we look only for '*'s. Additionally, we keep track of the location
// of these, and associated numbers, so we can determine which are gears.
fn part_2() {
    let f = File::open("input.txt").unwrap();
    let l = BufReader::new(f).lines().map(Result::unwrap);

    let line_len = 140; // Length of each line
    let empty = ".".repeat(line_len);

    let pattern = Regex::new(r"(\d+)").unwrap();

    // First we get all possible gears.
    let gears =
        // Pad the input with empty lines
        iter::once(empty.clone())
        .chain(l)
        .chain(iter::once(empty))
        // Create a moving window of triplets
        .tuple_windows::<(_, _, _)>()
        .enumerate()
        // We fold all possible gears with corresponding numbers into a hashmap.
        .fold(HashMap::new(), |mut acc: HashMap<(usize, usize), Vec<i32>>, (i, tup)| {
            // Match all numbers in the middle line.
            let matches = pattern.find_iter(tup.1.as_str());

            matches.for_each(|m| {
                // Go back one to get surrounding character. Careful about underflow.
                let start = m.start().saturating_sub(1);
                // End gets last idx + 1, so no need to add.
                // For edge case we will have to be careful not to overflow.
                let end = m.end().min(line_len-1);

                [&tup.0, &tup.1, &tup.2].iter().enumerate().for_each(|(k, t)| {
                    let slice = &t[start..=end];
                    slice.chars().enumerate().filter(|x| x.1 == '*').for_each(|x| {
                        // x,y choord of the gear
                        let k = (start+x.0, i+k-1);
                        let v = m.as_str().parse::<i32>().unwrap();

                        // Append the value to vector corresponding to the x,y choords
                        if let Some(g) = acc.get_mut(&k) {
                            g.push(v);
                        } else {
                            acc.insert(k, vec![v]);
                        }
                    });
                });
            });

            acc
        });

    let sum = gears
        .iter()
        // The gears have exactly two neighbouring numbers.
        .filter(|(_, v)| v.len() == 2)
        // Multiply to get gear ratio. We do this by folding, since iterating over
        // a vector gives us references to the elements, which will be problematic with
        // reduce.
        .map(|(_, v)| v.iter().fold(1, |acc, x| acc * x))
        // Sum up to get the final value.
        .sum::<i32>();

    println!("The total sum of gear ratios is: {}", sum);
}
