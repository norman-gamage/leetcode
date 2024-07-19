/**
 * 771. Jewels and Stones
 * https://leetcode.com/problems/jewels-and-stones/
 */

function numJewelsInStones(jewels: string, stones: string): number {
  let out: number = 0;

  stones.split("").forEach((s) => {
    if (jewels.includes(s)) out += 1;
  });

  return out;
}
