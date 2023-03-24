declare global {
  interface SketchInstance {}
  interface CircleInstance {}

  interface SketchClass {
    from_circle: (circle: CircleInstance) => SketchInstance
  }
  interface CircleClass {
    from_radius: (radius: number) => CircleInstance
  }
  const Sketch: SketchClass
  const Circle: CircleClass
}

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
    min: ({ outer }) => outer * 0.99,
  },
  {
    name: 'height',
    type: 'number',
    default: 1.0,
  },
]

export function shape(params) {
  const outer = 1
  const inner = 0.5
  const height = 1
  // const { outer, inner, height } = params
  
  return Sketch.from_circle(Circle.from_radius(outer))

  /*
  const outer_edge = Sketch.from_circle(Circle.from_radius(outer))
  const inner_edge = Sketch.from_circle(Circle.from_radius(inner))

  const footprint = outer_edge.difference(inner_edge)
  const spacer = footprint.sweep([0, 0, height])

  return spacer
  */
}
