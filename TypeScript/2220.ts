/**
 * 2220. Minimum Bit Flips to Convert Number
 * https://leetcode.com/problems/minimum-bit-flips-to-convert-number/
 */

function minBitFlips(start: number, goal: number): number {
  const countOnes = (n) => {
    return n === 0 ? n : (n & 1) + countOnes((n >>= 1));
  };

  return countOnes(start ^ goal);
}
