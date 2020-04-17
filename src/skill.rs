use crate::*;
// World interaction
// or
// Stat buff
#[derive(new, Clone, Serialize, Deserialize, Debug, Builder)]
pub struct SkillDefinition<K, E, S, I> {
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
}

pub struct SkillInstance<K> {
    pub skill_key: K,
    pub current_cooldown: f64,
}
