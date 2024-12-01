use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "01"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

fn  parse_lists<R: BufRead>(reader:R) -> (Vec<isize>, Vec<isize>) {
    let lines = reader.lines().map(|l|l.unwrap());
    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let nums = line.split_whitespace().map(|num| num.parse::<isize>().unwrap()).collect::<Vec<isize>>();
        left.push(nums[0]);
        right.push(nums[1]);
    }
    left.sort();
    right.sort();
    (left,right)
}

fn part1(left: &[isize], right: &[isize]) -> isize {
    left.iter().zip(right).fold(0, |acc, (l, r)| acc + (l - r).abs())
}

fn part2(left: &[isize], right: &[isize]) -> isize {
    let mut sum = 0;
    for l in left {
        let count=right.iter().filter(|r| **r == *l).count();
        sum += l*count as isize;
    }
    sum
}



fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let (mut left, mut right) = parse_lists(input_file);
    //assert_eq!(11, part1(&left,&right));
    let result = time_snippet!(part1(&left,&right));
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    //assert_eq!(31, part2(&left,&right));
    let result = time_snippet!(part2(&left,&right));
    println!("Result = {}", result);
    //endregion

    Ok(())
}
