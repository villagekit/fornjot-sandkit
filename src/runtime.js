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
  constructor(fj) {
    this.fj = fj
  }

  static fromRadius(radius) {
    return new Sketch(ops.op_circle_from_radius(radius))
  }
}

class Sketch {
  constructor(fj) {
    this.fj = fj
  }

  static fromCircle(circle) {
    return new Sketch(ops.op_sketch_from_circle(circle.fj))
  }
}

class Difference2d {
  constructor(fj) {
    this.fj = fj
  }

  static fromShapes(a, b) {
    switch (true) {
      case a instanceof Sketch && b instanceof Sketch:
        return new Difference2d(ops.op_difference2d_from_shapes_sketch_sketch(a.fj, b.fj))
      case a instanceof Difference2d && b instanceof Sketch:
        return new Difference2d(ops.op_difference2d_from_shapes_difference2d_sketch(a.fj, b.fj))
      case a instanceof Sketch && b instanceof Difference2d:
        return new Difference2d(ops.op_difference2d_from_shapes_sketch_difference2d(a.fj, b.fj))
      case a instanceof Difference2d && b instanceof Difference2d:
        return new Difference2d(ops.op_difference2d_from_shapes_difference2d_difference2d(a.fj, b.fj))
    }
  }
}

class Sweep {
  constructor(fj) {
    this.fj = fj
  }

  static fromPath(shape, path) {
    switch (true) {
      case shape instanceof Sketch:
        return new Sweep(ops.op_sweep_from_paths_sketch(shape.fj, path))
      case shape instanceof Difference2d:
        return new Sweep(ops.op_sweep_from_paths_difference2d(shape.fj, path))
    }
  }
}

globalThis.console = console
globalThis.Sketch = Sketch
globalThis.Circle = Circle
globalThis.Difference2d = Difference2d
globalThis.Sweep = Sweep
