/**
 * 3162. Find the Number of Good Pairs I
 * https://leetcode.com/problems/find-the-number-of-good-pairs-i/
 */

function numberOfPairs(nums1: number[], nums2: number[], k: number): number {
  let out: number = 0;
  nums2 = nums2.map((n: number) => k * n);

  for (let i: number = 0; i < nums2.length; i++) {
    if (nums2[i] === 1) {
      out += nums1.length;
      continue;
    }

    for (let j: number = 0; j < nums1.length; j++) {
      if (nums2[i] > nums1[j]) continue;
      if (nums1[j] % nums2[i] === 0) out++;
    }
  }

  return out;
}
