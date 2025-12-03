use itertools::*;
use std::fs;
use std::ops::RangeInclusive;

fn parse(input: &str) -> Vec<RangeInclusive<u64>> {
    return input
        .split(",")
        .map(|s| {
            let (start, end) = s.split("-").collect_tuple().unwrap();
            return start.parse().unwrap()..=end.parse().unwrap();
        })
        .collect();
}

fn part1(ranges: &Vec<RangeInclusive<u64>>) -> u64 {
    return ranges.iter().fold(0, |acc, r| {
        let mut count = 0;
        for val in r.clone().into_iter() {
            let val_str = val.to_string();
            let val_split = val_str.split_at(val_str.len() / 2);
            if val_split.0 == val_split.1 {
                count += val;
            }
        }
        return acc + count;
    });
}

fn part2(ranges: &Vec<RangeInclusive<u64>>) -> u64 {
    return ranges.iter().fold(0, |acc, r| {
        let mut count = 0;
        for val in r.clone().into_iter() {
            let val_str = val.to_string();
            for i in (1..=val_str.len() / 2).rev().into_iter() {
                let val_split = val_str.split_at(i).0;
                let rep_count = val_str.match_indices(val_split).collect::<Vec<_>>().len();
                let req_count = val_str.len().div_ceil(i);
                if rep_count == req_count {
                    count += val;
                    break;
                }
            }
        }
        return acc + count;
    });
}

fn main() {
    // let input = fs::read_to_string("src/bin/2025/day2/day2_example.txt")
    //     .expect("Should have been able to read the file.");
    let input = fs::read_to_string("src/bin/2025/day2/day2.txt")
        .expect("Should have been able to read the file.");
    let ranges = parse(&input);
    let p1_result = part1(&ranges);
    println!("Part 1: {p1_result}");
    let p2_result = part2(&ranges);
    println!("Part 2: {p2_result}")
}
