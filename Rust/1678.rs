/**
 * 1678. Goal Parser Interpretation
 * https://leetcode.com/problems/goal-parser-interpretation/submissions/1338178796/
 */

impl Solution {
  pub fn interpret(command: String) -> String {
    return command.replace("()", "o").replace("(al)", "al");
  }
}
