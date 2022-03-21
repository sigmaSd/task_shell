import { Bolt, Crate } from "https://deno.land/x/bolt@0.1.7/src/bolt.ts";

const taskShell: Crate = {
  name: "task_shell_wrap",
  path: "./task_shell_wrap",
};

const bolt = new Bolt([taskShell]);
await bolt.init();

const taskShellLib = Deno.dlopen(bolt.getLib("task_shell_wrap"), {
  "execute": {
    parameters: ["pointer", "usize"],
    result: "void",
    nonblocking: true,
  },
});

export async function execute(script: string) {
  await taskShellLib.symbols.execute(
    new TextEncoder().encode(script),
    script.length,
  );
}
