/**
 * 2037. Minimum Number of Moves to Seat Everyone
 * https://leetcode.com/problems/minimum-number-of-moves-to-seat-everyone/
 */

impl Solution {
  pub fn min_moves_to_seat(seats: Vec<i32>, students: Vec<i32>) -> i32 {
    let mut seats = seats;
    let mut students = students;

    seats.sort();
    students.sort();

    let mut out: i32 = 0;

    for i in 0..seats.len() {
      out += (seats[i] - students[i]).abs();
    }

    return out;
  }
}
