/**
 * 2956. Find Common Elements Between Two Arrays
 * https://leetcode.com/problems/find-common-elements-between-two-arrays/
 */

function findIntersectionValues(nums1: number[], nums2: number[]): number[] {
  const ans1: number = nums1.filter((n: number) => nums2.includes(n)).length;
  const ans2: number = nums2.filter((n: number) => nums1.includes(n)).length;
  return [ans1, ans2];
}
