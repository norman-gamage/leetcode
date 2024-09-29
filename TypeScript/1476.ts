/**
 * 1476. Subrectangle Queries
 * https://leetcode.com/problems/subrectangle-queries/
 */

class SubrectangleQueries {
  private rectangle: number[][] = [];

  constructor(rectangle: number[][]) {
    this.rectangle = rectangle;
  }

  updateSubrectangle(
    row1: number,
    col1: number,
    row2: number,
    col2: number,
    newValue: number
  ): void {
    for (let r: number = row1; r <= row2; r++) {
      for (let c: number = col1; c <= col2; c++) {
        this.rectangle[r][c] = newValue;
      }
    }
  }

  getValue(row: number, col: number): number {
    return this.rectangle[row][col];
  }
}
