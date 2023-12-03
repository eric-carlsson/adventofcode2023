use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter,
};

use itertools::Itertools;
use regex::Regex;

fn main() {
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
