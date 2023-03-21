((globalThis) => {
  const { core } = Deno

  function argsToMessage(...args) {
    return args.map((arg) => JSON.stringify(arg)).join(" ");
  }

  globalThis.console = {
    log: (...args) => {
      core.print(`${argsToMessage(...args)}\n`, false);
    },
  };

  globalThis.shapes = {
    rect: (x, y) => {
      return core.ops.op_shapes_rect(x, y)
    },
  }
})(globalThis);
