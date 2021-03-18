//! A crate providing advanced and general features for games.
//! It can be used just as much for simple ascii games than for full distributed mmorpg games.

#![deny(missing_docs)]

#[macro_use]
extern crate serde;
#[macro_use]
extern crate derive_new;
#[macro_use]
extern crate derive_builder;

mod faction;
mod item;
mod item_transition;
mod loot_tree;
mod permissions;
mod skill;
mod stat;
mod statistics;
mod tier;
mod unlock;
mod user;
mod user_group;
mod user_management;

pub use self::faction::*;
pub use self::item::*;
pub use self::item_transition::*;
pub use self::loot_tree::*;
pub use self::permissions::*;
pub use self::skill::*;
pub use self::stat::*;
pub use self::statistics::*;
pub use self::tier::*;
pub use self::unlock::*;
pub use self::user::*;
pub use self::user_group::*;
pub use self::user_management::*;
