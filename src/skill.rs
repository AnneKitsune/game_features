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

impl<K: Hash + Eq + Debug, E, S, I> SkillDefinition<K, E, S, I> {
    // TODO: implement inventory conditions
    pub fn check_conditions(&self, stats: &StatSet<K>, stat_defs: &StatDefinitions<K>) -> bool {
        for c in &self.conditions {
            if !c.check(stats, stat_defs) {
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
