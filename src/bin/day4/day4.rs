use std::fs;
use util::*;

fn parse(input: &str) -> Vec<Vec<char>> {
    return input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();
}

fn find_xmas(wordsearch: &Vec<Vec<char>>, i: isize, j: isize) -> i32 {
    let point = Point2D::new(i, j);
    let exp_string = "MAS";
    let mut count = 0;
    for dir in Direction2D::iter() {
        let mut n_p = point;
        let mut rest_chars = exp_string.chars().peekable();
        while let Some(next_exp) = rest_chars.next() {
            n_p = n_p + *dir;
            let next_char = n_p.safe_access(wordsearch);
            if !next_char.is_some_and(|c| *c == next_exp) {
                break;
            } else if !matches!(rest_chars.peek(), Some(_)) {
                count += 1;
            }
        }
    }
    return count;
}

fn part1(wordsearch: &Vec<Vec<char>>) -> i32 {
    let count = wordsearch
        .iter()
        .enumerate()
        .map(|(i, l)| {
            l.iter()
                .enumerate()
                .map(|(j, c)| {
                    if *c == 'X' {
                        return find_xmas(wordsearch, i as isize, j as isize);
                    } else {
                        return 0;
                    };
                })
                .sum::<i32>()
        })
        .sum();
    return count;
}

static X_DIRECTIONS: [Direction2D; 4] = [
    Direction2D::NE,
    Direction2D::SE,
    Direction2D::SW,
    Direction2D::NW,
];

fn find_x_mas(wordsearch: &Vec<Vec<char>>, i: isize, j: isize) -> i32 {
    let point = Point2D::new(i, j);
    let mut exp_char: char;
    for dir in X_DIRECTIONS.iter() {
        let n_point = point + *dir;
        let next_char = n_point.safe_access(wordsearch);
        if next_char.is_some_and(|f| *f == 'S') {
            exp_char = 'M';
        } else if next_char.is_some_and(|f| *f == 'M') {
            exp_char = 'S';
        } else {
            continue;
        }

        let n_point = point + dir.flip();
        let next_char = n_point.safe_access(wordsearch);
        if !next_char.is_some_and(|f| *f == exp_char) {
            continue;
        }

        let n_point = point + dir.rotate_90();
        let next_char = n_point.safe_access(wordsearch);
        if next_char.is_some_and(|f| *f == 'S') {
            exp_char = 'M';
        } else if next_char.is_some_and(|f| *f == 'M') {
            exp_char = 'S';
        } else {
            continue;
        }

        let n_point = point + dir.rotate_90().flip();
        let next_char = n_point.safe_access(wordsearch);
        if !next_char.is_some_and(|f| *f == exp_char) {
            continue;
        }
        return 1;
    }
    return 0;
}

fn part2(wordsearch: &Vec<Vec<char>>) -> i32 {
    let count = wordsearch
        .iter()
        .enumerate()
        .map(|(i, l)| {
            l.iter()
                .enumerate()
                .map(|(j, c)| {
                    if *c == 'A' {
                        return find_x_mas(wordsearch, i as isize, j as isize);
                    } else {
                        return 0;
                    };
                })
                .sum::<i32>()
        })
        .sum();
    return count;
}

fn main() {
    // let input = fs::read_to_string("src/bin/day4/day4_example.txt")
    //     .expect("Should have been able to read the file.");
    let input = fs::read_to_string("src/bin/day4/day4.txt")
        .expect("Should have been able to read the file.");
    let wordsearch = parse(&input);
    let p1_result = part1(&wordsearch);
    println!("Part 1: {p1_result}");
    let p2_result = part2(&wordsearch);
    println!("Part 2: {p2_result}")
}
