use itertools::*;
use std::fs;
use std::iter;

fn parse(input: &str) -> (Vec<i32>, Vec<i32>) {
    return input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            (
                parts[0].parse::<i32>().unwrap(),
                parts[1].parse::<i32>().unwrap(),
            )
        })
        .unzip();
}

fn part1(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
    return iter::zip(sorted(left), sorted(right))
        .map(|(l, r)| (l - r).abs())
        .sum();
}

fn part2(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
    let right_freq = right.iter().counts();
    return left
        .iter()
        .map(|v| v * (*right_freq.get(v).unwrap_or(&0) as i32))
        .sum();
}

fn main() {
    // let input = fs::read_to_string("src/day1/day1_example.txt")
    //     .expect("Should have been able to read the file.");
    let input =
        fs::read_to_string("src/day1/day1.txt").expect("Should have been able to read the file.");
    let (left, right) = parse(&input);
    let p1_result = part1(&left, &right);
    println!("Part 1: {p1_result}");
    let p2_result = part2(&left, &right);
    println!("Part 2: {p2_result}")
}
