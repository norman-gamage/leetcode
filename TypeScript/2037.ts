/**
 * 2037. Minimum Number of Moves to Seat Everyone
 * https://leetcode.com/problems/minimum-number-of-moves-to-seat-everyone/
 */

function minMovesToSeat(seats: number[], students: number[]): number {
  seats = seats.sort((a: number, b: number) => a - b);
  students = students.sort((a: number, b: number) => a - b);

  let out: number = 0;

  for (let i: number = 0; i < seats.length; i++) {
    out += Math.abs(seats[i] - students[i]);
  }

  return out;
}
