use crate::Message;
use act_identifiers::ActorId;
use anyhow::Result;
use std::fmt::Debug;
use tokio::sync::mpsc::{channel, Receiver, Sender};

pub struct Actor<A>
where
    A: ActorId + Debug,
{
    pub(crate) aid: A,
    inbound_tx: Sender<Message<A>>,
}

impl<A> Actor<A>
where
    A: ActorId + Debug,
{
    pub fn aid(&self) -> &A {
        &self.aid
    }

    pub fn tx(&self) -> Sender<Message<A>> {
        self.inbound_tx.clone()
    }
}

impl<A> Actor<A>
where
    A: ActorId + Debug,
{
    async fn internal_loop(mut inbound_rx: Receiver<Message<A>>, _outbound_tx: Sender<Message<A>>) {
        loop {
            tokio::select! {
                msg = inbound_rx.recv() => {
                    match msg {
                        Some(_msg) => {
                            todo!("Actor inbound handle not implemented")
                        }
                        None => break,
                    }
                }
            }
        }
    }
}

impl<A> Actor<A>
where
    A: ActorId + Debug + Send + 'static,
{
    pub async fn spawn(aid: A, outbound_tx: Sender<Message<A>>) -> Result<Self> {
        let (inbound_tx, inbound_rx) = channel::<Message<A>>(10);

        tokio::spawn(Self::internal_loop(inbound_rx, outbound_tx));

        Ok(Self { aid, inbound_tx })
    }
}
