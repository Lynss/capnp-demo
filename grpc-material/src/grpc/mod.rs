mod test_grpc;
mod test;

pub mod pojo {
    pub use test::*;
}

pub use test_grpc::*;