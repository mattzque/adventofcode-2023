use std::fs::read_to_string;

pub struct NumberIter<'a> {
    haystack: &'a str,
    index: usize,
}

impl<'a> NumberIter<'a> {
    const DIGITS: [&'static str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    pub fn new(haystack: &'a str) -> Self {
        Self { haystack, index: 0 }
    }
}

impl<'a> Iterator for NumberIter<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        // end of sequence?
        if self.index >= self.haystack.len() {
            None
        }
        else {
            // look for ascii digit at index
            let character = self.haystack[self.index..self.index+1].chars().next().unwrap();
            let number = if character.is_ascii_digit() {
                self.index += 1;
                Some(character.to_digit(10).unwrap() as u8)
            } else {
                // check for spelled-out words
                Self::DIGITS.iter().enumerate().find_map(|(index, word)| {
                    if self.haystack[self.index..].starts_with(word) {
                        self.index += word.len() - 1;
                        Some((index + 1) as u8)
                    } else {
                        None
                    }
                })
            };

            // recursively find the next number, skipping non-numbers in the haystack
            if number.is_some() {
                number
            } else {
                self.index += 1;
                self.next()
            }
        }
    }
}

#[test]
fn matcher_test() {
    assert_eq!(NumberIter::new("42").collect::<Vec<_>>(), [4, 2]);
    assert_eq!(NumberIter::new("foo42").collect::<Vec<_>>(), [4, 2]);
    assert_eq!(NumberIter::new("42foo").collect::<Vec<_>>(), [4, 2]);
    assert_eq!(NumberIter::new("4foo2").collect::<Vec<_>>(), [4, 2]);
    assert_eq!(NumberIter::new("foo4foo2").collect::<Vec<_>>(), [4, 2]);
    assert_eq!(NumberIter::new("4foo2foo").collect::<Vec<_>>(), [4, 2]);
    assert_eq!(NumberIter::new("foo4foo2foo").collect::<Vec<_>>(), [4, 2]);
    assert_eq!(NumberIter::new("4").collect::<Vec<_>>(), [4]);
    assert_eq!(NumberIter::new("foo4").collect::<Vec<_>>(), [4]);
    assert_eq!(NumberIter::new("4foo").collect::<Vec<_>>(), [4]);
    assert_eq!(NumberIter::new("foo4foo").collect::<Vec<_>>(), [4]);
    assert_eq!(NumberIter::new("foofourtwo").collect::<Vec<_>>(), [4, 2]);
    assert_eq!(NumberIter::new("4footwo").collect::<Vec<_>>(), [4, 2]);
    assert_eq!(NumberIter::new("onetwothreefour").collect::<Vec<_>>(), [1, 2, 3, 4]);
    assert_eq!(NumberIter::new("fivesixseveneightnine").collect::<Vec<_>>(), [5, 6, 7, 8, 9]);
}

/// Take the first and last number in line, concat, parse as an integer and return.
/// Both ascii digits (0-9) and spelled out words (one to nine) count as numbers.
/// Expects the line to contain at least one number, in which case it is repeated.
fn join_first_and_last_digits(line: &str) -> anyhow::Result<u8> {
    // collect all numbers in the line into an iterator:
    let mut iterator = NumberIter::new(line);

    // take first digit
    let first = iterator
        .next()
        .ok_or_else(|| anyhow::anyhow!("Invalid Input, Lines without digit"))?;

    // take last digit or repeat the first digit if there are no more digits
    let last = iterator.last().unwrap_or(first);

    // copy the two chars into a new string
    let string: String = format!("{}{}", first, last);

    // parse the string as a 8 bit unsigned integer
    let x: anyhow::Result<u8> = str::parse::<u8>(&string).map_err(|err| err.into());
    println!("{} -> {} => {}", line, string, x.unwrap());
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

    assert_eq!(join_first_and_last_digits("fourtwo").unwrap(), 42);
    assert_eq!(join_first_and_last_digits("foofourtwo").unwrap(), 42);
    assert_eq!(join_first_and_last_digits("fourtwofoo").unwrap(), 42);
    assert_eq!(join_first_and_last_digits("fourfootwo").unwrap(), 42);
    assert_eq!(join_first_and_last_digits("foofourfootwo").unwrap(), 42);
    assert_eq!(join_first_and_last_digits("fourfootwofoo").unwrap(), 42);
    assert_eq!(join_first_and_last_digits("foofourfootwofoo").unwrap(), 42);
    assert_eq!(join_first_and_last_digits("four").unwrap(), 44);
    assert_eq!(join_first_and_last_digits("foofour").unwrap(), 44);
    assert_eq!(join_first_and_last_digits("fourfoo").unwrap(), 44);
    assert_eq!(join_first_and_last_digits("foofourfoo").unwrap(), 44);

    assert_eq!(join_first_and_last_digits("two1nine").unwrap(), 29);
    assert_eq!(join_first_and_last_digits("eightwothree").unwrap(), 83);
    assert_eq!(join_first_and_last_digits("abcone2threexyz").unwrap(), 13);
    assert_eq!(join_first_and_last_digits("xtwone3four").unwrap(), 24);
    assert_eq!(join_first_and_last_digits("4nineeightseven2").unwrap(), 42);
    assert_eq!(join_first_and_last_digits("zoneight234").unwrap(), 14);
    assert_eq!(join_first_and_last_digits("7pqrstsixteen").unwrap(), 76);
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
///
/// --- Part Two ---
/// Your calculation isn't quite right. It looks like some of the digits
/// are actually spelled out with letters: one, two, three, four, five,
/// six, seven, eight, and nine also count as valid "digits".
///
/// Equipped with this new information, you now need to find the real
/// first and last digit on each line. For example:
///
/// two1nine
/// eightwothree
/// abcone2threexyz
/// xtwone3four
/// 4nineeightseven2
/// zoneight234
/// 7pqrstsixteen
/// In this example, the calibration values are 29, 83, 13, 24, 42,
/// 14, and 76. Adding these together produces 281.
///
fn main() {
    let contents = read_to_string("day1-input.txt").expect("Invalid Input!");

    let number = contents
        .split_terminator('\n')
        .map(|line| join_first_and_last_digits(line).unwrap())
        .fold(0u64, |acc, num| acc + num as u64);

    println!("Sum: {}", number);
}
