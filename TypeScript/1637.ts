/**
 * 1637. Widest Vertical Area Between Two Points Containing No Points
 * https://leetcode.com/problems/widest-vertical-area-between-two-points-containing-no-points/
 */

function maxWidthOfVerticalArea(points: number[][]): number {
  const arr: number[] = points
    .map((a: number[]) => a[0])
    .sort((a: number, b: number) => a - b);
  let max: number = 0;

  for (let i: number = 1; i < arr.length; i++) {
    if (arr[i] === arr[i - 1]) continue;
    max = Math.max(max, arr[i] - arr[i - 1]);
  }

  return max;
}
