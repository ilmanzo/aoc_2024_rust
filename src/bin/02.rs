use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

fn is_safe(report: &[i64]) -> bool {
    let sign = (report[0] - report[1]).signum();
    report[1..].iter().enumerate().all(|(i, n)| {
        (report[i] - n) * sign >= 1
            && (report[i] - n) * sign <= 3
            && (report[i] - n).signum() == sign
    })
}

fn parse_data<R: BufRead>(reader: R) -> Vec<Vec<i64>> {
    reader
        .lines()
        .map(|l| {
            l.expect("ERROR")
                .split_ascii_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect()
}

fn part1<R: BufRead>(reader: R) -> Result<usize> {
    let reports = parse_data(reader);
    let safe = reports.iter().filter(|x| is_safe(x)).count();
    Ok(safe)
}

fn part2<R: BufRead>(reader: R) -> Result<usize> {
    let reports = parse_data(reader);
    let mut safe = 0;
    for report in reports {
        if is_safe(&report) {
            safe += 1;
        } else {
            for i in 0..report.len() {
                let mut candidate = report.clone();
                candidate.remove(i);
                if is_safe(&candidate) {
                    safe += 1;
                    break;
                }
            }
        }
    }
    Ok(safe)
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");
    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    assert_eq!(4, part2(BufReader::new(TEST.as_bytes()))?);
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
