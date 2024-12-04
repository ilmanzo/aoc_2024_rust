use adv_code_2024::{read_input, start_day, string_to_grid};
use anyhow::{Ok, Result};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "04";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");
const TEST: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

fn part1<R: BufRead>(reader: R) -> Result<usize> {
    const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];
    const DIRECTIONS: [(i32, i32); 8] = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, -1),
        (-1, 1),
        (1, 1),
        (-1, -1),
    ];
    let input = read_input(reader)?;
    let g = string_to_grid(&input);
    let width = g[0].len();
    let height = g.len();
    let mut result = 0;
    for y in 0..height {
        for x in 0..width {
            if g[y][x] == XMAS[0] {
                for d in &DIRECTIONS {
                    let mut found = true;
                    for (j, w) in XMAS.iter().skip(1).enumerate() {
                        let nx = i32::try_from(x)? + d.0 * (i32::try_from(j)? + 1);
                        let ny = i32::try_from(y)? + d.1 * (i32::try_from(j)? + 1);
                        if nx < 0
                            || ny < 0
                            || nx >= i32::try_from(width)?
                            || ny >= i32::try_from(height)?
                            || g[ny as usize][nx as usize] != *w
                        {
                            found = false;
                            break;
                        }
                    }
                    if found {
                        result += 1;
                    }
                }
            }
        }
    }
    Ok(result)
}

fn part2<R: BufRead>(reader: R) -> Result<usize> {
    let input = read_input(reader)?;
    let g = string_to_grid(&input);
    let width = g[0].len();
    let height = g.len();
    let mut result = 0;
    for y in 0..height {
        for x in 0..width {
            if g[y][x] == 'A' && x > 0 && y > 0 && x < width - 1 && y < height - 1 {
                let c1 = g[y - 1][x - 1];
                let c2 = g[y + 1][x + 1];
                let c3 = g[y - 1][x + 1];
                let c4 = g[y + 1][x - 1];
                if ((c1 == 'M' && c2 == 'S') || (c1 == 'S' && c2 == 'M'))
                    && ((c3 == 'M' && c4 == 'S') || (c3 == 'S' && c4 == 'M'))
                {
                    result += 1;
                }
            }
        }
    }
    Ok(result)
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");
    assert_eq!(18, part1(BufReader::new(TEST.as_bytes()))?);
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {result}");
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    assert_eq!(9, part2(BufReader::new(TEST.as_bytes()))?);
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {result}");
    //endregion

    Ok(())
}
