/*
I don't know why, but I don't really like this solution, I feel like there should be a cleaner way to do this, but I'm to lazy to search for it.
*/
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::{collections::HashMap, fs, path::Path};

fn parse_input(path: impl Into<String>) -> Vec<Vec<i32>> {
    let file_string: String = path.into();
    let file_path = Path::new(file_string.as_str());
    let content = fs::read_to_string(file_path).unwrap();

    let data = content
        .lines()
        .map(|l| l.split_whitespace().map(|val| val.parse().unwrap()).collect())
        .collect();

    return data;
}

fn solve_day_pt1() {
    let input = parse_input("./resources/day2.txt");

    let result: i32 = input
        .par_iter()
        .map(|vec| {
            let sign = (vec[0] - vec[1]).signum();
            vec.windows(2)
                .all(|w| (1..=3).contains(&((w[0] - w[1]) * sign))) as i32
        })
        .sum();

    println!("[Part 1] result: {}", result);
}

fn solve_day_pt2() {
    let input = parse_input("./resources/day2.txt");
    let result: u32 = input
        .par_iter()
        .map(|vec| {
            let is_safe = |v: &[i32]| {
                let sign = (v[0] - v[1]).signum();
                v.windows(2)
                    .all(|w| (1..=3).contains(&((w[0] - w[1]) * sign)))
            };

            if is_safe(vec) {
                return 1;
            }

            (0..vec.len()).any(|i| {
                is_safe(
                    &vec.iter()
                        .enumerate()
                        .filter_map(|(j, &x)| if i != j { Some(x) } else { None })
                        .collect::<Vec<_>>(),
                )
            }) as u32
        })
        .sum();

    println!("[Part 2] result: {}", result);
}

#[cfg(test)]
mod day2 {
    use super::*;

    #[test]
    fn day2_pt1() {
        solve_day_pt1();
    }

    #[test]
    fn day2_pt2() {
        solve_day_pt2();
    }
}
