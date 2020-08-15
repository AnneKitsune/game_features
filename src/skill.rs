use crate::*;
use std::collections::HashMap;
use std::hash::Hash;

// World interaction
// or
// Stat buff
#[derive(new, Clone, Serialize, Deserialize, Debug, Builder)]
pub struct SkillDefinition<K, E, S, I, GE> {
    pub key: K,
    pub name: String,
    pub friendly_name: String,
    pub description: String,
    pub cooldown: f64,
    pub passive: bool,
    // stat usage
    pub conditions: Vec<StatCondition<S>>,
    pub item_conditions: Vec<(I, usize, UseMode)>,
    pub stat_effectors: Vec<E>,
    pub event_on_trigger: Vec<GE>,
}

pub struct SkillInstance<K> {
    pub skill_key: K,
    pub current_cooldown: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, new)]
pub struct SkillDefinitions<K: Hash + Eq, E, S, I, GE> {
    pub defs: HashMap<K, SkillDefinition<K, E, S, I, GE>>,
}

impl<K: Hash + Eq + Clone, E, S, I, GE> From<Vec<SkillDefinition<K, E, S, I, GE>>>
    for SkillDefinitions<K, E, S, I, GE>
{
    fn from(t: Vec<SkillDefinition<K, E, S, I, GE>>) -> Self {
        let defs = t
            .into_iter()
            .map(|s| (s.key.clone(), s))
            .collect::<HashMap<_, _>>();
        Self::new(defs)
    }
}
