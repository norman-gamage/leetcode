/**
 * 2535. Difference Between Element Sum and Digit Sum of an Array
 * https://leetcode.com/problems/difference-between-element-sum-and-digit-sum-of-an-array/
 */

impl Solution {
  pub fn difference_of_sum(nums: Vec<i32>) -> i32 {
    let mut element_sum = 0;
    let mut digit_sum = 0;

    for mut n in nums {
      element_sum += n;

      while n > 0 {
        digit_sum += n % 10;
        n /= 10;
      }
    }

    return element_sum - digit_sum;
  }
}
