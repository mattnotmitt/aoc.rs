use itertools::*;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;
use std::iter;
use util::Direction2D;
use util::Point2D;

fn parse(input: &str) -> (Vec<Vec<u8>>, Vec<Point2D>) {
    let mut trailheads: Vec<Point2D> = Vec::new();
    return (
        input
            .lines()
            .enumerate()
            .map(|(i, l)| {
                l.chars()
                    .enumerate()
                    .map(|(j, c)| {
                        let elevation = c.to_digit(10).unwrap() as u8;
                        if elevation == 0 {
                            trailheads.push(Point2D::new(i as isize, j as isize));
                        }
                        return elevation;
                    })
                    .collect()
            })
            .collect(),
        trailheads,
    );
}

fn part1(map: &Vec<Vec<u8>>, trailheads: &Vec<Point2D>) -> usize {
    let directions: Vec<Direction2D> = Vec::from([
        Direction2D::N,
        Direction2D::E,
        Direction2D::S,
        Direction2D::W,
    ]);
    return trailheads
        .iter()
        .map(|t| {
            let mut peaks: HashSet<Point2D> = HashSet::new();
            let mut q: VecDeque<Point2D> = VecDeque::from([*t]);
            while q.front().is_some() {
                let pos = q.pop_front().unwrap();
                let height = pos.safe_access(map).unwrap();
                for d in &directions {
                    let new_pos = pos + *d;
                    let maybe_new_height = (new_pos).safe_access(map);
                    if maybe_new_height.is_some_and(|v| v - height == 1) {
                        if maybe_new_height.is_some_and(|v| *v == 9) {
                            peaks.insert(new_pos);
                        } else {
                            q.push_back(new_pos);
                        }
                    }
                }
            }
            // println!("{:?}", peaks);

            return peaks.iter().count();
        })
        .sum();
}

fn part2(map: &Vec<Vec<u8>>, trailheads: &Vec<Point2D>) -> usize {
    let directions: Vec<Direction2D> = Vec::from([
        Direction2D::N,
        Direction2D::E,
        Direction2D::S,
        Direction2D::W,
    ]);
    return trailheads
        .iter()
        .map(|t| {
            let mut score = 0;
            let mut q: VecDeque<Point2D> = VecDeque::from([*t]);
            while q.front().is_some() {
                let pos = q.pop_front().unwrap();
                let height = pos.safe_access(map).unwrap();
                for d in &directions {
                    let new_pos = pos + *d;
                    let maybe_new_height = (new_pos).safe_access(map);
                    if maybe_new_height.is_some_and(|v| v - height == 1) {
                        if maybe_new_height.is_some_and(|v| *v == 9) {
                            score += 1;
                        } else {
                            q.push_back(new_pos);
                        }
                    }
                }
            }
            // println!("{:?}", peaks);

            return score;
        })
        .sum();
}

fn main() {
    // let input = fs::read_to_string("src/bin/day10/day10_example.txt")
    //     .expect("Should have been able to read the file.");
    let input = fs::read_to_string("src/bin/day10/day10.txt")
        .expect("Should have been able to read the file.");
    let (map, trailheads) = parse(&input);
    let p1_result = part1(&map, &trailheads);
    println!("Part 1: {p1_result}");
    let p2_result = part2(&map, &trailheads);
    println!("Part 2: {p2_result}")
}
