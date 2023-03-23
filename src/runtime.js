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

globalThis.console = console
