use itertools::*;
use regex::Regex;
use std::{collections::HashSet, fs};
use util::Point2D;

fn parse(input: &str) -> (Vec<Point2D>, Vec<Point2D>) {
    let re = Regex::new(r"(-?\d+),(-?\d+).+?(-?\d+),(-?\d+)").unwrap();

    return input
        .lines()
        .map(|l| {
            let (_, [px, py, vx, vy]) = re.captures(l).unwrap().extract();
            return (
                Point2D::new(px.parse().unwrap(), py.parse().unwrap()),
                Point2D::new(vx.parse().unwrap(), vy.parse().unwrap()),
            );
        })
        .unzip();
}

fn part1(positions: &Vec<Point2D>, velocities: &Vec<Point2D>, size: &(isize, isize)) -> u64 {
    let mut curr_positions = (*positions).to_vec();
    let (mid_x, mid_y) = (size.0 / 2, size.1 / 2);
    println!("{:?}", (mid_x, mid_y));
    let mut quadrant_count = (0, 0, 0, 0);
    for i in 0..100 {
        // println!("{i}");
        let mut new_positions: Vec<Point2D> = Vec::with_capacity(curr_positions.len());
        for (pos, vel) in curr_positions.iter().zip(velocities.iter()) {
            let new_pos = pos.move_wrap(size, vel);
            new_positions.push(new_pos);
            if i == 99 {
                if new_pos.x < mid_x && new_pos.y < mid_y {
                    quadrant_count.0 += 1;
                } else if new_pos.x > mid_x && new_pos.y < mid_y {
                    quadrant_count.1 += 1;
                } else if new_pos.x < mid_x && new_pos.y > mid_y {
                    quadrant_count.2 += 1;
                } else if new_pos.x > mid_x && new_pos.y > mid_y {
                    quadrant_count.3 += 1;
                }
            }
        }
        curr_positions = new_positions;
    }
    // println!("{:?}", curr_positions);
    return quadrant_count.0 * quadrant_count.1 * quadrant_count.2 * quadrant_count.3;
}

fn print_grid(grid: &Vec<Vec<char>>) {
    grid.iter()
        .for_each(|r| println!("{}", r.iter().collect::<String>()));
}

fn part2(positions: &Vec<Point2D>, velocities: &Vec<Point2D>, size: &(isize, isize)) {
    let mut curr_positions = (*positions).to_vec();
    for i in 0..10000 {
        let mut grid = vec![vec!['.'; size.1 as usize]; size.0 as usize];
        let mut new_positions: Vec<Point2D> = Vec::with_capacity(curr_positions.len());
        for (pos, vel) in curr_positions.iter().zip(velocities.iter()) {
            let new_pos = pos.move_wrap(size, vel);
            new_pos.update(&mut grid, '#');
            new_positions.push(new_pos);
        }
        if HashSet::<Point2D>::from_iter(new_positions.iter().cloned()).len() == 500 {
            println!("{}", i + 1);
            print_grid(&grid);
        }
        curr_positions = new_positions;
    }
    return;
}

fn main() {
    // let input = fs::read_to_string("src/bin/2024/day14/day14_example.txt")
    //     .expect("Should have been able to read the file.");
    // let size = (11, 7);
    let input = fs::read_to_string("src/bin/2024/day14/day14.txt")
        .expect("Should have been able to read the file.");
    let size = (101, 103);
    let (positions, velocities) = parse(&input);
    let p1_result = part1(&positions, &velocities, &size);
    println!("Part 1: {p1_result}"); // not 225030960
    part2(&positions, &velocities, &size);
}
