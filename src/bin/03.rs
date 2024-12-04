use adv_code_2024::{read_input, start_day};
use anyhow::{Ok, Result};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST1: &str = "\
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
";

const TEST2: &str = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
";

fn part1<R: BufRead>(reader: R) -> Result<usize> {
    let data = read_input(reader)?;
    let instruction = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut result = 0;
    for (_, [op1, op2]) in instruction.captures_iter(&data).map(|c| c.extract()) {
        result += op1.parse::<usize>().unwrap() * op2.parse::<usize>().unwrap();
    }
    Ok(result)
}

fn part2<R: BufRead>(reader: R) -> Result<usize> {
    let data = read_input(reader)?;
    let instruction =
        Regex::new(r"(?:mul\((?<op1>\d+),(?<op2>\d+)\))|(?<on>do\(\))|(?<off>don't\(\))").unwrap();
    let mut flag = true;
    let mut result = 0;

    for capture in instruction.captures_iter(&data) {
        if capture.name("on").is_some() {
            flag = true;
            continue;
        }
        if capture.name("off").is_some() {
            flag = false;
            continue;
        }
        if flag {
            let (num1, num2) = (&capture["op1"], &capture["op2"]);
            result += num1.parse::<usize>().unwrap() * num2.parse::<usize>().unwrap();
        }
    }
    Ok(result)
}

fn main() -> Result<()> {
    start_day(DAY);
    //region Part 1
    println!("=== Part 1 ===");

    assert_eq!(161, part1(BufReader::new(TEST1.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {result}");
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    assert_eq!(48, part2(BufReader::new(TEST2.as_bytes()))?);
    //
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {result}");
    //endregion

    Ok(())
}
