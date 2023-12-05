use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

fn main() {
    println!("--- Part 1 ---");
    part_1();
}

fn part_1() {
    let (seeds, maps) = parse_input();

    let loc = seeds
        .iter()
        .map(|seed| {
            // Traverse maps to find location. This can definitely be solved with recursion,
            // but to keep things "simple" we (ab)use fold instead.
            maps.iter().fold(*seed, |curr, set| {
                let m = set
                    .iter()
                    // Try to find a matching mapping, assuming there can only be one
                    .find(|(_, source_start, length)| {
                        (*source_start..=source_start + length).contains(&curr)
                    });

                match m {
                    Some((dest_start, source_start, _)) => curr - source_start + dest_start,
                    // If there's no match, we should use direct mapping
                    None => curr,
                }
            })
        })
        .min()
        .unwrap();

    println!("The minimum location number is: {:?}", loc);
}

fn parse_input() -> (Vec<i64>, Vec<HashSet<(i64, i64, i64)>>) {
    let f = File::open("input.txt").unwrap();
    let lines = BufReader::new(f).lines().map(Result::unwrap);

    let groupby = lines.group_by(|v| !v.is_empty());
    let mut groups = groupby.into_iter();

    // The first line is special as it contains a list of seed numbers.
    let seeds = groups
        .next()
        // Get the underlying string from the group
        .unwrap()
        .1
        .next()
        .unwrap()
        // Extract and parse the seed numbers
        .strip_prefix("seeds: ")
        .unwrap()
        .split(" ")
        // Numbers are big, so regular i32 is not enough
        .map(|v| v.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    // The remaining lines contains convertion maps. We read these into a set of convertion maps,
    // that each consists of a vector of tuples. We don't need the map name, since we can use the
    // index instead (mapping 0 -> 1 -> 2 ...).
    let maps = groups
        // group_by will include the groups for all keys, so we filter out the groups that
        // correspond to empty lines. But first, we collect into a vector so we can take
        // ownership of the group's content.
        .map(|(_, g)| g.collect_vec())
        .filter(|g| !g.iter().all(|l| l.is_empty()))
        .map(|g| {
            g.iter()
                // We skip the first line as it's the map name.
                .skip(1)
                // The remaining lines we parse and collect into tuples.
                .map(|l| {
                    l.split(" ")
                        .map(|v| v.parse::<i64>().unwrap())
                        // Each line contains three numbers
                        .next_tuple::<(_, _, _)>()
                        .unwrap()
                })
                // Finally we collect the tuples into a set.
                .collect::<HashSet<_>>()
        })
        .collect::<Vec<_>>();

    (seeds, maps)
}
