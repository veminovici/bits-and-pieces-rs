use crate::{CtrlMessage, HealthResponse, Message, QuitResponse};
use act_identifiers::ActorId;
use anyhow::{anyhow, Result};
use std::fmt::Debug;
use tokio::sync::mpsc::{channel, Receiver, Sender};

pub struct Bus<A>
where
    A: ActorId + Debug,
{
    ctrl_tx: Sender<CtrlMessage>,
    actors_tx: Sender<Message<A>>,
}

impl<A> Bus<A>
where
    A: ActorId + Debug,
{
    pub fn bus_tx(&self) -> Sender<Message<A>> {
        self.actors_tx.clone()
    }

    async fn internal_loop(
        mut ctrl_rx: Receiver<CtrlMessage>,
        mut actors_rx: Receiver<Message<A>>,
    ) {
        loop {
            tokio::select! {
                msg = ctrl_rx.recv() =>
                    match msg {
                        Some(CtrlMessage::Quit(tx)) => {
                            tx.send(QuitResponse {  }).unwrap();
                            break
                        },
                        Some(CtrlMessage::Health(tx)) => {
                            tx.send(HealthResponse::Healty).unwrap();
                        },
                        None => break, // all senders have dropped
                    },
                _msg = actors_rx.recv() => {
                    todo!("Handler for the actors message not implemented yet!")
                }
            }
        }
    }

    async fn send_ctrl(&mut self, ctrl_msg: CtrlMessage) -> Result<()> {
        self.ctrl_tx
            .send(ctrl_msg)
            .await
            .map_err(|_e| anyhow!("Failed to send the control message"))
    }

    pub async fn query_health(&mut self) -> Result<HealthResponse> {
        let (health_tx, health_rx) = tokio::sync::oneshot::channel::<HealthResponse>();

        self.send_ctrl(CtrlMessage::Health(health_tx))
            .await
            .unwrap();

        health_rx
            .await
            .map_err(|e| anyhow!("Failed to receive the health of the bus [{e}]"))
    }

    pub async fn stop(&mut self) -> Result<()> {
        let (quit_tx, quit_rx) = tokio::sync::oneshot::channel::<QuitResponse>();
        self.send_ctrl(CtrlMessage::Quit(quit_tx)).await.unwrap();
        quit_rx
            .await
            .map(|_| ())
            .map_err(|e| anyhow!("Failed to get the quit response [{e}]"))
    }

    pub fn add_actor(&mut self, _aid: A, _tx: Sender<Message<A>>) {}
}

impl<A> Bus<A>
where
    A: ActorId + Debug + Send + 'static,
{
    pub async fn spawn() -> Result<Self> {
        let (ctrl_tx, ctrl_rx) = channel::<CtrlMessage>(10);
        let (actors_tx, actors_rx) = channel::<Message<A>>(10);

        tokio::spawn(Self::internal_loop(ctrl_rx, actors_rx));

        Ok(Self { ctrl_tx, actors_tx })
    }
}
