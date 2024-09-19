/**
 * 1732. Find the Highest Altitude
 * https://leetcode.com/problems/find-the-highest-altitude/
 */

function largestAltitude(gain: number[]): number {
  let height: number = 0;
  let out: number = 0;

  gain.forEach((g: number) => {
    height += g;

    if (height > out) {
      out = height;
    }
  });

  return out;
}
