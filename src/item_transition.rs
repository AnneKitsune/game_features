use crate::*;

use std::collections::HashMap;
use std::hash::Hash;

// crafting
/// A transition from one or more items into one or more different items.
/// Can be used for all sorts of crafting.
#[derive(new, Clone, Serialize, Deserialize, Debug, Builder)]
pub struct ItemTransitionDefinition<K, I, E, S, U: Default> {
    /// The id of this item transition.
    pub key: K,
    /// The name of the transition.
    pub name: String,
    /// The friendly name of the transition.
    pub friendly_name: String,
    /// The icon path.
    pub icon_path: Option<String>,
    /// The different input items, quantities and `UseMode` for each one.
    pub input_items: Vec<(I, usize, UseMode)>,
    /// The required stats conditions required to process the transition.
    pub stat_conditions: Vec<StatCondition<S>>,
    /// The effectors applied during crafting.
    pub stat_effectors: Vec<E>,
    /// The different output items.
    pub output_items: Vec<ItemInstance<I, U>>,
    /// What happens when you lose the condition required to continue the transition.
    pub on_condition_lost: ConditionLostReaction,
    /// The time to complete the transition.
    pub time_to_complete: f64,
    /// Consume the input items at the start of the transition, regardless of the result.
    pub consume_input_immediate: bool,
    /// Automatically transition when all the required conditions are met.
    pub auto_trigger: bool,
}

/// The way items are used in a transition.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum UseMode {
    /// Consumes the item. The item will be lost.
    Consume,
    /// Uses a set amount of durability from the item.
    UseOnce {
        /// The amount of durability used.
        durability: f64,
    },
    /// Uses a set amount of durability from the item each second.
    UsePerSecond {
        /// The durability usage per second.
        rate: f64,
    },
}

/// What happens when the transition is stopped or conditions aren't met anymore for it to
/// continue.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum ConditionLostReaction {
    /// Nothing happens, the transition continues.
    None,
    /// The transition pauses and keeps its progress.
    Pause,
    /// The transition is cancelled and all progress is lost.
    /// If consume_input_immediate was true, the input items are not returned.
    Cancel,
}

/// A transition in progress.
#[derive(new, Clone, Serialize, Deserialize, Debug)]
pub struct ItemTransitionBatch<K> {
    /// The transition id.
    transition: K,
    /// The number of transitions that are queued.
    remaining: u32,
    /// The time until the current transition is completed.
    next_completion_remaining: f64,
}

/// The definitions of all known stats.
#[derive(Debug, Clone, Serialize, Deserialize, new)]
pub struct ItemTransitionDefinitions<K: Hash + Eq, I, E, S, U: Default> {
    /// The definitions.
    pub defs: HashMap<K, ItemTransitionDefinition<K, I, E, S, U>>,
}

impl<K: Hash + Eq, I, E, S, U: Default> Default for ItemTransitionDefinitions<K, I, E, S, U> {
    fn default() -> Self {
        Self {
            defs: HashMap::default(),
        }
    }
}

impl<K: Hash + Eq + Clone, I, E, S, U: Default> From<Vec<ItemTransitionDefinition<K, I, E, S, U>>>
    for ItemTransitionDefinitions<K, I, E, S, U>
{
    fn from(t: Vec<ItemTransitionDefinition<K, I, E, S, U>>) -> Self {
        let defs = t
            .into_iter()
            .map(|s| (s.key.clone(), s))
            .collect::<HashMap<_, _>>();
        Self::new(defs)
    }
}
