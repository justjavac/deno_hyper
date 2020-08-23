let pluginId = Deno.openPlugin("./target/debug/deno_hyper.dll");

//@ts-ignore
const { op_hyper_run } = Deno.core.ops();

export function run(): Promise<void> {
  //@ts-ignore
  Deno.core.dispatch(op_hyper_run);

  return new Promise(
    ((r) => {
      //@ts-ignore
      Deno.core.setAsyncHandler(op_hyper_run, (response) => {
        r();
      });
    }),
  );
}

export function close(): void {
  Deno.close(pluginId);
}
