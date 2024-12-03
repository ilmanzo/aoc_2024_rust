use std::io::{BufRead, BufReader, Read,Error};

pub fn start_day(day: &str) {
    println!("Advent of Code 2024 - Day {day:0>2}");
}

/// Reads the contents of a file into a string.
///
/// # Errors
///
/// Returns an error if the file cannot be opened or read.
/// 
pub fn read_input<R: BufRead>(reader: R) -> Result<String,Error> {
    let mut reader = BufReader::new(reader);
    let mut buffer = String::new();
    let _bytes=reader.read_to_string(&mut buffer)?;
    Ok(buffer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        start_day("00");
    }
}
