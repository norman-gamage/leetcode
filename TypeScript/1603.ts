/**
 * 1603. Design Parking System
 * https://leetcode.com/problems/design-parking-system/
 */

class ParkingSystem {
  private arr: number[];

  constructor(big: number, medium: number, small: number) {
    this.arr = [big, medium, small];
  }

  addCar(carType: number): boolean {
    if (this.arr[carType - 1] === 0) return false;

    this.arr[carType - 1] -= 1;

    return true;
  }
}
