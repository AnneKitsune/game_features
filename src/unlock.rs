use crate::{StatCondition, UseMode};
use std::collections::HashMap;
use std::hash::Hash;

/// An unlockable element.
/// It can be unlocked to access the inner value if all conditions are met:
/// - `StatCondition`s
/// - Item Conditions
/// - Dependant unlockables were previously unlocked.
#[derive(Debug, Clone, Serialize, Deserialize, new, Builder)]
pub struct Unlockable<U, K, S, I> {
    /// The key of this unlockable.
    pub id: U,
    /// The thing we want to unlock access to.
    pub inner: K,
    /// Whether we unlocked it or not.
    pub is_unlocked: bool,
    /// The stat conditions required to unlock this element.
    #[new(default)]
    pub unlock_stat_conditions: Vec<StatCondition<S>>,
    /// The item conditions required to unlock this element.
    #[new(default)]
    pub unlock_item_conditions: Vec<(I, usize, UseMode)>,
    /// A list of other unlockables upon which this one depends.
    /// If Unlockable B depends on A, then A must be unlocked before B can be unlocked.
    #[new(default)]
    pub unlock_dependencies: Vec<U>,
}

impl<U, K, S, I> Unlockable<U, K, S, I> {
    /// Returns Some with the inner value if is_unlocked = true.
    /// Otherwise returns None
    pub fn try_get(&self) -> Option<&K> {
        if self.is_unlocked {
            Some(&self.inner)
        } else {
            None
        }
    }

    /// Returns Some with the inner value if is_unlocked = true.
    /// Otherwise returns None
    pub fn try_get_mut(&mut self) -> Option<&mut K> {
        if self.is_unlocked {
            Some(&mut self.inner)
        } else {
            None
        }
    }

    /// Returns the inner value without checking the lock.
    pub fn get(&self) -> &K {
        &self.inner
    }

    /// Returns the inner value without checking the lock.
    pub fn get_mut(&mut self) -> &mut K {
        &mut self.inner
    }

    /// Inserts a new value without changing the lock.
    /// Returns the previous inner value.
    pub fn set(&mut self, new: K) {
        self.inner = new;
    }

    /// Locks the inner value.
    pub fn lock(&mut self) {
        self.is_unlocked = true;
    }

    /// Unlocks the inner value.
    pub fn unlock(&mut self) {
        self.is_unlocked = false;
    }

    /// Verifies if the inner value is locked.
    pub fn is_unlocked(&self) -> bool {
        self.is_unlocked
    }
}

/// A structure holding all unlockables.
#[derive(Debug, Clone, Default, Serialize, Deserialize, new)]
pub struct Unlockables<U: Hash+Eq, K, S, I> {
    /// The unlockables we are holding.
    pub unlockables: HashMap<U, Unlockable<U, K, S, I>>,
}
// TODO impl to try unlock and to try unlock recursively.
