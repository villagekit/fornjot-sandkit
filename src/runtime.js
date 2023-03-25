const { core } = Deno
const { ops } = core

function argsToMessage(...args) {
  return args.map((arg) => JSON.stringify(arg)).join(" ")
}

const console = {
  log: (...args) => {
    core.print(`${argsToMessage(...args)}\n`, false)
  },
}

class Circle {
  constructor({ radius }) {
    this.radius = radius
  }

  static fromRadius(radius) {
    return new Circle(ops.op_circle_from_radius(radius))
  }
}

class Sketch {
  constructor({ chain, color }) {
    this.chain = chain
    this.color = color
  }

  static fromCircle(circle) {
    return new Sketch(ops.op_sketch_from_circle(circle))
  }
}

class Difference2d {
  constructor({ shapes }) {
    this.shapes = shapes
  }

  static fromShapes(a, b) {
    switch (true) {
      case a instanceof Sketch && b instanceof Sketch:
        return new Difference2d(ops.op_difference2d_from_shapes_sketch_sketch(a, b))
      case a instanceof Difference2d && b instanceof Sketch:
        return new Difference2d(ops.op_difference2d_from_shapes_difference2d_sketch(a, b))
      case a instanceof Sketch && b instanceof Difference2d:
        return new Difference2d(ops.op_difference2d_from_shapes_sketch_difference2d(a, b))
      case a instanceof Difference2d && b instanceof Difference2d:
        return new Difference2d(ops.op_difference2d_from_shapes_difference2d_difference2d(a, b))
    }
  }
}

class Sweep {
  constructor({ shape, path }) {
    this.shape = shape
    this.path = path
  }

  static fromPath(shape, path) {
    switch (true) {
      case shape instanceof Sketch:
        return new Sweep(ops.op_sweep_from_paths_sketch(shape, path))
      case shape instanceof Difference2d:
        return new Sweep(ops.op_sweep_from_paths_difference2d(shape, path))
    }
  }
}

globalThis.console = console
globalThis.Sketch = Sketch
globalThis.Circle = Circle
globalThis.Difference2d = Difference2d
globalThis.Sweep = Sweep
