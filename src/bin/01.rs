use adv_code_2024::start_day;
use anyhow::{Ok, Result};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

fn parse_lists<R: BufRead>(reader: R) -> (Vec<usize>, Vec<usize>) {
    let lines = reader.lines().map(|l| l.unwrap());
    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let nums = line
            .split_whitespace()
            .map(|num| num.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        left.push(nums[0]);
        right.push(nums[1]);
    }
    left.sort_unstable();
    right.sort_unstable();
    (left, right)
}

fn part1(left: &[usize], right: &[usize]) -> usize {
    left.iter()
        .zip(right)
        .fold(0, |acc, (l, r)| acc + l.abs_diff(*r))
}

fn part2(left: &[usize], right: &[usize]) -> usize {
    left.iter()
        .map(|l| (*l) * right.iter().filter(|r| *r == l).count())
        .sum()
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");
    let test_data = BufReader::new(TEST.as_bytes());
    let (test_left, test_right) = parse_lists(test_data);
    assert_eq!(11, part1(&test_left, &test_right));

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let (left, right) = parse_lists(input_file);
    let result = time_snippet!(part1(&left, &right));
    println!("Result = {result}");
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    let test_data = BufReader::new(TEST.as_bytes());
    let (test_left, test_right) = parse_lists(test_data);
    assert_eq!(31, part2(&test_left, &test_right));
    let result = time_snippet!(part2(&left, &right));
    println!("Result = {result}");
    //endregion

    Ok(())
}
