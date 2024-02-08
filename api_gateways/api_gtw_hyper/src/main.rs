use apigtw_lib::{config::load_config, svc::Svc};
use hyper::server::conn::http1;
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Read the configuration for our service
    let cfg = load_config("config.yaml");

    // Start the TcpListener
    let addr: SocketAddr = ([127, 0, 0, 1], cfg.port).into();
    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);

    let svc = Svc::new();

    loop {
        // Accept new incoming connection
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        // Start processing the new incoming connection.
        let svc_clone = svc.clone();
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new().serve_connection(io, svc_clone).await {
                println!("Failed to serve connection: {:?}", err);
            }
        });
    }
}
