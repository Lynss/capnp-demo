use grpc_material::pojo::{HelloRequest,HelloResponse};
use grpc_material::Greeter;

pub struct TestImpl;

impl Greeter for TestImpl{
    fn say_hello(&self, o: ::grpc::RequestOptions, httpRequest: HelloRequest) -> HelloResponse{

    }
}