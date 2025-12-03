use itertools::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use util;
use util::Direction2D;
use util::Point2D;
use util::remove_whitespace;

struct Map {
    robot_position: Point2D,
    box_positions: HashSet<Point2D>,
    wall_positions: HashSet<Point2D>,
}

fn parse(input: &str) -> (Map, Vec<Direction2D>) {
    let split_input: Vec<&str> = input.split("\n\n").collect();
    let mut robot_position = Point2D::new(-1, -1);
    let mut box_positions: HashSet<Point2D> = HashSet::new();
    let mut wall_positions: HashSet<Point2D> = HashSet::new();

    split_input[0].lines().enumerate().for_each(|(i, l)| {
        l.chars().enumerate().for_each(|(j, c)| match c {
            '@' => robot_position = Point2D::new(i as isize, j as isize),
            'O' => drop(box_positions.insert(Point2D::new(i as isize, j as isize))),
            '#' => drop(wall_positions.insert(Point2D::new(i as isize, j as isize))),
            '.' => (),
            _ => panic!("unexpected map tile"),
        })
    });

    let directions = remove_whitespace(split_input[1])
        .chars()
        .map(|c| match c {
            '^' => Direction2D::N,
            '>' => Direction2D::E,
            'v' => Direction2D::S,
            '<' => Direction2D::W,
            _ => panic!("unexpected direction"),
        })
        .collect();
    return (
        Map {
            robot_position,
            box_positions,
            wall_positions,
        },
        directions,
    );
}

