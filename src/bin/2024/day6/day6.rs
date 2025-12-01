use itertools::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::iter;
use util;
use util::Direction2D;
use util::Point2D;

#[derive(PartialEq, Debug, Clone)]
enum TileType {
    Empty,
    Visited,
    Blocked,
}

fn parse(input: &str) -> (Vec<Vec<TileType>>, Point2D) {
    let mut guard_pos = Point2D::new(0, 0);
    return (
        input
            .lines()
            .enumerate()
            .map(|(i, l)| {
                l.chars()
                    .enumerate()
                    .map(|(j, c)| match c {
                        '.' => TileType::Empty,
                        '#' => TileType::Blocked,
                        '^' => {
                            guard_pos = Point2D::new(i as isize, j as isize);
                            return TileType::Visited;
                        }
                        _ => panic!("unexpected tile type"),
                    })
                    .collect()
            })
            .collect(),
        guard_pos,
    );
}

fn part1(mut tilemap: Vec<Vec<TileType>>, mut guard_pos: Point2D) -> (i32, HashSet<Point2D>) {
    let mut dir = Direction2D::N;
    let mut visited_points: HashSet<Point2D> = HashSet::new();
    loop {
        let new_guard_pos = guard_pos + dir;
        match new_guard_pos.safe_access(&tilemap) {
            None => break,
            Some(TileType::Blocked) => dir = dir.rotate_90(),
            Some(_) => {
                tilemap[new_guard_pos.x as usize][new_guard_pos.y as usize] = TileType::Visited;
                guard_pos = new_guard_pos;
                visited_points.insert(guard_pos);
            }
        }
    }
    return (
        tilemap
            .iter()
            .flatten()
            .filter(|t| **t == TileType::Visited)
            .count() as i32,
        visited_points,
    );
}

fn part2(
    inp_tilemap: &Vec<Vec<TileType>>,
    inp_guard_pos: Point2D,
    visited_points: &HashSet<Point2D>,
) -> i32 {
    let mut count = 0;
    for point in visited_points {
        let mut tilemap = inp_tilemap.clone();
        let mut guard_pos = inp_guard_pos.clone();
        tilemap[point.x as usize][point.y as usize] = TileType::Blocked;
        let mut dir = Direction2D::N;
        let mut visit_counter: HashMap<Point2D, u8> = HashMap::new();
        loop {
            let new_guard_pos = guard_pos + dir;
            match new_guard_pos.safe_access(&tilemap) {
                None => break,
                Some(TileType::Blocked) => dir = dir.rotate_90(),
                Some(_) => {
                    tilemap[new_guard_pos.x as usize][new_guard_pos.y as usize] = TileType::Visited;
                    guard_pos = new_guard_pos;
                    let visit_count = visit_counter.entry(guard_pos).or_insert(0);
                    if *visit_count > 3 {
                        // becomes stable at 4 visits
                        count += 1;
                        // println!(
                        //     "{}\n",
                        //     tilemap
                        //         .iter()
                        //         .enumerate()
                        //         .map(|(i, l)| l
                        //             .iter()
                        //             .enumerate()
                        //             .map(|(j, t)| match t {
                        //                 TileType::Blocked => {
                        //                     if i as isize == point.x && j as isize == point.y {
                        //                         'O'
                        //                     } else {
                        //                         '#'
                        //                     }
                        //                 }
                        //                 TileType::Empty => '.',
                        //                 TileType::Visited => 'X',
                        //             })
                        //             .join(""))
                        //         .join("\n")
                        // );
                        break;
                    }
                    *visit_count += 1;
                }
            }
        }
    }
    return count;
}

fn main() {
    // let input = fs::read_to_string("src/bin/2024/day6/day6_example.txt")
    //     .expect("Should have been able to read the file.");
    let input = fs::read_to_string("src/bin/2024/day6/day6.txt")
        .expect("Should have been able to read the file.");
    let (tilemap, guard_pos) = parse(&input);
    let (p1_result, visited_points) = part1(tilemap.clone(), guard_pos);
    println!("Part 1: {p1_result}");
    let p2_result = part2(&tilemap, guard_pos, &visited_points);
    println!("Part 2: {p2_result}")
}
