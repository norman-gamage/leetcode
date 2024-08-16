/**
 * 2194. Cells in a Range on an Excel Sheet
 * https://leetcode.com/problems/cells-in-a-range-on-an-excel-sheet/
 */

impl Solution {
  pub fn cells_in_range(s: String) -> Vec<String> {
    let s = s.chars().collect::<Vec<_>>();
    let mut out: Vec<String> = Vec::new();

    let row_beg: u8 = s[0] as u8;
    let col_beg: u8 = s[1].to_digit(10).unwrap() as u8;
    let row_end: u8 = s[3] as u8;
    let col_end: u8 = s[4].to_digit(10).unwrap() as u8;

    for i in row_beg..=row_end {
      for j in col_beg..=col_end {
        out.push(format!("{}{}", char::from(i), j));
      }
    }

    return out;
  }
}
