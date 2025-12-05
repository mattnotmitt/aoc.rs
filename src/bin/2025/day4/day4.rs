use itertools::*;
use std::collections::HashSet;
use std::fs;
use std::iter;
use util::*;

fn parse(input: &str) -> HashSet<Point2D> {
    let mut rolls = HashSet::new();
    input.lines().enumerate().for_each(|(x, l)| {
        l.char_indices().for_each(|(y, c)| {
            if c == '@' {
                rolls.insert(Point2D::new(x as isize, y as isize));
            }
        });
    });
    return rolls;
}

fn part1(rolls: &HashSet<Point2D>) -> u32 {
    let mut count = 0;
    for roll in rolls {
        let mut adj_count = 0;
        for dir in Direction2D::iter() {
            if rolls.contains(&(*roll + *dir)) {
                adj_count += 1;
            }
        }
        if adj_count < 4 {
            count += 1;
        }
    }
    return count;
}

fn part2(rolls: &HashSet<Point2D>) -> u32 {
    let mut new_rolls = rolls.clone();
    let mut rem_rolls = HashSet::new();
    let mut count = 0;
    loop {
        for roll in &new_rolls {
            let mut adj_count = 0;
            for dir in Direction2D::iter() {
                if new_rolls.contains(&(*roll + *dir)) {
                    adj_count += 1;
                }
            }
            if adj_count < 4 {
                count += 1;
                rem_rolls.insert(*roll);
            }
        }
        if rem_rolls.len() == 0 {
            break;
        }
        new_rolls = new_rolls.difference(&rem_rolls).cloned().collect();
        rem_rolls.clear();
    }
    return count;
}

fn main() {
    // let input = fs::read_to_string("src/bin/2025/day4/day4_example.txt")
    //     .expect("Should have been able to read the file.");
    let input = fs::read_to_string("src/bin/2025/day4/day4.txt")
        .expect("Should have been able to read the file.");
    let rolls = parse(&input);
    let p1_result = part1(&rolls);
    println!("Part 1: {p1_result}");
    let p2_result = part2(&rolls);
    println!("Part 2: {p2_result}")
}
