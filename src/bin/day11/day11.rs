use std::collections::HashMap;
use std::fs;

fn parse(input: &str) -> Vec<u64> {
    return input
        .split(" ")
        .map(|stone| stone.parse::<u64>().unwrap())
        .collect();
}

fn part1(stones: &Vec<u64>) -> u64 {
    let mut memo: HashMap<String, u64> = HashMap::new();

    return stones
        .iter()
        .map(|stone| gen_stones(0, 25, *stone, &mut memo))
        .sum();
}

fn gen_stones(blink: u8, limit: u8, stone: u64, memo: &mut HashMap<String, u64>) -> u64 {
    if blink == limit {
        return 1;
    }
    let key = blink.to_string() + "_" + &stone.to_string();
    if memo.contains_key(&key) {
        return *memo.get(&key).unwrap();
    }
    let ret_val;
    if stone == 0 {
        ret_val = gen_stones(blink + 1, limit, 1, memo);
    } else {
        let s_str = stone.to_string();
        if s_str.len() % 2 == 0 {
            let left = s_str[0..s_str.len() / 2].parse::<u64>().unwrap();
            let right = s_str[s_str.len() / 2..].parse::<u64>().unwrap();
            ret_val = gen_stones(blink + 1, limit, left, memo)
                + gen_stones(blink + 1, limit, right, memo);
        } else {
            ret_val = gen_stones(blink + 1, limit, stone * 2024, memo);
        }
    }
    memo.insert(key, ret_val);
    return ret_val;
}

fn part2(stones: &Vec<u64>) -> u64 {
    let mut memo: HashMap<String, u64> = HashMap::new();

    return stones
        .iter()
        .map(|stone| gen_stones(0, 75, *stone, &mut memo))
        .sum();
}

fn main() {
    // let input = fs::read_to_string("src/bin/day11/day11_example.txt")
    //     .expect("Should have been able to read the file.");
    let input = fs::read_to_string("src/bin/day11/day11.txt")
        .expect("Should have been able to read the file.");
    let stones = parse(&input);
    let p1_result = part1(&stones);
    println!("Part 1: {p1_result}");
    let p2_result = part2(&stones);
    println!("Part 2: {p2_result}")
}
