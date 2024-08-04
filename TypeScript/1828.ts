/**
 * 1828. Queries on Number of Points Inside a Circle
 * https://leetcode.com/problems/queries-on-number-of-points-inside-a-circle/
 */

function countPoints(points: number[][], queries: number[][]): number[] {
  const out: number[] = [];

  queries.forEach((q: number[]) => {
    let cnt: number = 0;

    points.forEach((p: number[]) => {
      if ((p[0] - q[0]) ** 2 + (p[1] - q[1]) ** 2 <= q[2] ** 2) cnt += 1;
    });

    out.push(cnt);
  });

  return out;
}
