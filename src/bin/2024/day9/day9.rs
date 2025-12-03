use std::fs;

fn parse(input: &str) -> (Vec<i32>, Vec<(usize, u32)>, Vec<(usize, u32)>) {
    let mut disk: Vec<i32> = Vec::new();
    let mut free_space: Vec<(usize, u32)> = Vec::new();
    let mut files: Vec<(usize, u32)> = Vec::new();
    input.char_indices().for_each(|(i, c)| {
        if i % 2 == 0 {
            files.push((disk.len(), c.to_digit(10).unwrap()));
            disk.extend(vec![i as i32 / 2; c.to_digit(10).unwrap() as usize]);
        } else {
            free_space.push((disk.len(), c.to_digit(10).unwrap()));
            disk.extend(vec![-1; c.to_digit(10).unwrap() as usize]);
        }
    });
    return (disk, free_space, files);
}

fn part1(disk: &Vec<i32>) -> u64 {
    let mut new_disk = disk.clone();
    let mut fptr = 0;
    for (rptr, v) in disk.iter().enumerate().rev() {
        // println!("{fptr} - {rptr}");
        if fptr >= rptr {
            break;
        }
        while new_disk[fptr] != -1 {
            fptr += 1;
            if fptr >= rptr {
                break;
            }
        }
        if fptr >= rptr {
            break;
        }
        if *v != -1 {
            new_disk[fptr] = *v;
            new_disk[rptr] = -1;
        }
    }
    return new_disk
        .iter()
        .enumerate()
        .map(|(i, v)| if *v != -1 { i as u64 * *v as u64 } else { 0 })
        .sum();
}

fn part2(files: &Vec<(usize, u32)>, free_space: &Vec<(usize, u32)>) -> u64 {
    let mut new_files = files.clone();
    let mut new_free_space = free_space.clone();

    for (file_id, (file_pos, file_size)) in files.iter().enumerate().rev() {
        let mut free_space_data = (-1, (0, 0));
        for (space_id, (space_pos, space_size)) in new_free_space.iter().enumerate() {
            if file_size > space_size || space_pos > file_pos {
                continue;
            } else {
                free_space_data = (space_id as i32, (*space_pos, *space_size));
                break;
            }
        }
        if free_space_data.0 == -1 {
            continue;
        } else {
            new_files[file_id] = (free_space_data.1.0, *file_size);
            // println!(
            //     "Moving {file_id} of size {file_size} to {:?}",
            //     free_space_data.1.0
            // );
            if *file_size == free_space_data.1.1 {
                new_free_space.remove(free_space_data.0 as usize);
            } else if *file_size < free_space_data.1.1 {
                new_free_space[free_space_data.0 as usize] = (
                    free_space_data.1.0 + *file_size as usize,
                    free_space_data.1.1 - *file_size,
                );
            }
        }
    }
    return new_files
        .iter()
        .enumerate()
        .map(|(id, (pos, len))| {
            let mut sum: u64 = 0;
            for x in 0..*len {
                sum += (*pos as u64 + x as u64) * id as u64
            }
            return sum;
        })
        .sum();
}

fn main() {
    // let input = fs::read_to_string("src/bin/2024/day9/day9_example.txt")
    //     .expect("Should have been able to read the file.");
    let input = fs::read_to_string("src/bin/2024/day9/day9.txt")
        .expect("Should have been able to read the file.");
    let (disk, free_space, files) = parse(&input);
    let p1_result = part1(&disk);
    println!("Part 1: {p1_result}");
    let p2_result = part2(&files, &free_space);
    println!("Part 2: {p2_result}")
}
