/**
 * 1656. Design an Ordered Stream
 * https://leetcode.com/problems/design-an-ordered-stream/
 */

class OrderedStream {
  private arr: string[];
  private idx: number;

  constructor(n: number) {
    this.arr = Array(n);
    this.idx = 0;
  }

  insert(idKey: number, value: string): string[] {
    this.arr[idKey - 1] = value;

    const out: string[] = [];

    while (this.idx < this.arr.length && this.arr[this.idx] !== undefined) {
      out.push(this.arr[this.idx]);
      this.idx += 1;
    }

    return out;
  }
}
