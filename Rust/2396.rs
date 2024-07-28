/**
 * 2396. Strictly Palindromic Number
 * https://leetcode.com/problems/strictly-palindromic-number/
 */

fn format_with_radix(mut n: u32, radix: u32) -> String {
  let mut result = String::new();

  loop {
    result.push(std::char::from_digit(n % radix, radix).unwrap());
    n /= radix;
    if n == 0 {
      break;
    }
  }

  return result.chars().rev().collect();
}

impl Solution {
  pub fn is_strictly_palindromic(n: i32) -> bool {
    for i in 2..n - 1 {
      let str = format_with_radix(n as u32, i as u32);

      for j in 0..str.len() / 2 + 1 {
        if Some(str.chars().nth(j)) != Some(str.chars().nth(str.len() - 1 - j)) {
          return false;
        }
      }
    }

    return true;
  }
}
