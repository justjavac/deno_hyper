let pluginId = Deno.openPlugin("./target/debug/deno_hyper.dll");

//@ts-ignore
const { op_hyper_run } = Deno.core.ops();

//@ts-ignore
Deno.core.setAsyncHandler(op_hyper_run, (response) => {
  console.log(`Plugin Async Response: `, response);
});

export function run(): Promise<void> {
  //@ts-ignore
  Deno.core.dispatch(
    op_hyper_run,
    new Uint8Array([116, 101, 115, 116]),
    new Uint8Array([49, 50, 51]),
  )!;
  
  return new Promise((r => {
    //@ts-ignore
    Deno.core.setAsyncHandler(op_hyper_run, (response) => {
      r();
    });
  }))
}

export function close(): void {
  Deno.close(pluginId);
}
