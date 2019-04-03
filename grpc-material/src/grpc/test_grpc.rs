// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]


// interface

pub trait Greeter {
    fn say_hello(&self, o: ::grpc::RequestOptions, p: super::test::HelloRequest) -> ::grpc::SingleResponse<super::test::HelloResponse>;
}

// client

pub struct GreeterClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
    method_sayHello: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::test::HelloRequest, super::test::HelloResponse>>,
}

impl ::grpc::ClientStub for GreeterClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        GreeterClient {
            grpc_client: grpc_client,
            method_sayHello: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/helloword.Greeter/sayHello".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }
}

impl Greeter for GreeterClient {
    fn say_hello(&self, o: ::grpc::RequestOptions, p: super::test::HelloRequest) -> ::grpc::SingleResponse<super::test::HelloResponse> {
        self.grpc_client.call_unary(o, p, self.method_sayHello.clone())
    }
}

// server

pub struct GreeterServer;


impl GreeterServer {
    pub fn new_service_def<H : Greeter + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/helloword.Greeter",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/helloword.Greeter/sayHello".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.say_hello(o, p))
                    },
                ),
            ],
        )
    }
}
