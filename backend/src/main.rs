#![feature(async_await, futures_api,await_macro)]
extern crate actix_web;
extern crate actix_web_async_await;
extern crate capnp;
extern crate capnp_builder;
extern crate capnp_rpc;
extern crate env_logger;
extern crate futures;
extern crate listenfd;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio;

mod handler;

use actix_web::{http, middleware, server, App};
use actix_web_async_await::compat;
use http::Method;
use listenfd::ListenFd;
use middleware::Logger;
use std::env;

fn main() {
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let mut listenfd = ListenFd::from_env();
    let mut server = server::new(|| {
        App::new()
            .middleware(Logger::default())
            .resource("/greet/{name}", |r| {
                r.method(Method::GET).with_async(compat(handler::test))
            })
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)
    } else {
        server.bind("127.0.0.1:3001").unwrap()
    };

    server.run();
}
