use crate::*;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

// World interaction
// or
// Stat buff
/// The definition of an usable skill.
#[derive(new, Clone, Serialize, Deserialize, Debug, Builder)]
pub struct SkillDefinition<K, E, S, I> {
    /// The id of this skill.
    pub key: S,
    /// The name.
    pub name: String,
    /// The computer friendly name.
    pub friendly_name: String,
    /// The complete description of this skill.
    pub description: String,
    /// The cooldown between usages of this skill.
    pub cooldown: f64,
    /// Whether this skill is will activate automatically once all conditions are met or
    /// if it needs to be manually activated.
    pub passive: bool,
    /// The stats conditions required to activate this skill.
    pub conditions: Vec<StatCondition<K>>,
    /// The item conditions required to activate this skill.
    pub item_conditions: Vec<(I, usize, UseMode)>,
    /// The caused stat effectors.
    pub stat_effectors: Vec<E>,
}

/// # Generics
/// K: Stat Key
/// E: Effector Key
/// S: Skill Key
/// I: Item Key
/// IT: Item Type
/// CD: Item Custom Data
impl<K: Hash + Eq + Debug, E, S, I: Clone + PartialEq + Debug> SkillDefinition<K, E, S, I> {
    /// Checks if all the conditions to use this skill are met.
    pub fn check_conditions<IT: SlotType, CD: Default + Clone + Debug>(&self, stats: &StatSet<K>, inventory: &Inventory<I, IT, CD>, stat_defs: &StatDefinitions<K>) -> bool {
        for c in &self.conditions {
            if !c.check(stats, stat_defs) {
                return false;
            }
        }
        for ic in &self.item_conditions {
            if !inventory.has_quantity(&ic.0, ic.1) {
                return false;
            }
        }
        true
    }
}

/// An instance of a skill.
/// There is one per skill per entity that can use it.
/// Holds the cooldown for each skill.
#[derive(Debug, Clone, Serialize, Deserialize, new)]
pub struct SkillInstance<S> {
    /// The skill key.
    pub skill_key: S,
    /// The remaining time to wait before we can use this skill again.
    pub current_cooldown: f64,
}

/// The set of skill that can be used by an entity.
#[derive(Debug, Clone, Serialize, Deserialize, new)]
pub struct SkillSet<S: Hash + Eq> {
    /// The skills mapped by their key.
    pub skills: HashMap<S, SkillInstance<S>>,
}

impl<S: Hash + Eq + Clone> From<Vec<S>> for SkillSet<S> {
    fn from(t: Vec<S>) -> Self {
        let mut h = HashMap::new();
        for s in t {
            h.insert(s.clone(), SkillInstance::new(s, 0.0));
        }
        Self {
            skills: h,
        }
    }
}

/// Holds the definitions of all known skills.
#[derive(Debug, Clone, Serialize, Deserialize, new)]
pub struct SkillDefinitions<K, E, S: Hash + Eq, I> {
    /// The definitions.
    pub defs: HashMap<S, SkillDefinition<K, E, S, I>>,
}

impl<K, E, S: Hash+Eq, I> Default for SkillDefinitions<K, E, S, I> {
    fn default() -> Self {
        Self {
            defs: HashMap::default(),
        }
    }
}

impl<K, E, S: Hash + Eq + Clone, I> From<Vec<SkillDefinition<K, E, S, I>>>
    for SkillDefinitions<K, E, S, I>
{
    fn from(t: Vec<SkillDefinition<K, E, S, I>>) -> Self {
        let defs = t
            .into_iter()
            .map(|s| (s.key.clone(), s))
            .collect::<HashMap<_, _>>();
        Self::new(defs)
    }
}
