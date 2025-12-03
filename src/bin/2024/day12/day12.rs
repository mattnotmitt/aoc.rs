use itertools::*;
use std::collections::HashSet;
use std::fs;
use util::Direction2D;
use util::Point2D;

fn parse(input: &str) -> Vec<Vec<char>> {
    return input.lines().map(|l| l.chars().collect()).collect();
}

fn check_plots(
    farm: &Vec<Vec<char>>,
    checked_points: &mut HashSet<Point2D>,
    plot: &Point2D,
    plant: &char,
) -> (u32, u32, u32) {
    checked_points.insert(*plot);
    let mut t_area = 1;
    let mut t_perimeter = 4;
    let mut t_corners = 0;

    if (*plot + Direction2D::N)
        .safe_access(farm)
        .is_none_or(|p| p != plant)
        && (*plot + Direction2D::E)
            .safe_access(farm)
            .is_none_or(|q| q != plant)
    {
        t_corners += 1;
        // println!("{plant}: Has NE convex corner @ {:?}", plot)
    }

    if (*plot + Direction2D::E)
        .safe_access(farm)
        .is_none_or(|p| p != plant)
        && (*plot + Direction2D::S)
            .safe_access(farm)
            .is_none_or(|q| q != plant)
    {
        t_corners += 1;
        // println!("{plant}: Has SE convex corner @ {:?}", plot)
    }
    if (*plot + Direction2D::S)
        .safe_access(farm)
        .is_none_or(|p| p != plant)
        && (*plot + Direction2D::W)
            .safe_access(farm)
            .is_none_or(|q| q != plant)
    {
        t_corners += 1;
        // println!("{plant}: Has SW convex corner @ {:?}", plot)
    }
    if (*plot + Direction2D::W)
        .safe_access(farm)
        .is_none_or(|p| p != plant)
        && (*plot + Direction2D::N)
            .safe_access(farm)
            .is_none_or(|q| q != plant)
    {
        t_corners += 1;
        // println!("{plant}: Has NW convex corner @ {:?}", plot)
    }

    if (*plot + Direction2D::N)
        .safe_access(farm)
        .is_some_and(|p| p == plant)
        && (*plot + Direction2D::E)
            .safe_access(farm)
            .is_some_and(|q| q == plant)
        && (*plot + Direction2D::NE)
            .safe_access(farm)
            .is_some_and(|r| r != plant)
    {
        t_corners += 1;
        // println!("{plant}: Has NE concave corner @ {:?}", plot)
    }

    if (*plot + Direction2D::E)
        .safe_access(farm)
        .is_some_and(|p| p == plant)
        && (*plot + Direction2D::S)
            .safe_access(farm)
            .is_some_and(|q| q == plant)
        && (*plot + Direction2D::SE)
            .safe_access(farm)
            .is_some_and(|r| r != plant)
    {
        t_corners += 1;
        // println!("{plant}: Has SE concave corner @ {:?}", plot)
    }

    if (*plot + Direction2D::S)
        .safe_access(farm)
        .is_some_and(|p| p == plant)
        && (*plot + Direction2D::W)
            .safe_access(farm)
            .is_some_and(|q| q == plant)
        && (*plot + Direction2D::SW)
            .safe_access(farm)
            .is_some_and(|r| r != plant)
    {
        t_corners += 1;
        // println!("{plant}: Has SW concave corner @ {:?}", plot)
    }

    if (*plot + Direction2D::W)
        .safe_access(farm)
        .is_some_and(|p| p == plant)
        && (*plot + Direction2D::N)
            .safe_access(farm)
            .is_some_and(|q| q == plant)
        && (*plot + Direction2D::NW)
            .safe_access(farm)
            .is_some_and(|r| r != plant)
    {
        t_corners += 1;
        // println!("{plant}: Has NW concave corner @ {:?}", plot)
    }

    for d in Direction2D::iter_cardinal() {
        let new_plot = *plot + *d;
        if new_plot.safe_access(farm).is_some_and(|p| p == plant) {
            t_perimeter -= 1;
            if checked_points.contains(&new_plot) {
                continue;
            }
            let (area, perimeter, corners) = check_plots(
                farm,
                checked_points,
                &new_plot,
                new_plot.safe_access(farm).unwrap(),
            );
            t_area += area;
            t_perimeter += perimeter;
            t_corners += corners;
        }
    }
    // println!(
    //     "Checking {plant} @ {:?}. Area {t_area}, Perimeter {t_perimeter}",
    //     plot
    // );
    return (t_area, t_perimeter, t_corners);
}

fn part1(farm: &Vec<Vec<char>>) -> u32 {
    let mut total = 0;
    let mut checked_points: HashSet<Point2D> = HashSet::new();
    for (i, l) in farm.iter().enumerate() {
        for (j, p) in l.iter().enumerate() {
            let plot = Point2D::new(i as isize, j as isize);
            if checked_points.contains(&plot) {
                continue;
            }
            let (area, perimeter, _) = check_plots(farm, &mut checked_points, &plot, p);
            // println!(
            //     "Checking {p} @ {:?}. Area {area}, Perimeter {perimeter}",
            //     plot
            // );
            total += area * perimeter;
        }
    }
    return total;
}

fn part2(farm: &Vec<Vec<char>>) -> u32 {
    let mut total = 0;
    let mut checked_points: HashSet<Point2D> = HashSet::new();
    for (i, l) in farm.iter().enumerate() {
        for (j, p) in l.iter().enumerate() {
            let plot = Point2D::new(i as isize, j as isize);
            if checked_points.contains(&plot) {
                continue;
            }
            let (area, _, corners) = check_plots(farm, &mut checked_points, &plot, p);
            println!("Checking {p} @ {:?}. Area {area}, Corners {corners}", plot);
            total += area * corners;
        }
    }
    return total;
}

fn main() {
    // let input = fs::read_to_string("src/bin/2024/day12/day12_example.txt")
    //     .expect("Should have been able to read the file.");
    let input = fs::read_to_string("src/bin/2024/day12/day12.txt")
        .expect("Should have been able to read the file.");
    let farm = parse(&input);
    let p1_result = part1(&farm);
    println!("Part 1: {p1_result}");
    let p2_result = part2(&farm);
    println!("Part 2: {p2_result}")
}
