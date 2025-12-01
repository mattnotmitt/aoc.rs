use std::fs;

fn parse(input: &str) -> Vec<i32> {
    return input
        .lines()
        .map(|l| {
            let res = l.replace("R", "0").replace("L", "-");
            return res.parse::<i32>().unwrap();
        })
        .collect();
}

fn part1(instructions: &Vec<i32>) -> i32 {
    let mut zero_count = 0;
    instructions.iter().fold(50, |dial, i| {
        let mut res = dial + i;
        res = res.rem_euclid(100);
        if res == 0 {
            zero_count += 1;
        }
        // println!("{res}");
        return res;
    });
    return zero_count;
}

fn part2(instructions: &Vec<i32>) -> i32 {
    let mut zero_count = 0;
    instructions.iter().fold(50, |dial, i| {
        let res = (dial + i).rem_euclid(100);
        // println!("({dial} + {i}) % 100 = {res}");
        zero_count += (i / 100).abs();
        if i.is_positive() && res <= dial && dial != 0 {
            // println!("Hit!");
            zero_count += 1;
        } else if i.is_negative() && res >= dial && dial != 0 {
            // println!("Hit!");
            zero_count += 1;
        } else if res == 0 {
            // println!("Hit!");
            zero_count += 1;
        }
        return res;
    });
    return zero_count;
}

fn main() {
    // let input = fs::read_to_string("src/bin/2025/day1/day1_example.txt")
    //     .expect("Should have been able to read the file.");
    let input = fs::read_to_string("src/bin/2025/day1/day1.txt")
        .expect("Should have been able to read the file.");
    let instructions = parse(&input);
    let p1_result = part1(&instructions);
    println!("Part 1: {p1_result}");
    let p2_result = part2(&instructions);
    println!("Part 2: {p2_result}")
}
