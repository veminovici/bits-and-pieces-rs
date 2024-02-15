use crate::ActorId;
use act_ids::Identifier;
use std::fmt::{Debug, Display};

pub struct MsgId<A, I>
where
    A: ActorId,
    I: Identifier,
{
    aid: A,
    mid: I,
}

impl<A, I> MsgId<A, I>
where
    A: ActorId,
    I: Identifier + Default,
{
    pub fn new(aid: A) -> Self {
        Self {
            aid,
            mid: Default::default(),
        }
    }

    pub fn aid(&self) -> &A {
        &self.aid
    }

    pub fn mid(&self) -> &I {
        &self.mid
    }
}

impl<A, I> Clone for MsgId<A, I>
where
    A: ActorId + Clone,
    I: Identifier + Clone,
{
    fn clone(&self) -> Self {
        Self {
            aid: self.aid.clone(),
            mid: self.mid.clone(),
        }
    }
}

//
// Debug + Display
//

impl<A, I> Debug for MsgId<A, I>
where
    A: ActorId + Debug,
    I: Identifier + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "mid=({:?}:{:?})", self.aid, self.mid)
    }
}

impl<A, I> Display for MsgId<A, I>
where
    A: ActorId + Display,
    I: Identifier + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}:{})", self.aid, self.mid)
    }
}

//
// PartialEq, Eq, PartialOrd, Ord
//

impl<A, I> PartialEq for MsgId<A, I>
where
    A: ActorId + PartialEq,
    I: Identifier + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.aid == other.aid && self.mid == other.mid
    }
}

impl<A, I> Eq for MsgId<A, I>
where
    A: ActorId + Eq,
    I: Identifier + Eq,
{
}

#[cfg(test)]
mod tests {
    use act_ids::IntoIdentifierIterator;

    use super::*;

    #[test]
    fn new() {
        let aid = "a";
        let mid = MsgId::<_, u8>::new(aid);
        assert_eq!(mid.aid, aid);
        assert_eq!(mid.mid, 0_u8);
    }

    #[test]
    fn to_debug() {
        let mid = MsgId::<_, u8>::new("A");
        assert_eq!(format!("{mid:?}"), "mid=(\"A\":0)")
    }

    #[test]
    fn to_display() {
        let mid = MsgId::<_, u8>::new("A");
        assert_eq!(mid.to_string(), "(A:0)")
    }

    #[test]
    fn eq() {
        let mid1 = MsgId::<_, u8>::new("A");
        let mid2 = MsgId::<_, u8>::new("A");
        assert_eq!(mid1, mid2)
    }

    #[test]
    fn iter_ids_u32() {
        let mid = MsgId::<_, u32>::new("A");
        let mut iter = mid.into_ids_iterator(|mid| {
            let aid = mid.aid;
            let mid = mid.mid + 1;
            MsgId { aid, mid }
        });

        assert_eq!(iter.next(), Some(MsgId { aid: "A", mid: 0 }));
        assert_eq!(iter.next(), Some(MsgId { aid: "A", mid: 1 }));
    }
}
