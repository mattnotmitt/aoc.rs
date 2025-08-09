use std::fs;

// fn parse(input: &str) -> (Vec<i32>, Vec<i32>) {
//     return (vec![0], vec![0]);
// }

fn part1(input: &str) -> i32 {
    let mul_command = "mul(";
    let mut total = 0;
    let mut i = 0;
    while i + 4 < input.len() {
        let mut skip_count = 1;
        let command = &input[i..i + 4];
        if command == mul_command {
            skip_count += 3;
            let next_comma = input[i + skip_count..].find(',');
            if next_comma.is_none() || next_comma.unwrap() > 3 {
                i += skip_count;
                continue;
            }
            let first_num =
                input[i + skip_count..(i + skip_count + next_comma.unwrap())].parse::<i32>();
            skip_count += next_comma.unwrap() + 1;
            if first_num.is_err() {
                i += skip_count;
                continue;
            }
            let next_paren = input[i + skip_count..].find(')');
            if next_paren.is_none() || next_paren.unwrap() > 3 {
                i += skip_count;
                continue;
            }
            let second_num =
                input[i + skip_count..(i + skip_count + next_paren.unwrap())].parse::<i32>();
            skip_count += next_paren.unwrap() + 1;
            if second_num.is_err() {
                i += skip_count;
                continue;
            }
            total += first_num.unwrap() * second_num.unwrap();
        }
        i += skip_count;
    }
    return total;
}

fn part2(input: &str) -> i32 {
    let mul_command = "mul(";
    let do_command = "do()";
    let dont_command = "don't()";

    let mut go_ahead = true;
    let mut total = 0;
    let mut i = 0;
    while i + 4 < input.len() {
        let mut skip_count = 1;
        let command = &input[i..i + 7];
        if command.starts_with(do_command) {
            skip_count += 3;
            go_ahead = true;
        } else if command.starts_with(dont_command) {
            skip_count += 6;
            go_ahead = false;
        } else if command.starts_with(mul_command) {
            skip_count += 3;
            if !go_ahead {
                i += skip_count;
                continue;
            }
            let next_comma = input[i + skip_count..].find(',');
            if next_comma.is_none() || next_comma.unwrap() > 3 {
                i += skip_count;
                continue;
            }
            let first_num =
                input[i + skip_count..(i + skip_count + next_comma.unwrap())].parse::<i32>();
            skip_count += next_comma.unwrap() + 1;
            if first_num.is_err() {
                i += skip_count;
                continue;
            }
            let next_paren = input[i + skip_count..].find(')');
            if next_paren.is_none() || next_paren.unwrap() > 3 {
                i += skip_count;
                continue;
            }
            let second_num =
                input[i + skip_count..(i + skip_count + next_paren.unwrap())].parse::<i32>();
            skip_count += next_paren.unwrap() + 1;
            if second_num.is_err() {
                i += skip_count;
                continue;
            }
            total += first_num.unwrap() * second_num.unwrap();
        }
        i += skip_count;
    }
    return total;
}

fn main() {
    let input =
        fs::read_to_string("src/day3/day3.txt").expect("Should have been able to read the file.");
    // let input = fs::read_to_string("src/day3/day3_example.txt")
    //     .expect("Should have been able to read the file.");
    // let (left, right) = parse(&input);
    let p1_result = part1(&input);
    println!("Part 1: {p1_result}");
    let p2_result = part2(&input);
    println!("Part 2: {p2_result}")
}
