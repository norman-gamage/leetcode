# 1603. Design Parking System
# https://leetcode.com/problems/design-parking-system/

class ParkingSystem:
  __arr = []

  def __init__(self, big: int, medium: int, small: int):
    self.arr = [big, medium, small]

  def addCar(self, carType: int) -> bool:
    if (self.arr[carType - 1] == 0):
      return False

    self.arr[carType - 1] -= 1

    return True
