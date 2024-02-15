use crate::{ActorId, FromId};
use std::fmt::{Debug, Display};

pub struct ToId<A: ActorId>(pub(crate) A);

impl<A> ToId<A>
where
    A: ActorId,
{
    pub fn new(aid: A) -> Self {
        Self(aid)
    }
}

impl<A> Clone for ToId<A>
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

impl<A> Default for ToId<A>
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

impl<A> Debug for ToId<A>
where
    A: ActorId + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "tid=[{:?}]", self.0)
    }
}

impl<A> Display for ToId<A>
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

impl<A> PartialEq for ToId<A>
where
    A: ActorId + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<A> Eq for ToId<A> where A: ActorId + Eq {}

impl<A> AsRef<A> for ToId<A>
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

impl<A> From<A> for ToId<A>
where
    A: ActorId,
{
    fn from(aid: A) -> Self {
        Self(aid)
    }
}

impl<A> From<FromId<A>> for ToId<A>
where
    A: ActorId,
{
    fn from(value: FromId<A>) -> Self {
        value.0.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use act_ids::IntoIdentifierIterator;

    #[test]
    fn new() {
        let tid = ToId::new("tid");
        assert_eq!(tid.to_string(), "tid".to_string());
    }

    #[test]
    fn to_debug() {
        let tid = ToId::from(10_u32);
        assert_eq!(format!("{:?}", tid), "tid=[10]");
    }

    #[test]
    fn to_display() {
        let tid = ToId::from("tid");
        assert_eq!(tid.to_string(), "tid");
    }

    #[test]
    fn from_default() {
        let tid = ToId::<u32>::default();
        assert_eq!(tid.as_ref(), &0);
    }

    #[test]
    fn eq() {
        let tid1 = ToId::from("tid");
        let tid2 = ToId::from("tid");
        assert_eq!(tid1, tid2);
    }

    #[test]
    fn from_fid() {
        let fid = FromId::from("fid");
        let tid = ToId::<&str>::from(fid);
        assert_eq!(tid.0, "fid");
    }

    #[test]
    fn iter_ids_u32() {
        let id = ToId::from(10_u32);
        let mut iter = id.into_ids_iterator(|fid| (fid.0 + 1).into());
        assert_eq!(iter.next(), Some(10.into()));
        assert_eq!(iter.next(), Some(11.into()));
    }

    #[test]
    fn iter_ids_str() {
        let id = ToId::from("a");
        let mut iter = id.into_ids_iterator(|fid| fid.0.into());
        assert_eq!(iter.next(), Some("a".into()));
        assert_eq!(iter.next(), Some("a".into()));
    }
}
