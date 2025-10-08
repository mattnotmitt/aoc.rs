use itertools::*;
use std::fs;
use std::iter;

fn parse(input: &str) -> Vec<(u64, Vec<u64>)> {
    return input
        .lines()
        .map(|l| {
            let pair: Vec<&str> = l.split(": ").collect();
            return (
                pair[0].parse::<u64>().unwrap(),
                pair[1]
                    .split(" ")
                    .map(|v| v.parse::<u64>().unwrap())
                    .collect(),
            );
        })
        .collect();
}

enum Operator {
    MUL,
    ADD,
    CONCAT,
}

fn check_equation(result: u64, total: u64, operator: Operator, rest: &[u64]) -> bool {
    let mut new_total = total;
    match operator {
        Operator::ADD => new_total += rest[0],
        Operator::MUL => new_total *= rest[0],
        Operator::CONCAT => panic!("never should have come here"),
    }
    if new_total == result {
        return true;
    } else if new_total > result {
        return false;
    } else if rest.len() == 1 {
        return false;
    }
    return check_equation(result, new_total, Operator::ADD, &rest[1..])
        || check_equation(result, new_total, Operator::MUL, &rest[1..]);
}

fn part1(equations: &Vec<(u64, Vec<u64>)>) -> u64 {
    return equations
        .iter()
        .filter_map(|e| {
            if check_equation(e.0, 0, Operator::ADD, e.1.as_slice())
                || check_equation(e.0, 0, Operator::MUL, e.1.as_slice())
            {
                return Some(e.0);
            }
            return None;
        })
        .sum();
}

fn check_equation_2(result: u64, total: u64, operator: Operator, rest: &[u64]) -> bool {
    let mut new_total = total;
    match operator {
        Operator::ADD => new_total += rest[0],
        Operator::MUL => new_total *= rest[0],
        Operator::CONCAT => {
            assert!(&rest[0].to_string().parse::<u64>().unwrap() == &rest[0]);
            new_total = (new_total.to_string() + &rest[0].to_string())
                .parse::<u64>()
                .unwrap();
        }
    }

    if new_total > result {
        return false;
    }
    if rest.len() == 1 {
        if new_total == result {
            return true;
        }
        return false;
    }
    return check_equation_2(result, new_total, Operator::ADD, &rest[1..])
        || check_equation_2(result, new_total, Operator::MUL, &rest[1..])
        || check_equation_2(result, new_total, Operator::CONCAT, &rest[1..]);
}

fn part2(equations: &Vec<(u64, Vec<u64>)>) -> u64 {
    return equations
        .iter()
        .filter_map(|e| {
            if check_equation_2(e.0, e.1[0], Operator::ADD, &e.1[1..])
                || check_equation_2(e.0, e.1[0], Operator::MUL, &e.1[1..])
                || check_equation_2(e.0, e.1[0], Operator::CONCAT, &e.1[1..])
            {
                return Some(e.0);
            }
            return None;
        })
        .sum();
}

fn main() {
    // let input = fs::read_to_string("src/bin/day7/day7_example.txt")
    //     .expect("Should have been able to read the file.");
    let input = fs::read_to_string("src/bin/day7/day7.txt")
        .expect("Should have been able to read the file.");
    let equations = parse(&input);
    let p1_result = part1(&equations);
    println!("Part 1: {p1_result}");
    let p2_result = part2(&equations);
    println!("Part 2: {p2_result}")
}
