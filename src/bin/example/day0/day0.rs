use itertools::*;
use std::fs;
use std::iter;

fn parse(input: &str) -> (Vec<i32>, Vec<i32>) {
    return (vec![0], vec![0]);
}

fn part1(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
    return 0;
}

fn part2(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
    return 0;
}

fn main() {
    let input = fs::read_to_string("src/bin/example/day0/day0_example.txt")
        .expect("Should have been able to read the file.");
    // let input = fs::read_to_string("src/bin/example/day0/day0.txt")
    //     .expect("Should have been able to read the file.");
    let (left, right) = parse(&input);
    let p1_result = part1(&left, &right);
    println!("Part 1: {p1_result}");
    let p2_result = part2(&left, &right);
    println!("Part 2: {p2_result}")
}
