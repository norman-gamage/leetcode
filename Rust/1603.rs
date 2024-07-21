/**
 * 1603. Design Parking System
 * https://leetcode.com/problems/design-parking-system/
 */

struct ParkingSystem {
  arr: Vec<i32>,
}

impl ParkingSystem {
  fn new(big: i32, medium: i32, small: i32) -> Self {
    ParkingSystem { arr: [big, medium, small].to_vec() }
  }

  fn add_car(&mut self, car_type: i32) -> bool {
    let car_type = (car_type as usize) - 1;

    if self.arr[car_type] == 0 {
      return false;
    }

    self.arr[car_type] -= 1;

    return true;
  }
}
