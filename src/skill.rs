use crate::*;
// World interaction
// or
// Stat buff
#[derive(new, Clone, Serialize, Deserialize, Debug, Builder)]
pub struct SkillDefinition<K, S> {
    pub key: K,
    pub name: String,
    pub description: String,
    pub cooldown: f64,
    pub passive: bool,
    // stat usage
    pub conditions: Vec<StatCondition<S>>,
    pub stat_effectors: Vec<StatEffector<S>>,
}

pub struct SkillInstance<K> {
    pub skill_key: K,
    pub current_cooldown: f64,
}
