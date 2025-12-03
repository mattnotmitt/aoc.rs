use itertools::*;
use std::fs;
use std::iter;

fn parse(input: &str) -> Vec<Vec<u32>> {
    return input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
}

fn part1(banks: &Vec<Vec<u32>>) -> u32 {
    return banks
        .iter()
        .map(|batts| {
            let mut max_tens = 0;
            let mut tens_idx = 0;
            let mut max_units = 0;
            for (i, y) in batts.iter().enumerate().take(batts.len() - 1) {
                if *y > max_tens {
                    tens_idx = i;
                    max_tens = *y;
                }
            }
            // println!("Skipping first {tens_idx} for units.");
            for y in batts.iter().skip(tens_idx + 1) {
                if *y > max_units {
                    max_units = *y;
                }
            }
            // println!(
            //     "Found {max_tens} & {max_units} -> {}",
            //     max_tens * 10 + max_units
            // );
            return max_tens * 10 + max_units;
        })
        .sum();
}

fn part2(banks: &Vec<Vec<u32>>) -> u64 {
    let digits = 12;
    return banks
        .iter()
        .map(|batts| {
            let mut bank_total: u64 = 0;
            let mut last_index: isize = -1;
            for i in 0..digits {
                let remaining = digits - i;
                let max = batts
                    .iter()
                    .enumerate()
                    .skip((last_index + 1) as usize)
                    .take(batts.len() - remaining - (last_index as usize))
                    .max_by(|a, b| {
                        return a.1.cmp(b.1).then_with(|| b.0.cmp(&a.0));
                    })
                    .unwrap();
                last_index = max.0 as isize;
                bank_total += *max.1 as u64 * 10u64.pow(remaining as u32 - 1)
            }
            return bank_total;
        })
        .sum();
}

fn main() {
    // let input = fs::read_to_string("src/bin/2025/day3/day3_example.txt")
    //     .expect("Should have been able to read the file.");
    let input = fs::read_to_string("src/bin/2025/day3/day3.txt")
        .expect("Should have been able to read the file.");
    let banks = parse(&input);
    let p1_result = part1(&banks);
    println!("Part 1: {p1_result}");
    let p2_result = part2(&banks);
    println!("Part 2: {p2_result}");
}
