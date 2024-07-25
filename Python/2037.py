# 2037. Minimum Number of Moves to Seat Everyone
# https://leetcode.com/problems/minimum-number-of-moves-to-seat-everyone/

class Solution:
  def minMovesToSeat(self, seats: List[int], students: List[int]) -> int:
    seats.sort()
    students.sort()

    out = 0

    for i in range(0, len(seats)):
      out += abs(seats[i] - students[i])

    return out
