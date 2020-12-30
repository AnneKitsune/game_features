use crate::*;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

// World interaction
// or
// Stat buff
#[derive(new, Clone, Serialize, Deserialize, Debug, Builder)]
pub struct SkillDefinition<K, E, S, I> {
    pub key: S,
    pub name: String,
    pub friendly_name: String,
    pub description: String,
    pub cooldown: f64,
    pub passive: bool,
    // stat usage
    pub conditions: Vec<StatCondition<K>>,
    pub item_conditions: Vec<(I, usize, UseMode)>,
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
    // TODO: implement inventory conditions
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

#[derive(Debug, Clone, Serialize, Deserialize, new)]
pub struct SkillInstance<S> {
    pub skill_key: S,
    pub current_cooldown: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, new)]
pub struct SkillSet<S: Hash + Eq> {
    pub skills: HashMap<S, SkillInstance<S>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, new)]
pub struct SkillDefinitions<K, E, S: Hash + Eq, I> {
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
