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
  type = "Circle"
  constructor({ radius }) {
    this.radius = radius
  }

  static fromRadius(radius) {
    return new Circle({ radius })
  }
}

class Sketch {
  type = "Sketch"
  constructor({ chain }) {
    this.chain = chain
  }

  static fromCircle(circle) {
    return new Sketch({ chain: circle })
  }
}

class Difference2d {
  type = "Difference2d"
  constructor({ shapes }) {
    this.shapes = shapes
  }

  static fromShapes(a, b) {
    return new Difference2d({ shapes: [a, b] })
  }
}

class Sweep {
  type = "Sweep"
  constructor({ shape, path }) {
    this.shape = shape
    this.path = path
  }

  static fromPath(shape, path) {
    return new Sweep({ shape, path })
  }
}

globalThis.console = console
globalThis.Sketch = Sketch
globalThis.Circle = Circle
globalThis.Difference2d = Difference2d
globalThis.Sweep = Sweep
