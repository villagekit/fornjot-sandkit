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

const shapes = {
  rect: (x, y) => {
    return ops.op_shapes_rect(x, y)
  },
}

globalThis.console = console
globalThis.shapes = shapes
