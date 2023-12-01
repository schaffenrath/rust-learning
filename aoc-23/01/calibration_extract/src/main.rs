/*
As they're making the final adjustments, they discover that their calibration document (your puzzle input) has been amended by a very young Elf who was apparently just excited to show off her art skills. Consequently, the Elves are having trouble reading the values on the document.

The newly-improved calibration document consists of lines of text; each line originally contained a specific calibration value that the Elves now need to recover. On each line, the calibration value can be found by combining the first digit and the last digit (in that order) to form a single two-digit number.

For example:

1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet

In this example, the calibration values of these four lines are 12, 38, 15, and 77. Adding these together produces 142.

Consider your entire calibration document. What is the sum of all of the calibration values?

Your puzzle answer was 53974.
--- Part Two ---

Your calculation isn't quite right. It looks like some of the digits are actually spelled out with letters: one, two, three, four, five, six, seven, eight, and nine also count as valid "digits".

Equipped with this new information, you now need to find the real first and last digit on each line. For example:

two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen

In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76. Adding these together produces 281.

What is the sum of all of the calibration values?

Your puzzle answer was 52840.
*/


use std::io::prelude::*;

fn get_str_of_file(filename: &str) -> String {
  use std::fs::File;
  use std::io::BufReader;
  // Right use of expect here?
  let file = File::open(filename).expect("Unable to open file {filename}");
  let mut buf_reader = BufReader::new(file);
  let mut calib_str = String::new();
  buf_reader.read_to_string(&mut calib_str).expect("Unable to read file {filename}");
  calib_str
}

// Must be an easier way to do this
fn extract_alpha_num_from_curr_pos(substr: &str) -> Option<i32> {
  let alpha_nums = vec!["one","two","three","four","five","six","seven","eight","nine"];
  for (idx, alpha_num) in alpha_nums.iter().enumerate() {
      let mut substr_idx = 0;
      for (idx2, matching) in alpha_num.chars().enumerate() {
          if substr_idx >= substr.len() || substr.chars().nth(substr_idx).unwrap() != matching {
            break;
          }
          if idx2 == alpha_num.len()-1 {
            return Some((idx+1).try_into().unwrap());
          }
          substr_idx += 1;
      }
  }
  None
}

// TODO: Refactor, too much nesting
fn extract_codes(calib_str: &str) -> Vec<i32> {
  let mut codes = Vec::new();
  for line in calib_str.lines() {
    let mut code_of_line = String::new();
    let mut latest_digit: Option<char> = None;
    for (idx, char) in line.chars().enumerate() {
      if char.is_numeric() {
        latest_digit = Some(char);
        if code_of_line.is_empty() {
          code_of_line.push(latest_digit.unwrap());
        }
        } else if let Some(val) = extract_alpha_num_from_curr_pos(&line[idx..]) {
        latest_digit = Some(val.to_string().chars().nth(0).unwrap());
          if code_of_line.is_empty() {
            code_of_line.push(latest_digit.unwrap());
          }
        }
    }
    if latest_digit != None {
        code_of_line.push(latest_digit.unwrap());
    }
    codes.push(code_of_line.parse::<i32>().unwrap());
  }
  codes
}

// TODO: try SIMD reduce
fn reduce(codes: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for num in codes {
        sum += num;
    }
    sum
}

fn main() -> std::io::Result<()> {
    // Relative file location to build target
    let content = get_str_of_file("../../calibration_code.txt");
    let codes = extract_codes(&content); 
    let sum = reduce(&codes);
    println!("Sum: {}", sum);
    Ok(())
}
