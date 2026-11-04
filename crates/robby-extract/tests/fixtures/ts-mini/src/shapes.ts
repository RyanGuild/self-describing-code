import { helper } from './util';
import { format } from 'demo-ext';

/** A drawable shape. */
export interface Drawable {
  draw(): void;
}

// Point on a canvas
export class Point implements Drawable {
  x = 0;
  draw(): void {
    helper(this.x);
    format(this.x);
  }
}

export function main(): void {
  const p = new Point();
  p.draw();
  if (p.x) {
    helper(1);
  }
}
