use capnp::capability::Promise;
use capnp::message::Builder;
use capnp::Error;
use capnp_builder::test_capnp::greeter::{SayHelloParams, SayHelloResults, Server};
use capnp_builder::test_capnp::hello_response as HelloResponse;

pub struct GreeterImpl;

impl Server for GreeterImpl {
    fn say_hello(
        &mut self,
        params: SayHelloParams,
        mut results: SayHelloResults,
    ) -> Promise<(), Error> {
        let name = params
            .get()
            .and_then(|request_wrapper| request_wrapper.get_request())
            .expect("failed to get the request")
            .get_name()
            .expect("failed to get the request name");
        let greet_message = format!("hello,{}", name);
        println!("ready to send '{}' to {}", &greet_message, &name);
        //build response struct
        let mut builder = Builder::new_default();
        let mut response = builder.init_root::<HelloResponse::Builder>();
        response.set_message(greet_message.as_str());
        results
            .get()
            .set_response(response.into_reader())
            .expect("failed to set the response");
        Promise::ok(())
    }
}
