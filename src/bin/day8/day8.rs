use itertools::*;
use std::collections::{HashMap, HashSet};
use std::fs;
use util::Point2D;

/// Returns width, height and an antenna map
fn parse(input: &str) -> (u32, u32, HashMap<char, Vec<Point2D>>) {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().collect::<Vec<&str>>().len();
    println!("width: {width}, height: {height}");
    let mut antenna_map: HashMap<char, Vec<Point2D>> = HashMap::new();
    input.lines().enumerate().for_each(|(i, l)| {
        l.chars().enumerate().for_each(|(j, c)| {
            if c == '.' {
                return;
            }
            antenna_map
                .entry(c)
                .or_insert(Vec::new())
                .push(Point2D::new(i as isize, j as isize))
        });
    });
    return (width as u32, height as u32, antenna_map);
}

fn check_antinode(width: &u32, height: &u32, node: &Point2D) -> bool {
    return !(node.x < 0 || node.x >= *height as isize || node.y < 0 || node.y >= *width as isize);
}

fn part1(width: &u32, height: &u32, antenna_map: &HashMap<char, Vec<Point2D>>) -> u32 {
    let mut antinodes: HashSet<Point2D> = HashSet::new();

    antenna_map.iter().for_each(|(_, locs)| {
        locs.iter().tuple_combinations().for_each(|(l, r)| {
            let diff = *r - *l;
            let antinode_a = *l - diff;
            if check_antinode(width, height, &antinode_a) {
                antinodes.insert(antinode_a);
            }
            let antinode_b = *r + diff;
            if check_antinode(width, height, &antinode_b) {
                antinodes.insert(antinode_b);
            }
        })
    });
    return antinodes.len() as u32;
}

fn part2(width: &u32, height: &u32, antenna_map: &HashMap<char, Vec<Point2D>>) -> u32 {
    let mut antinodes: HashSet<Point2D> = HashSet::new();

    antenna_map.iter().for_each(|(_, locs)| {
        locs.iter().tuple_combinations().for_each(|(l, r)| {
            antinodes.insert(*l);
            antinodes.insert(*r);
            let diff = *r - *l;
            let mut antinode_a = *l - diff;
            while check_antinode(width, height, &antinode_a) {
                antinodes.insert(antinode_a);
                antinode_a = antinode_a - diff;
            }
            let mut antinode_b = *r + diff;
            while check_antinode(width, height, &antinode_b) {
                antinodes.insert(antinode_b);
                antinode_b = antinode_b + diff;
            }
        })
    });
    return antinodes.len() as u32;
}

fn main() {
    // let input = fs::read_to_string("src/bin/day8/day8_example.txt")
    //     .expect("Should have been able to read the file.");
    let input = fs::read_to_string("src/bin/day8/day8.txt")
        .expect("Should have been able to read the file.");
    let (width, height, antenna_map) = parse(&input);
    let p1_result = part1(&width, &height, &antenna_map);
    println!("Part 1: {p1_result}");
    let p2_result = part2(&width, &height, &antenna_map);
    println!("Part 2: {p2_result}");
}
