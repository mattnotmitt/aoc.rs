use regex::Regex;
use std::fs;

struct Machine {
    pub button_a: (i64, i64),
    pub button_b: (i64, i64),
    pub prize: (i64, i64),
}

impl Machine {
    pub fn new(ax: i64, ay: i64, bx: i64, by: i64, px: i64, py: i64) -> Machine {
        Machine {
            button_a: (ax, ay),
            button_b: (bx, by),
            prize: (px, py),
        }
    }
}

fn parse(input: &str) -> Vec<Machine> {
    return input
        .split("\n\n")
        .map(|m| {
            let lines: Vec<&str> = m.lines().collect();
            let re = Regex::new(r"(?:\+|=)(\d+),.+(?:\+|=)(\d+)").unwrap();
            let (_, [ax_s, ay_s]) = re.captures(lines[0]).unwrap().extract();
            let (_, [bx_s, by_s]) = re.captures(lines[1]).unwrap().extract();
            let (_, [px_s, py_s]) = re.captures(lines[2]).unwrap().extract();
            return Machine::new(
                ax_s.parse::<i64>().unwrap(),
                ay_s.parse::<i64>().unwrap(),
                bx_s.parse::<i64>().unwrap(),
                by_s.parse::<i64>().unwrap(),
                px_s.parse::<i64>().unwrap(),
                py_s.parse::<i64>().unwrap(),
            );
        })
        .collect();
}

fn part1(machines: &Vec<Machine>) -> u64 {
    return machines
        .iter()
        .map(|m| {
            // a = button_a.0, b = button_b.0, c = button_a.1, d = button_b.1
            let det = m.button_a.0 * m.button_b.1 - m.button_b.0 * m.button_a.1;
            let a = (m.button_b.1 * m.prize.0 - m.button_b.0 * m.prize.1) / det;
            let b = (m.button_a.0 * m.prize.1 - m.button_a.1 * m.prize.0) / det;
            // println!("{:?}, {:?}: {a}, {b}", m.prize.0, m.prize.1);
            if (
                m.button_a.0 * a + m.button_b.0 * b,
                m.button_a.1 * a + m.button_b.1 * b,
            ) != (m.prize.0, m.prize.1)
            {
                return 0;
            }

            return (a as u64) * 3 + (b as u64);
        })
        .sum();
}

fn part2(machines: &Vec<Machine>) -> u64 {
    return machines
        .iter()
        .map(|m| {
            let prize_x = m.prize.0 + 10000000000000;
            let prize_y = m.prize.1 + 10000000000000;
            // a = button_a.0, b = button_b.0, c = button_a.1, d = button_b.1
            let det = m.button_a.0 * m.button_b.1 - m.button_b.0 * m.button_a.1;
            let a = (m.button_b.1 * prize_x - m.button_b.0 * prize_y) / det;
            let b = (m.button_a.0 * prize_y - m.button_a.1 * prize_x) / det;
            // println!("{:?}, {:?}: {a}, {b}", m.prize.0, m.prize.1);
            if (
                m.button_a.0 * a + m.button_b.0 * b,
                m.button_a.1 * a + m.button_b.1 * b,
            ) != (prize_x, prize_y)
            {
                return 0;
            }

            return (a as u64) * 3 + (b as u64);
        })
        .sum();
}

fn main() {
    // let input = fs::read_to_string("src/bin/2024/day13/day13_example.txt")
    //     .expect("Should have been able to read the file.");
    let input = fs::read_to_string("src/bin/2024/day13/day13.txt")
        .expect("Should have been able to read the file.");
    let machines = parse(&input);
    let p1_result = part1(&machines);
    println!("Part 1: {p1_result}");
    let p2_result = part2(&machines);
    println!("Part 2: {p2_result}")
}
