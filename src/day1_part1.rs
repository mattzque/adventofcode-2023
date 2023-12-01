use std::fs::read_to_string;

/// Take first and last digit in line, concat, parse as an integer and return.
/// Expects the line to contain at least one digit, in which case it is repeated.
fn join_first_and_last_digits(line: &str) -> anyhow::Result<u8> {
    // create iterator of only digit char's
    let mut iterator = line.chars().filter(|char| char.is_ascii_digit());

    // take first digit
    let first = iterator
        .next()
        .ok_or_else(|| anyhow::anyhow!("Invalid Input, Lines without digit"))?;

    // take last digit or repeat the first digit if there are no more digits
    let last = iterator.last().unwrap_or(first);

    // copy the two chars into a new string
    let string: String = [first, last].iter().collect();

    // parse the string as a 8 bit unsigned integer
    str::parse::<u8>(&string).map_err(|err| err.into())
}

#[test]
fn test_join_first_and_last_digits() {
    assert_eq!(join_first_and_last_digits("42").unwrap(), 42);
    assert_eq!(join_first_and_last_digits("foo42").unwrap(), 42);
    assert_eq!(join_first_and_last_digits("42foo").unwrap(), 42);
    assert_eq!(join_first_and_last_digits("4foo2").unwrap(), 42);
    assert_eq!(join_first_and_last_digits("foo4foo2").unwrap(), 42);
    assert_eq!(join_first_and_last_digits("4foo2foo").unwrap(), 42);
    assert_eq!(join_first_and_last_digits("foo4foo2foo").unwrap(), 42);
    assert_eq!(join_first_and_last_digits("4").unwrap(), 44);
    assert_eq!(join_first_and_last_digits("foo4").unwrap(), 44);
    assert_eq!(join_first_and_last_digits("4foo").unwrap(), 44);
    assert_eq!(join_first_and_last_digits("foo4foo").unwrap(), 44);
}

/// The newly-improved calibration document consists of lines of text;
/// each line originally contained a specific calibration value that the
/// Elves now need to recover. On each line, the calibration value
/// can be found by combining the first digit and the last digit
/// (in that order) to form a single two-digit number.
///
/// For example:
///
/// 1abc2
/// pqr3stu8vwx
/// a1b2c3d4e5f
/// treb7uchet
/// In this example, the calibration values of these four lines are
/// 12, 38, 15, and 77. Adding these together produces 142.
///
/// Consider your entire calibration document. What is the sum of all
/// of the calibration values?
fn main() {
    let contents = read_to_string("day1-input.txt").expect("Invalid Input!");

    let number = contents
        .split_terminator('\n')
        .map(|line| join_first_and_last_digits(line).unwrap())
        .fold(0u64, |acc, num| acc + num as u64);

    println!("Sum: {}", number);
}
