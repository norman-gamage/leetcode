/**
 * 1678. Goal Parser Interpretation
 * https://leetcode.com/problems/goal-parser-interpretation/submissions/1338178796/
 */

function interpret(command: string): string {
  return command.replaceAll("()", "o").replaceAll("(al)", "al");
}
