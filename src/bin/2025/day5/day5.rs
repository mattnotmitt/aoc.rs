use std::fs;
use std::ops::RangeInclusive;

fn parse(input: &str) -> (Vec<RangeInclusive<u64>>, Vec<u64>) {
    let split_input: Vec<_> = input.split("\n\n").collect();
    let ranges = split_input[0]
        .lines()
        .map(|l| {
            let range_split: Vec<_> = l.split("-").map(|n| n.parse::<u64>().unwrap()).collect();
            return range_split[0]..=range_split[1];
        })
        .collect();
    let ingredients: Vec<_> = split_input[1]
        .lines()
        .map(|l| l.parse::<u64>().unwrap())
        .collect();
    return (ranges, ingredients);
}

fn part1(ranges: &Vec<RangeInclusive<u64>>, ingredients: &Vec<u64>) -> u32 {
    let mut count = 0;
    for ing in ingredients {
        for range in ranges {
            if range.contains(ing) {
                count += 1;
                break;
            }
        }
    }
    return count;
}

fn part2(ranges: &Vec<RangeInclusive<u64>>) -> u64 {
    let mut sorted_ranges = ranges.clone();
    sorted_ranges.sort_by(|a, b| a.start().cmp(b.start()));
    let mut merged_ranges = vec![sorted_ranges.first().unwrap().clone()];
    for range in sorted_ranges.iter().skip(1) {
        let last_range = (*merged_ranges.last().unwrap()).clone();
        if range.start() > last_range.end() {
            merged_ranges.push(range.clone());
        } else {
            if range.end() > last_range.end() {
                let last_index = merged_ranges.len() - 1;
                merged_ranges[last_index] = *last_range.start()..=*range.end()
            }
        }
    }
    return merged_ranges.iter().map(|r| r.end() - r.start() + 1).sum();
}

fn main() {
    // let input = fs::read_to_string("src/bin/2025/day5/day5_example.txt")
    //     .expect("Should have been able to read the file.");
    let input = fs::read_to_string("src/bin/2025/day5/day5.txt")
        .expect("Should have been able to read the file.");
    let (ranges, ingredients) = parse(&input);
    let p1_result = part1(&ranges, &ingredients);
    println!("Part 1: {p1_result}");
    let p2_result = part2(&ranges);
    println!("Part 2: {p2_result}")
}
