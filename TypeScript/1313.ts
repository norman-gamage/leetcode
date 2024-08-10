/**
 * 1313. Decompress Run-Length Encoded List
 * https://leetcode.com/problems/decompress-run-length-encoded-list/
 */

function decompressRLElist(nums: number[]): number[] {
  const out: number[] = [];

  for (let i: number = 0; i < nums.length; i += 2) {
    for (let j: number = 0; j < nums[i]; j++) {
      out.push(nums[i + 1]);
    }
  }

  return out;
}
