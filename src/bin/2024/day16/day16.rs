use itertools::*;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;
use std::hash::Hash;
use std::u32::MAX;
use util::*;

#[derive(PartialEq, Debug, Clone, Copy)]
enum TileType {
    Valid,
    Wall,
    End,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Pose {
    pub position: Point2D,
    pub orientation: Direction2D,
}

fn parse(input: &str) -> (Vec<Vec<TileType>>, Pose) {
    let mut start_pose = Pose {
        position: Point2D::new(0, 0),
        orientation: Direction2D::E,
    };
    return (
        input
            .lines()
            .enumerate()
            .map(|(i, l)| {
                l.chars()
                    .enumerate()
                    .map(|(j, c)| match c {
                        '.' => TileType::Valid,
                        '#' => TileType::Wall,
                        'S' => {
                            start_pose.position = Point2D::new(i as isize, j as isize);
                            return TileType::Valid;
                        }
                        'E' => TileType::End,
                        _ => panic!("unexpected tile type"),
                    })
                    .collect()
            })
            .collect(),
        start_pose,
    );
}

#[derive(Clone, Eq, PartialEq)]
struct State {
    position: Point2D,
    orientation: Direction2D,
    tile_count: u32,
    rotation_count: u32,
    path: HashSet<Point2D>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Prioritize paths with lower total cost: tile_count + rotation_count * 1000
        (other.tile_count + other.rotation_count * 1000)
            .cmp(&(self.tile_count + self.rotation_count * 1000))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn part1(grid: &Vec<Vec<TileType>>, start_pose: &Pose) -> u32 {
    let mut visited = HashSet::new();
    let mut pq = BinaryHeap::new();
    let mut count = 0;

    pq.push(State {
        position: start_pose.position,
        orientation: start_pose.orientation,
        tile_count: 0,
        rotation_count: 0,
        path: HashSet::from([start_pose.position]),
    });

    while let Some(current) = pq.pop() {
        count += 1;
        let current_pose = Pose {
            position: current.position,
            orientation: current.orientation,
        };

        if visited.contains(&current_pose) {
            continue;
        }

        visited.insert(current_pose);

        if *current.position.safe_access(grid).unwrap() == TileType::End {
            println!("Remaining {}, visited {}", pq.len(), count);
            return current.tile_count + current.rotation_count * 1000;
        }

        for dir in Direction2D::iter_cardinal() {
            if *dir == current.orientation.flip() {
                continue;
            }
            let new_pose = Pose {
                position: current.position + *dir,
                orientation: *dir,
            };

            // Check if the new position is valid
            if *new_pose.position.safe_access(grid).unwrap() == TileType::Valid
                || *new_pose.position.safe_access(grid).unwrap() == TileType::End
            {
                let new_tile_count = current.tile_count + 1;
                let new_rotation_count = if current.orientation != *dir {
                    current.rotation_count + 1
                } else {
                    current.rotation_count
                };

                let mut new_path = current.path.clone();
                new_path.insert(new_pose.position);

                let new_state = State {
                    position: new_pose.position,
                    orientation: *dir,
                    tile_count: new_tile_count,
                    rotation_count: new_rotation_count,
                    path: new_path,
                };

                pq.push(new_state);
            }
        }
    }

    // No path found
    0
}

fn part2(grid: &Vec<Vec<TileType>>, start_pose: &Pose) -> u32 {
    let mut min_cost = HashMap::new();
    let mut pq = BinaryHeap::new();
    let mut optimal_seats = HashSet::new();
    let mut optimal_score = MAX;

    pq.push(State {
        position: start_pose.position,
        orientation: start_pose.orientation,
        tile_count: 0,
        rotation_count: 0,
        path: HashSet::from([start_pose.position]),
    });

    // Initialize the min_cost map for the start state
    min_cost.insert((start_pose.position, start_pose.orientation), 0);

    while let Some(current) = pq.pop() {
        let current_state = (current.position, current.orientation);
        let current_cost = current.tile_count + current.rotation_count * 1000;

        // Skip if we've already found a better path
        if let Some(&min_cost_val) = min_cost.get(&current_state) {
            if current_cost > min_cost_val {
                continue;
            }
        }

        // Check if this state is the end
        if *current.position.safe_access(grid).unwrap() == TileType::End {
            let score = current.tile_count + current.rotation_count * 1000;
            if optimal_score == MAX {
                optimal_score = score
            }
            if score > optimal_score {
                break;
            }
            optimal_seats.extend(current.path);
            // println!("Optimal score: {}", score);
            continue; // Continue to process other paths with same cost
        }

        // Process each direction
        for dir in Direction2D::iter_cardinal() {
            let new_pos = current.position + *dir;
            let new_orientation = *dir;

            // Skip if new position is not valid
            if *new_pos.safe_access(grid).unwrap() != TileType::Valid
                && *new_pos.safe_access(grid).unwrap() != TileType::End
            {
                continue;
            }

            let new_tile_count = current.tile_count + 1;
            let new_rotation_count = if current.orientation != new_orientation {
                current.rotation_count + 1
            } else {
                current.rotation_count
            };

            let new_state = (new_pos, new_orientation);
            let new_cost = new_tile_count + new_rotation_count * 1000;

            let mut new_path = current.path.clone();
            new_path.insert(new_pos);

            // Update the minimum cost for this new state
            if !min_cost.contains_key(&new_state) || new_cost < *min_cost.get(&new_state).unwrap() {
                min_cost.insert(new_state, new_cost);
                pq.push(State {
                    position: new_pos,
                    orientation: new_orientation,
                    tile_count: new_tile_count,
                    rotation_count: new_rotation_count,
                    path: new_path,
                });
            } else if new_cost == *min_cost.get(&new_state).unwrap() {
                // Same cost: push into queue to find all paths
                pq.push(State {
                    position: new_pos,
                    orientation: new_orientation,
                    tile_count: new_tile_count,
                    rotation_count: new_rotation_count,
                    path: new_path,
                });
            }
        }
    }

    optimal_seats.len() as u32
}

fn main() {
    // let input = fs::read_to_string("src/bin/2024/day16/day16_example2.txt")
    //     .expect("Should have been able to read the file.");
    let input = fs::read_to_string("src/bin/2024/day16/day16.txt")
        .expect("Should have been able to read the file.");
    let (grid, init) = parse(&input);
    let p1_result = part1(&grid, &init);
    println!("Part 1: {p1_result}");
    let p2_result = part2(&grid, &init);
    println!("Part 2: {p2_result}")
}
