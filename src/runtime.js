const { core } = Deno
const { ops } = core

function argsToMessage(...args) {
  return args.map((arg) => JSON.stringify(arg)).join(" ");
}

const console = {
  log: (...args) => {
    core.print(`${argsToMessage(...args)}\n`, false);
  },
};

const Sketch = {
  from_circle: (circle) => {
    return ops.op_sketch_from_circle(circle)
  }
}

const Circle = {
  from_radius: (radius) => {
    return ops.op_circle_from_radius(radius)
  }
}

globalThis.console = console
globalThis.Sketch = Sketch
globalThis.Circle = Circle
