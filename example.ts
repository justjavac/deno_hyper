import { run } from "./mod.ts";

const server = run();
console.log("server is running.");
await server;
