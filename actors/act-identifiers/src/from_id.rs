use crate::{ActorId, ToId};
use std::fmt::{Debug, Display};

pub struct FromId<A: ActorId>(pub(crate) A);

impl<A> FromId<A>
where
    A: ActorId,
{
    pub fn new(aid: A) -> Self {
        Self(aid)
    }
}

impl<A> Clone for FromId<A>
where
    A: ActorId + Clone,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

//
// Default
//

impl<A> Default for FromId<A>
where
    A: ActorId + Default,
{
    fn default() -> Self {
        Self(Default::default())
    }
}

//
// Debug + Display
//

impl<A> Debug for FromId<A>
where
    A: ActorId + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fid=[{:?}]", self.0)
    }
}

impl<A> Display for FromId<A>
where
    A: ActorId + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

//
// PartialEq, Eq, PartialOrd, Ord
//

impl<A> PartialEq for FromId<A>
where
    A: ActorId + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<A> Eq for FromId<A> where A: ActorId + Eq {}

impl<A> AsRef<A> for FromId<A>
where
    A: ActorId,
{
    fn as_ref(&self) -> &A {
        &self.0
    }
}

//
// From
//

impl<A> From<A> for FromId<A>
where
    A: ActorId,
{
    fn from(aid: A) -> Self {
        Self(aid)
    }
}

impl<A> From<ToId<A>> for FromId<A>
where
    A: ActorId,
{
    fn from(tid: ToId<A>) -> Self {
        tid.0.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use act_ids::IntoIdentifierIterator;

    #[test]
    fn new() {
        let fid = FromId::new("fid");
        assert_eq!(fid.as_ref().to_string(), "fid".to_string());
    }

    #[test]
    fn to_debug() {
        let fid = FromId::from(10_u32);
        assert_eq!(format!("{:?}", fid), "fid=[10]");
    }

    #[test]
    fn to_display() {
        let fid = FromId::from("fid");
        assert_eq!(fid.to_string(), "fid");
    }

    #[test]
    fn from_default() {
        let fid = FromId::<u32>::default();
        assert_eq!(fid.as_ref(), &0);
    }

    #[test]
    fn eq() {
        let fid1 = FromId::from("fid");
        let fid2 = FromId::from("fid");
        assert_eq!(fid1, fid2);
    }

    #[test]
    fn from_tid() {
        let tid = ToId::from("tid");
        let fid = FromId::<&str>::from(tid);
        assert_eq!(fid.0, "tid");
    }

    #[test]
    fn iter_ids_u32() {
        let id = FromId::from(10_u32);
        let mut iter = id.into_ids_iterator(|fid| (fid.0 + 1).into());
        assert_eq!(iter.next(), Some(10.into()));
        assert_eq!(iter.next(), Some(11.into()));
    }

    #[test]
    fn iter_ids_str() {
        let id = FromId::from("a");
        let mut iter = id.into_ids_iterator(|fid| fid.0.into());
        assert_eq!(iter.next(), Some("a".into()));
        assert_eq!(iter.next(), Some("a".into()));
    }
}
