export const params = [
  {
    name: 'outer',
    type: 'number',
    default: 1.0,
    min: ({ inner }) => inner * 1.01,
  },
  {
    name: 'inner',
    type: 'number',
    default: 0.5,
    max: ({ outer }) => outer * 0.99,
  },
  {
    name: 'height',
    type: 'number',
    default: 1.0,
  },
]
export function shape(params: ParamValues) {
  const { outer, inner, height } = params
  
  const outerEdge = Sketch.fromCircle(Circle.fromRadius(outer))
  const innerEdge = Sketch.fromCircle(Circle.fromRadius(inner))

  const footprint = Difference2d.fromShapes(outerEdge, innerEdge)
  const spacer = Sweep.fromPath(footprint, [0, 0, height])

  console.log('JavaScript shape:', spacer)

  return spacer
}

/* type helpers */

// TODO derive from param definition
interface ParamValues {
  outer: number
  inner: number
  height: number
}

declare global {
  export class Circle {
    static fromRadius: (radius: number) => Circle
  }
  export class Sketch {
    static fromCircle: (circle: Circle) => Sketch
  }
  interface Difference2dFromShapes {
    (a: Sketch, b: Sketch): Difference2d
    (a: Difference2d, b: Sketch): Difference2d
    (a: Sketch, b: Difference2d): Difference2d
    (a: Difference2d, b: Difference2d): Difference2d
  }
  export class Difference2d {
    static fromShapes: Difference2dFromShapes
  }
  interface SweepFromPath {
    (shape: Sketch, path: Array<number>): Sweep
    (shape: Difference2d, path: Array<number>): Sweep
  }
  export class Sweep {
    static fromPath: SweepFromPath
  }
}

