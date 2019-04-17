extern crate capnp;
extern crate capnp_builder;
extern crate capnp_rpc;
extern crate futures;
extern crate tokio;
extern crate tokio_core;
extern crate tokio_io;

use capnp_builder::RPC_ADDRESS;
use service::run_rpc_server;

fn main() {
    println!("starting to bootstrap the rpc server");
    match run_rpc_server() {
        Ok(_) => println!("successfully running rpc server in {}", RPC_ADDRESS),
        Err(e) => {
            eprintln!("failed to run the rpc server,caused by :{:?}", e);
        }
    };
}
