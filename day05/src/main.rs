use std::{
    collections::HashSet,
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
    let (seeds, maps) = parse_input();

    let loc = seeds
        .iter()
        .map(|seed| {
            // Traverse maps to find location
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

fn part_2() {
    let (seeds, maps) = parse_input();

    let r = seeds
        .iter()
        .tuples()
        // For each seed range we iterate through the maps, and for each map, we have to compute new ranges.
        .flat_map(|(seed_start, seed_range)| {
            maps.iter()
                .fold(vec![(*seed_start, seed_start + seed_range)], |dest, map| {
                    dest.iter().flat_map(|&r| map_range(r, map)).collect_vec()
                })
        })
        // Get the minimum range (= minimum start of ranges).
        .min_by(|a, b| a.0.cmp(&b.0))
        .unwrap();

    println!("The minimum location number is: {}", r.0);
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

// Map a range of values to a new vector of values, using a convertion map.
fn map_range(src: (i64, i64), map: &HashSet<(i64, i64, i64)>) -> Vec<(i64, i64)> {
    let (start, end) = src;
    // Sort by source range start
    let map_sorted = map.iter().sorted_by(|a, b| a.1.cmp(&b.1));

    let mut dest = Vec::new();
    let mut ptr = start;

    for &(d, s, r) in map_sorted.clone() {
        if s + r < start || s > end {
            // Range is entirely outside src, skip it
            continue;
        }

        if ptr < s {
            // Outside range, do direct mapping
            dest.push((ptr, s));
            ptr = s;
        }

        // Add overlapping part. -s+d here converts from source to dest.
        dest.push((ptr - s + d, end.min(s + r) - s + d));
        ptr = end.min(s + r);
    }

    // Final map range may not cover entire src, in which case we need to directly map this part.
    let fin = map_sorted.last().unwrap();
    let map_end = fin.1 + fin.2;
    if map_end < end {
        dest.push((map_end, end))
    }

    dest
}
