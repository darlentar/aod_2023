use std::fs::File;
use std::io::{self, BufRead};

fn day_1_1(filename: &str) -> io::Result<u32> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    Ok(reader
        .lines()
        .map(|line| {
            let first_digit = line
                .as_ref()
                .unwrap()
                .as_bytes()
                .iter()
                .find(|c| c.is_ascii_digit())
                .map(|c| (c - b'0') * 10)
                .unwrap();

            let last_digit = line
                .as_ref()
                .unwrap()
                .as_bytes()
                .iter()
                .rfind(|c| c.is_ascii_digit())
                .map(|c| c - b'0')
                .unwrap();

            u32::from(first_digit + last_digit)
        })
        .sum())
}

const NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn find_number(slice: &str) -> Option<usize> {
    NUMBERS
        .iter()
        .enumerate()
        .find_map(|(i, n)| slice.starts_with(*n).then_some(i + 1))
        .or({
            let c = slice.as_bytes()[0];
            c.is_ascii_digit().then_some((c - b'0').into())
        })
}

fn day_1_2(filename: &str) -> io::Result<u32> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    Ok(reader
        .lines()
        .map(|line| {
            let line = line.unwrap();

            let first_digit = (0..line.len())
                .find_map(|i| find_number(&line[i..]))
                .unwrap() as u8;

            let last_digit = (0..line.len())
                .rev()
                .find_map(|i| find_number(&line[i..]))
                .unwrap() as u8;

            u32::from(10 * first_digit + last_digit)
        })
        .sum())
}
fn main() -> io::Result<()> {
    println!("{}", day_1_1("../input1.txt")?);
    println!("{}", day_1_2("../input1.txt")?);
    Ok(())
}
