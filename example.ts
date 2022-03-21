import { execute } from "./mod.ts";

await execute("ls");
await execute("pwd");
await execute("ls -l | wc -l");
