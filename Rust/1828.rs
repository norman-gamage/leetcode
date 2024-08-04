/**
 * 1828. Queries on Number of Points Inside a Circle
 * https://leetcode.com/problems/queries-on-number-of-points-inside-a-circle/
 */

impl Solution {
  pub fn count_points(points: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut out: Vec<i32> = Vec::new();

    for q in queries.iter() {
      let mut cnt: i32 = 0;

      for p in points.iter() {
        if (p[0] - q[0]).pow(2) + (p[1] - q[1]).pow(2) <= q[2].pow(2) {
          cnt += 1;
        }
      }

      out.push(cnt);
    }

    return out;
  }
}
