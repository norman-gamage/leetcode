/**
 * 1561. Maximum Number of Coins You Can Get
 * https://leetcode.com/problems/maximum-number-of-coins-you-can-get/
 */

function maxCoins(piles: number[]): number {
  piles = piles.sort((a: number, b: number) => a - b);
  let idx: number = piles.length / 3;
  let out: number = 0;

  while (idx < piles.length) {
    out += piles[idx];
    idx += 2;
  }

  return out;
}
