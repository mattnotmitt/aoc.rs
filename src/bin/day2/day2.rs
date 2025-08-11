use std::fs;

fn parse(input: &str) -> Vec<Vec<i32>> {
    return input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|v| v.parse::<i32>().unwrap())
                .collect()
        })
        .collect();
}

fn part1(reports: &Vec<Vec<i32>>) -> usize {
    return reports
        .iter()
        .filter(|report| {
            let mut direction = 0_i32;
            report
                .windows(2)
                // make pattern irrefutable
                .flat_map(<&[i32; 2]>::try_from)
                .all(|[a, b]| {
                    if direction == 0 {
                        direction = if (a - b).is_positive() { 1 } else { -1 };
                    }
                    let diff = a - b;
                    if diff.is_positive() && direction.is_positive() {
                        return (1..=3).contains(&diff);
                    } else if diff.is_negative() && direction.is_negative() {
                        return (-3..=1).contains(&diff);
                    } else {
                        return false;
                    }
                })
        })
        .count();
}

fn part2(reports: &Vec<Vec<i32>>) -> usize {
    return reports
        .iter()
        .filter(|report| {
            (0..report.len())
                .map(|idx| {
                    report
                        .iter()
                        .enumerate()
                        .filter_map(|(i, e)| if i != idx { Some(e) } else { None })
                        .collect::<Vec<&i32>>()
                })
                .any(|report| {
                    let mut direction = 0_i32;
                    report
                        .windows(2)
                        // make pattern irrefutable
                        .flat_map(<[&i32; 2]>::try_from)
                        .all(|[a, b]| {
                            if direction == 0 {
                                direction = if (a - b).is_positive() { 1 } else { -1 };
                            }
                            let diff = a - b;
                            if diff.is_positive() && direction.is_positive() {
                                return (1..=3).contains(&diff);
                            } else if diff.is_negative() && direction.is_negative() {
                                return (-3..=1).contains(&diff);
                            } else {
                                return false;
                            }
                        })
                })
        })
        .count();
}

fn main() {
    // let input = fs::read_to_string("src/bin/day2/day2_example.txt")
    //     .expect("Should have been able to read the file.");
    let input = fs::read_to_string("src/bin/day2/day2.txt")
        .expect("Should have been able to read the file.");
    let reports = parse(&input);
    let p1_result = part1(&reports);
    println!("Part 1: {p1_result}");
    let p2_result = part2(&reports);
    println!("Part 2: {p2_result}")
}
