use std::{collections::HashMap, fs, path::Path};

fn parse_input(path: impl Into<String>) -> (Vec<u32>, Vec<u32>) {
    let file_string: String = path.into();
    let file_path = Path::new(file_string.as_str());
    let content = fs::read_to_string(file_path).unwrap();
    let mut col1_vec = Vec::new();
    let mut col2_vec = Vec::new();

    content.lines().for_each(|line| {
        let mut numbers = line.split_whitespace();
        let col1 = numbers.next().unwrap().parse::<u32>().unwrap();
        let col2 = numbers.next().unwrap().parse::<u32>().unwrap();
        col1_vec.push(col1);
        col2_vec.push(col2);
    });

    return (col1_vec, col2_vec);
}

fn solve_day_pt1() {
    let (mut left, mut right) = parse_input("./resources/day1.txt");

    left.sort();
    right.sort();

    let distance = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| if l > r { l - r } else { r - l })
        .sum::<u32>();
    println!("[Part 1] distance: {}", distance);
}

fn solve_day_pt2() {
    let (left, right) = parse_input("./resources/day1.txt");
    // Hashmap number, appearence
    let mut map: HashMap<u32, u32> = HashMap::new();

    right.iter().for_each(|r| {
        map.entry(*r).and_modify(|e| *e += 1).or_insert(1);
    });

    let distance: u32 = left
        .iter()
        .map(|val| map.get(val).unwrap_or(&0) * val)
        .sum();

    println!("[Part 2] distance: {:?}", distance);
}

#[cfg(test)]
mod day1 {
    use super::*;

    #[test]
    fn day1_pt1() {
        solve_day_pt1();
    }

    #[test]
    fn day1_pt2() {
        solve_day_pt2();
    }
}
