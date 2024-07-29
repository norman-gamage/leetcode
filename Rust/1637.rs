/**
 * 1637. Widest Vertical Area Between Two Points Containing No Points
 * https://leetcode.com/problems/widest-vertical-area-between-two-points-containing-no-points/
 */

impl Solution {
  pub fn max_width_of_vertical_area(points: Vec<Vec<i32>>) -> i32 {
    let mut arr: Vec<i32> = points
      .into_iter()
      .map(|a| a[0])
      .collect();
    arr.sort();
    let mut max = 0;

    for i in 1..arr.len() {
      if arr[i] == arr[i - 1] {
        continue;
      }

      max = std::cmp::max(max, arr[i] - arr[i - 1]);
    }

    return max;
  }
}
