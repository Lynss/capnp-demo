mod implement;

use capnp_builder::test_capnp::greeter::ToClient;
use capnp_builder::RPC_ADDRESS;
use capnp_rpc::Server;
use capnp_rpc::{rpc_twoparty_capnp, twoparty, RpcSystem};
use futures::{Future, Stream};
use implement::GreeterImpl;
use std::error::Error;
use std::net::ToSocketAddrs;
use tokio_core::net::TcpListener;
use tokio_core::reactor::Core;
use tokio_io::AsyncRead;

pub fn run_rpc_server() -> Result<(), Box<dyn Error>> {
    let addr = RPC_ADDRESS
        .to_socket_addrs()
        .unwrap()
        .next()
        .expect("failed to parse address");

    let mut core = Core::new()?;
    let handle = core.handle();
    let socket = TcpListener::bind(&addr, &handle)?;

    let greeter = ToClient::new(GreeterImpl).into_client::<Server>();

    let done = socket.incoming().for_each(move |(socket, _)| {
        socket.set_nodelay(true)?;
        let (reader, writer) = socket.split();
        let network = twoparty::VatNetwork::new(
            reader,
            writer,
            rpc_twoparty_capnp::Side::Server,
            Default::default(),
        );
        let rpc_system = RpcSystem::new(Box::new(network), Some(greeter.clone().client));
        handle.spawn(rpc_system.map_err(|e| {
            eprintln!("failed to run the rpc system: {:?}", e);
        }));
        Ok(())
    });
    core.run(done).map_err(|e| e.into())
}

#[cfg(test)]
mod test {}
