use tokio::sync::oneshot::Sender;

#[derive(Debug)]
pub enum HealthResponse {
    Healty,
}

#[derive(Debug)]
pub struct QuitResponse {}

#[derive(Debug)]
pub enum CtrlMessage {
    Quit(Sender<QuitResponse>),
    Health(Sender<HealthResponse>),
}
