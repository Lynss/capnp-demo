use capnp_builder::RPC_ADDRESS;
use std::net::ToSocketAddrs;
use tokio_core::net::TcpStream;
use tokio_core::reactor;

use capnp_rpc::{rpc_twoparty_capnp, twoparty, RpcSystem};

use capnp::message::Builder;
use capnp_builder::test_capnp::greeter;
use capnp_builder::test_capnp::hello_request as HelloRequest;
use futures::Future;
use tokio_io::AsyncRead;

pub fn main() {
    let name = "lynss";
    let mut core = reactor::Core::new().unwrap();
    let handle = core.handle();
    let addr = RPC_ADDRESS
        .to_socket_addrs()
        .unwrap()
        .next()
        .expect("could not parse address");
    let stream = core.run(TcpStream::connect(&addr, &handle)).unwrap();
    stream.set_nodelay(true).unwrap();
    let (reader, writer) = stream.split();
    let rpc_network = Box::new(twoparty::VatNetwork::new(
        reader,
        writer,
        rpc_twoparty_capnp::Side::Client,
        Default::default(),
    ));
    let mut rpc_system = RpcSystem::new(rpc_network, None);
    let greeter_client: greeter::Client = rpc_system.bootstrap(rpc_twoparty_capnp::Side::Server);
    handle.spawn(rpc_system.map_err(|e| eprintln!("failed to run the rpc_system client:{:?}", e)));

    //build request struct
    let mut builder = Builder::new_default();
    let mut request = builder.init_root::<HelloRequest::Builder>();
    request.set_name(&name);

    //set the request param and send the request
    let mut quest = greeter_client.say_hello_request();
    quest.get().set_request(request.into_reader()).unwrap();
    let response = core.run(quest.send().promise).unwrap();
    //receive the response
    let response = response
        .get()
        .expect("failed to get response")
        .get_response()
        .expect("failed to get response content");
    let message = response.get_message().expect("failed to get message");
    dbg!(message);
}
