use tokio_tungstenite::connect_async;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let addr: std::net::SocketAddr = "[::1]:3000".parse()?;

    let (ws_stream, _) = connect_async("ws://127.0.0.1:12345/")
        .await
        .expect("Failed to connect");
    println!("WebSocket handshake has been successfully completed");

    Ok(())
}
