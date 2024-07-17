/**
 * 1689. Partitioning Into Minimum Number Of Deci-Binary Numbers
 * https://leetcode.com/problems/partitioning-into-minimum-number-of-deci-binary-numbers/
 */

function minPartitions(n: string): number {
  return Math.max(...n.split("").map(Number));
}
