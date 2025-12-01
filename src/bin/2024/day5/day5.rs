use itertools::*;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::iter;

fn parse(input: &str) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let sections: Vec<&str> = input.split("\n\n").collect();
    let rules = sections[0]
        .lines()
        .map(|l| {
            let pair = l
                .split('|')
                .map(|v| v.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            return (pair[0], pair[1]);
        })
        .collect();
    let updates = sections[1]
        .lines()
        .map(|l| l.split(',').map(|v| v.parse::<u32>().unwrap()).collect())
        .collect();
    return (rules, updates);
}

fn part1(rules: &Vec<(u32, u32)>, updates: &Vec<Vec<u32>>) -> i32 {
    return updates
        .iter()
        .filter(|u| {
            let mut rule_map: HashMap<u32, HashSet<u32>> = HashMap::new();
            for (before, val) in rules {
                if u.contains(before) && u.contains(val) {
                    rule_map
                        .entry(*val)
                        .or_insert(HashSet::new())
                        .insert(*before);
                }
            }
            let mut seen_vals: HashSet<u32> = HashSet::new();
            for v in *u {
                seen_vals.insert(*v);
                if rule_map.get(v).is_some_and(|r| !r.is_subset(&seen_vals)) {
                    return false;
                }
            }
            return true;
        })
        .map(|u| {
            let val = u[u.len() / 2];
            return val;
        })
        .sum::<u32>() as i32;
}

fn part2(rules: &Vec<(u32, u32)>, updates: &Vec<Vec<u32>>) -> i32 {
    let mut ruleset: HashSet<(&u32, &u32)> = HashSet::new();
    for (before, val) in rules {
        ruleset.insert((before, val));
    }

    let sort_func = |l: &u32, r: &u32| -> Ordering {
        if ruleset.contains(&(l, r)) {
            return Ordering::Less;
        } else if ruleset.contains(&(r, l)) {
            return Ordering::Greater;
        }
        return Ordering::Equal;
    };

    return updates
        .iter()
        .map(|u| {
            if !u.is_sorted_by(|l, r| sort_func(l, r) != Ordering::Greater) {
                let mut new_update = u.clone();
                new_update.sort_by(sort_func);
                return new_update[new_update.len() / 2];
            }
            return 0;
        })
        .sum::<u32>() as i32;
}

fn main() {
    // let input = fs::read_to_string("src/bin/2024/day5/day5_example.txt")
    //     .expect("Should have been able to read the file.");
    let input = fs::read_to_string("src/bin/2024/day5/day5.txt")
        .expect("Should have been able to read the file.");
    let (rules, updates) = parse(&input);
    let p1_result = part1(&rules, &updates);
    println!("Part 1: {p1_result}");
    let p2_result = part2(&rules, &updates);
    println!("Part 2: {p2_result}")
}
