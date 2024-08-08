/**
 * 2160. Minimum Sum of Four Digit Number After Splitting Digits
 * https://leetcode.com/problems/minimum-sum-of-four-digit-number-after-splitting-digits/
 */

impl Solution {
  pub fn minimum_sum(num: i32) -> i32 {
    let mut num = num;
    let mut arr: Vec<i32> = Vec::new();

    while num != 0 {
      arr.push(num % 10);
      num /= 10;
    }

    for i in 0..2 {
      for j in 2..4 {
        if arr[i] > arr[j] {
          [arr[i], arr[j]] = [arr[j], arr[i]];
        }
      }
    }

    return arr[0] * 10 + arr[2] + arr[1] * 10 + arr[3];
  }
}