fn move_box(
    box_pos: Point2D,
    direction: Direction2D,
    box_positions: &mut HashSet<Point2D>,
    wall_positions: &HashSet<Point2D>,
) -> bool {
    // println!("Moving box @ {:?} {:?}", box_pos, direction);
    let new_pos = box_pos + direction;
    if wall_positions.contains(&new_pos) {
        return false;
    }
    if box_positions.contains(&new_pos)
        && !move_box(new_pos, direction, box_positions, wall_positions)
    {
        return false;
    }
    box_positions.remove(&box_pos);
    box_positions.insert(new_pos);
    return true;
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum BoxHalf {
    L,
    R,
}

fn part1(map: &Map, directions: &Vec<Direction2D>) -> u32 {
    let mut curr_pos = map.robot_position;
    let mut curr_box_positions = map.box_positions.clone();
    for (i, direction) in directions.iter().enumerate() {
        // println!(
        //     "Step {}: {:?} <- {:?}, {:?}",
        //     i, curr_pos, direction, curr_box_positions
        // );
        let new_pos = curr_pos + *direction;
        if map.wall_positions.contains(&new_pos) {
            continue;
        }
        if curr_box_positions.contains(&new_pos)
            && !move_box(
                new_pos,
                *direction,
                &mut curr_box_positions,
                &map.wall_positions,
            )
        {
            continue;
        }
        assert!(
            !map.wall_positions.contains(&new_pos),
            "Robot must not be inside a wall. {:?}",
            new_pos
        );
        assert!(
            !curr_box_positions.contains(&new_pos),
            "Robot must not be inside a box. {:?}",
            new_pos
        );
        curr_pos = new_pos;
    }
    // println!("{:?}", curr_box_positions);
    return curr_box_positions
        .iter()
        .map(|b| (b.x * 100 + b.y) as u32)
        .sum();
}

fn check_box2(
    box_pos: Point2D,
    direction: Direction2D,
    box_positions: &mut HashMap<Point2D, BoxHalf>,
    wall_positions: &HashSet<Point2D>,
) -> bool {
    let box_type = *box_positions.get(&box_pos).unwrap();
    println!(
        "Checking box half {:?} @ {:?} {:?}",
        box_type, box_pos, direction
    );
    let sib_pos = match box_positions.get(&box_pos) {
        Some(BoxHalf::L) => box_pos + Direction2D::E,
        Some(BoxHalf::R) => box_pos + Direction2D::W,
        None => panic!("could not find sibling!"),
    };
    let sib_type = *box_positions.get(&sib_pos).unwrap();
    assert_ne!(box_type, sib_type);
    let new_pos = box_pos + direction;
    let sib_new_pos = sib_pos + direction;
    if wall_positions.contains(&new_pos) || wall_positions.contains(&sib_new_pos) {
        println!(
            "Failed box move, wall in way.\n  main? {}\n  sib? {}",
            wall_positions.contains(&new_pos),
            wall_positions.contains(&sib_new_pos)
        );
        return false;
    }

    let state_copy = box_positions.clone();

    let box_at_new_pos = box_positions.contains_key(&new_pos) && new_pos != sib_pos;
    let new_box_movable =
        box_at_new_pos && check_box2(new_pos, direction, box_positions, wall_positions);
    let box_at_sib_new_pos = box_positions.contains_key(&sib_new_pos) && sib_new_pos != box_pos;
    let new_sib_box_movable =
        box_at_sib_new_pos && check_box2(sib_new_pos, direction, box_positions, wall_positions);

    let no_box_or_box_movable = !box_at_new_pos || new_box_movable;
    let sib_no_box_or_box_movable = !box_at_sib_new_pos || new_sib_box_movable;

    if !(no_box_or_box_movable && sib_no_box_or_box_movable) {
        println!(
            "Box move required, but failed. Resetting state.\n  main: box? {}, movable? {}\n  sib: box? {}, movable? {}",
            box_at_new_pos, new_box_movable, box_at_sib_new_pos, new_sib_box_movable
        );
        state_copy.clone_into(box_positions);
        return false;
    }

    println!(
        "Moving {:?}:{:?} -> {:?}:{:?}",
        box_pos, sib_pos, new_pos, sib_new_pos
    );
    box_positions.remove(&box_pos);
    box_positions.remove(&sib_pos);
    box_positions.insert(new_pos, box_type);
    box_positions.insert(sib_new_pos, sib_type);

    return true;
}

fn part2(map: &Map, directions: &Vec<Direction2D>) -> u32 {
    let mut curr_pos = Point2D {
        x: map.robot_position.x,
        y: map.robot_position.y * 2,
    };
    let mut curr_box_positions: HashMap<Point2D, BoxHalf> = map
        .box_positions
        .iter()
        .flat_map(|b| {
            vec![
                (Point2D::new(b.x, b.y * 2), BoxHalf::L),
                (Point2D::new(b.x, b.y * 2 + 1), BoxHalf::R),
            ]
        })
        .collect();
    let wall_positions: HashSet<Point2D> = map
        .wall_positions
        .iter()
        .flat_map(|w| vec![Point2D::new(w.x, w.y * 2), Point2D::new(w.x, w.y * 2 + 1)])
        .collect();
    println!("Wall Positions: {:?}", wall_positions);
    for (i, direction) in directions.iter().enumerate() {
        println!("\nStep {}: {:?} <- {:?}", i, curr_pos, direction);
        let new_pos = curr_pos + *direction;
        if wall_positions.contains(&new_pos) {
            println!("Failed bot move, wall in way.");
            continue;
        }
        if curr_box_positions.contains_key(&new_pos)
            && !check_box2(
                new_pos,
                *direction,
                &mut curr_box_positions,
                &wall_positions,
            )
        {
            println!("Failed box move.");
            continue;
        }
        println!("Box state after move: {:?}", curr_box_positions);
        assert_eq!(
            map.box_positions.len() * 2,
            curr_box_positions.len(),
            "Number of boxes is larger than initial count."
        );
        assert!(
            !wall_positions.contains(&new_pos),
            "Robot must not be inside a wall. {:?}",
            new_pos
        );
        assert!(
            !curr_box_positions.contains_key(&new_pos),
            "Robot must not be inside a box. {:?}",
            new_pos
        );
        curr_pos = new_pos;
    }
    println!("{:?}", curr_box_positions);
    return curr_box_positions
        .iter()
        .filter(|b| *b.1 == BoxHalf::L)
        .map(|b| (b.0.x * 100 + b.0.y) as u32)
        .sum();
}

fn main() {
    // let input = fs::read_to_string("src/bin/2024/day15/day15_example_big.txt")
    //     .expect("Should have been able to read the file.");
    let input = fs::read_to_string("src/bin/2024/day15/day15.txt")
        .expect("Should have been able to read the file.");
    let (map, directions) = parse(&input);
    let p1_result = part1(&map, &directions);
    println!("Part 1: {p1_result}");
    let p2_result = part2(&map, &directions);
    println!("Part 2: {p2_result}")
}
