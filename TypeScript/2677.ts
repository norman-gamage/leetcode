/**
 * 2677. Chunk Array
 * https://leetcode.com/problems/chunk-array/
 */

type JSONValue =
  | null
  | boolean
  | number
  | string
  | JSONValue[]
  | { [key: string]: JSONValue };
type Obj = Record<string, JSONValue> | Array<JSONValue>;

function chunk(arr: Obj[], size: number): Obj[][] {
  let tmp: Obj[] = [];
  let out: Obj[][] = [];

  arr.forEach((v: Obj, i: number) => {
    tmp.push(v);

    if (tmp.length === size || i === arr.length - 1) {
      out.push(tmp);
      tmp = [];
    }
  });

  return out;
}
