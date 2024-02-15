use crate::{Actor, Bus};
use act_identifiers::ActorId;
use anyhow::{Ok, Result};
use std::fmt::Debug;

pub struct System<A>
where
    A: ActorId + Debug,
{
    bus: Bus<A>,
}

impl<A> System<A>
where
    A: ActorId + Copy + Debug + Send + 'static,
{
    pub async fn spawn() -> Result<Self> {
        // create the dispatcher
        let mut bus = Bus::<A>::spawn().await.unwrap();

        // check the health of the dispatcher
        let health = bus.query_health().await.unwrap();
        println!("SYSTEM: bus health is: {health:?}");

        Ok(Self { bus })
    }

    pub async fn stop(&mut self) -> Result<()> {
        // Ask bus to stop.
        self.bus.stop().await.unwrap();

        Ok(())
    }

    pub async fn spawn_actor(&mut self, aid: A) -> Result<Actor<A>> {
        let actors_tx = self.bus.bus_tx();
        let actor = Actor::spawn(aid, actors_tx).await.unwrap();

        // Add the actor to the bus, so the bus can start sending messages to it.
        self.bus.add_actor(actor.aid, actor.tx());

        Ok(actor)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::time::Duration;
    use tokio::time::sleep;

    #[tokio::test]
    async fn run_and_quit() {
        const SLEEP: u64 = 1000;
        eprintln!("SYSTEM: Run and Quit starting ...");

        let mut system = System::<u8>::spawn().await.unwrap();

        sleep(Duration::from_millis(SLEEP)).await;

        eprintln!("SYSTEM: Creating an actor ...");

        let aid = 10_u8;
        let actor = system.spawn_actor(aid).await.unwrap();
        assert_eq!(actor.aid(), &aid);

        sleep(Duration::from_millis(SLEEP)).await;

        system.stop().await.unwrap();

        eprintln!("SYSTEM: Run and Quit ended!");
    }
}
