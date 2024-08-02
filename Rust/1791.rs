/**
 * 1791. Find Center of Star Graph
 * https://leetcode.com/problems/find-center-of-star-graph/
 */

impl Solution {
  pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
    return if edges[0][0] == edges[1][0] || edges[0][0] == edges[1][1] {
      edges[0][0]
    } else {
      edges[0][1]
    };
  }
}
