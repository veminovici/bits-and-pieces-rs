use std::fmt::Debug;

use act_identifiers::{ActorId, FromId, ToId};

#[derive(Debug)]
pub struct Message<A>
where
    A: ActorId + Debug,
{
    fid: FromId<A>,
    tid: ToId<A>,
}

impl<A> Message<A>
where
    A: ActorId + Debug,
{
    pub fn new<F, T>(f: F, t: T) -> Self
    where
        F: Into<FromId<A>>,
        T: Into<ToId<A>>,
    {
        Self {
            fid: f.into(),
            tid: t.into(),
        }
    }

    pub fn fid(&self) -> &FromId<A> {
        &self.fid
    }

    pub fn tid(&self) -> &ToId<A> {
        &self.tid
    }
}
