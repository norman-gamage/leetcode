/**
 * 2194. Cells in a Range on an Excel Sheet
 * https://leetcode.com/problems/cells-in-a-range-on-an-excel-sheet/
 */

function cellsInRange(s: string): string[] {
  const out: string[] = [];

  let row_beg: number = s.charCodeAt(0);
  let col_beg: number = Number(s[1]);
  let row_end: number = s.charCodeAt(3);
  let col_end: number = Number(s[4]);

  for (let i: number = row_beg; i <= row_end; i++) {
    for (let j: number = col_beg; j <= col_end; j++) {
      out.push(`${String.fromCharCode(i)}${j}`);
    }
  }

  return out;
}
