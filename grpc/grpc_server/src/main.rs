use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};
use tonic::{transport::Server, Request, Response, Status};

pub mod hello_world {
    tonic::include_proto!("hello_world");
}

fn log_request(request: Request<()>) -> Result<Request<()>, Status> {
    println!("Incoming request: {:?}", request.metadata());
    Ok(request)
}

#[derive(Debug, Default)]
pub struct MyGreeter;
#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        let reply = hello_world::HelloReply {
            message: format!("Hello, {}!", request.into_inner().name),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = tracing_subscriber::fmt::Subscriber::builder().finish();
    tracing::subscriber::set_global_default(subscriber).expect("Setting global default failed");

    let addr = "[::1]:50051".parse()?;

    let greeter = MyGreeter::default();
    // let greeter_service = GreeterServer::new(greeter);
    let greeter_service = GreeterServer::with_interceptor(greeter, log_request);
    Server::builder()
        .add_service(greeter_service)
        .serve(addr)
        .await?;
    Ok(())
}
