use act_ids::Identifier;

mod from_id;
mod msg_id;
mod to_id;

pub use from_id::*;
pub use msg_id::*;
pub use to_id::*;

pub trait ActorId: Identifier {}
impl<T: Identifier> ActorId for T {}
