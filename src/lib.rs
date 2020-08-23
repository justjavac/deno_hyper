extern crate deno_core;
extern crate futures;
extern crate hyper;

use deno_core::plugin_api::{Buf, Interface, Op, ZeroCopyBuf};

use std::convert::Infallible;
use std::net::SocketAddr;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};

use futures::future::FutureExt;

#[no_mangle]
pub fn deno_plugin_init(interface: &mut dyn Interface) {
  interface.register_op("op_hyper_run", op_run);
}

fn op_run(
  _interface: &mut dyn Interface,
  _zero_copy: &mut [ZeroCopyBuf],
) -> Op {
  let fut = async move {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
      // We'll bind to 127.0.0.1:3000
      let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

      // A `Service` is needed for every connection, so this
      // creates one from our `hello_world` function.
      let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(hello_world))
      });

      // Then bind and serve...
      let server = Server::bind(&addr).serve(make_svc);

      // Run forever-ish...
      if let Err(err) = server.await {
        eprintln!("server error: {}", err);
      }
    });
    Box::new([]) as Buf
  };

  Op::Async(fut.boxed())
}

async fn hello_world(
  _req: Request<Body>,
) -> Result<Response<Body>, Infallible> {
  Ok(Response::new("Hello, World".into()))
}
